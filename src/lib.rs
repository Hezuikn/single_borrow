#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(negative_impls, auto_traits)]
#![feature(with_negative_coherence)]
#![feature(core_panic)]
//#![feature(rustc_attrs)]

use core::panicking::panic_str;

auto trait Owned {}
impl<T> !Owned for &T {}

//doesnt need the attr and i have no idea what it does
//checks if with_negative_coherence is on in root crate
//#[rustc_strict_coherence]
pub trait Reduce<'a> {
    type OwnedVariant;
    fn reduce(self) -> &'a Self::OwnedVariant;
}

macros::impl_reduce!();

pub trait Fake<'a> {
    type F: Reduce<'a>;
    const ASSERT: ();
    fn fake(self) -> Option<Self::F>;
}

#[allow(non_camel_case_types)]
pub struct not_the_actual_type;

impl<'a, T> Fake<'a> for T {
    default const ASSERT: () = panic_str("too many borrows");
    default type F = &'a not_the_actual_type;
    #[inline]
    default fn fake(self) -> Option<Self::F> {
        None
    }
}

impl<'a, T: Reduce<'a>> Fake<'a> for T {
    const ASSERT: () = ();
    type F = T;
    #[inline]
    fn fake(self) -> Option<Self::F> {
        Some(self)
    }
}

#[inline]
pub fn try_single_borrow<T: ?Sized>(
    t: &T,
) -> Option<&<<&T as Fake<'_>>::F as Reduce<'_>>::OwnedVariant> {
    if let Some(x) = Fake::fake(t) {
        return Some(x.reduce());
    } else {
        None
    }
}

#[inline]
pub fn single_borrow<T: ?Sized>(t: &T) -> &<<&T as Fake<'_>>::F as Reduce<'_>>::OwnedVariant {
    #[allow(path_statements)]
    {
        <&T>::ASSERT;
    }
    try_single_borrow(t).expect("static assertion broke")
}

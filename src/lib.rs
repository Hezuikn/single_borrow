#![allow(incomplete_features)]
#![feature(with_negative_coherence, specialization)]
#![feature(negative_impls, auto_traits)]
#![feature(inline_const)]

auto trait Owned {} 
impl<T: ?Sized> !Owned for &T {}

pub trait SingleBorrow<'a> {
    type OwnedVariant: ?Sized;
    fn single_borrow(&'a self) -> &'a Self::OwnedVariant;
}

impl<'a, T: Owned + ?Sized> SingleBorrow<'a> for T {
    type OwnedVariant = T;
    #[inline]
    fn single_borrow(&'a self) -> &'a Self::OwnedVariant {
        self
    }
}

impl<'a, T: ?Sized> SingleBorrow<'a> for &'a T where T: SingleBorrow<'a> {
    type OwnedVariant = <T as SingleBorrow<'a>>::OwnedVariant;
    #[inline]
    fn single_borrow(&self) -> &'a Self::OwnedVariant {
        SingleBorrow::single_borrow(*self)
    }
}

pub unsafe trait Fake<'a> {
    type Fake: ?Sized + SingleBorrow<'a>;
    fn fake(&self) -> &Self::Fake;
}

#[allow(non_camel_case_types)]
pub enum not_the_actual_type {}

unsafe impl<'a, T: ?Sized> Fake<'a> for T {
    default type Fake = not_the_actual_type;
    default fn fake(&self) -> &Self::Fake {
        const {
            panic!("static assertion failed: this ought to be exhaustive")
        }
    }
}

unsafe impl<'a, T: ?Sized + SingleBorrow<'a>> Fake<'a> for T {
    type Fake = T;
    #[inline]
    fn fake(&self) -> &Self::Fake {
        self
    }
}

#[inline]
pub fn single_borrow<T: ?Sized>(t: &T) -> &<<T as Fake<'_>>::Fake as SingleBorrow<'_>>::OwnedVariant {
    t.fake().single_borrow()
}

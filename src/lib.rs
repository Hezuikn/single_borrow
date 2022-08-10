#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(inline_const, const_type_name)]

//downstream
pub trait SingleBorrow<'a> {
    type Less;
    fn single_borrow(self) -> &'a Self::Less;
}

impl<'a, T> SingleBorrow<'a> for &'a T {
    default type Less = T;
    #[inline]
    default fn single_borrow(self) -> &'a Self::Less {
        unsafe { transmate(self) }
    }
}

impl<'a, T> SingleBorrow<'a> for &'a &T {
    type Less = <&'a T as SingleBorrow<'a>>::Less;
    #[inline]
    fn single_borrow(self) -> &'a Self::Less {
        SingleBorrow::single_borrow(*self)
    }
}

//

impl<'a, T> SingleBorrow<'a> for &'a mut T {
    default type Less = T;
    #[inline]
    default fn single_borrow(self) -> &'a Self::Less {
        unsafe { transmate(&*self) }
    }
}

impl<'a, T> SingleBorrow<'a> for &'a mut &mut T {
    type Less = <&'a T as SingleBorrow<'a>>::Less;
    #[inline]
    fn single_borrow(self) -> &'a Self::Less {
        SingleBorrow::single_borrow(&**self)
    }
}

//

impl<'a, T> SingleBorrow<'a> for &'a mut &T {
    type Less = <&'a T as SingleBorrow<'a>>::Less;
    #[inline]
    fn single_borrow(self) -> &'a Self::Less {
        SingleBorrow::single_borrow(&**self)
    }
}

impl<'a, T> SingleBorrow<'a> for &'a &mut T {
    type Less = <&'a T as SingleBorrow<'a>>::Less;
    #[inline]
    fn single_borrow(self) -> &'a Self::Less {
        SingleBorrow::single_borrow(&**self)
    }
}

#[inline]
const unsafe fn transmate<T, U>(t: &T) -> &U {
    const {
        [()][!konst::string::eq_str(std::any::type_name::<T>(), std::any::type_name::<U>()) as usize];
    };
    return std::mem::transmute(t);
}

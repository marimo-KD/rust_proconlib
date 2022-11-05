use std::{fmt, ops};

pub trait Element: Sized + Clone + PartialEq + fmt::Debug {}
impl<T: Sized + Clone + PartialEq + fmt::Debug> Element for T {}

pub trait Zero: ops::Add<Output = Self> + ops::AddAssign + Element {
    fn zero() -> Self;
}

pub trait One: ops::Mul<Output = Self> + ops::MulAssign + Element {
    fn one() -> Self;
}

pub trait Ring: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign {}
impl<T: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign> Ring for T {}

pub trait Field: Ring + ops::Div<Output = Self> + ops::DivAssign {}
impl<T: Ring + ops::Div<Output = Self> + ops::DivAssign> Field for T {}

pub trait Magma: Element {
    fn op<L: Into<Self>, R: Into<Self>>(lhs: L, rhs: R) -> Self;
    fn op_from_left(&mut self, left: Self) {
        *self = Self::op(left, self.clone());
    }
    fn op_from_right(&mut self, right: Self) {
        *self = Self::op(self.clone(), right);
    }
}

pub trait Semigroup: Magma {}

pub trait Identity {
    fn identity() -> Self;
}

pub trait Quasigroup: Magma {
    fn inv(self) -> Self;
}

pub trait Loop: Quasigroup + Identity {}
impl<T: Quasigroup + Identity> Loop for T {}

pub trait Monoid: Semigroup + Identity {}
impl<T: Semigroup + Identity> Monoid for T {}

pub trait Commut: Semigroup {}

pub trait Group: Monoid + Loop {}
impl<T: Monoid + Loop> Group for T {}

pub trait Abel: Group {}


#[macro_export]
macro_rules! def_monoid {
    (
        derive($($attr:meta),*),
        $pub:vis struct $name:ident {
            $(
                $field_vis:vis $field_name:ident : $field_type:ty
            ),*$(,)*
        },
        $identity:expr, $op:item
    ) => {
        #[derive(Debug, Clone, PartialEq, $($attr),*)]
        $pub struct $name {
            $(
                $field_vis $field_name : $field_type
            ),*
        }
        $crate::impl_monoid!{$name, $identity, $op}
    };
    (
        derive($($attr:meta),*),
        $pub:vis struct $name:ident (
            $field_vis: vis $field_type: ty
        ),
        $identity:expr, $op:item
    ) => {
        #[derive(Debug, Clone, PartialEq, $($attr),*)]
        $pub struct $name (
            $field_vis $field_type
        );
        $crate::impl_monoid!{$name, $identity, $op}
        impl From<$field_type> for $name {
            fn from(t: $field_type) -> Self {
                Self(t)
            }
        }
    };
    (
        derive($($attr:meta),*),
        $pub:vis struct $name:ident (
            $(
                $field_vis: vis $field_type: ty
            ),*
        ),
        $identity:expr, $op:item
    ) => {
        #[derive(Debug, Clone, PartialEq, $($attr),*)]
        $pub struct $name (
            $(
                $field_vis $field_type
            ),*
        );
        $crate::impl_monoid!{$name, $identity, $op}
    };
}
#[macro_export]
macro_rules! impl_monoid {
    ($name: ident, $identity: expr, $op:item) => {
        impl Magma for $name {
            fn op<L: Into<Self>, R:Into<Self>>(lhs: L, rhs: R) -> Self {
                let lhs = lhs.into();
                let rhs = rhs.into();
                $op
                op(lhs, rhs)
            }
        }
        impl Semigroup for $name {}
        impl Identity for $name {
            fn identity() -> Self {
                $identity
            }
        }
    };
}

macro_rules! impl_one_integer {
    ($t: ty) => {
        impl One for $t {
            fn one() -> Self {
                1 as $t
            }
        }
    };
    ($($t:ty),+) => {
        $(impl_one_integer!($t);)+
    };
}
macro_rules! impl_zero_integer {
    ($t: ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0 as $t
            }
        }
    };
    ($($t:ty),+) => {
        $(impl_zero_integer!($t);)+
    };
}
macro_rules! impl_abel_integer {
    ($t: ty) => {
        impl Magma for $t {
            fn op<L: Into<Self>, R: Into<Self>>(lhs: L, rhs: R) ->Self {
                lhs.into() + rhs.into()
            }
        }
        impl Semigroup for $t {}
        impl Identity for $t {
            fn identity() -> Self {
                0 as $t
            }
        }
        impl Quasigroup for $t {
            fn inv(self) -> Self {
                -self
            }
        }
        impl Abel for $t {}
    };
    ($($t:ty),+) => {
        $(impl_abel_integer!($t);)+
    };
}
impl_one_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
impl_zero_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
impl_abel_integer!(i8, i16, i32, i64, i128);

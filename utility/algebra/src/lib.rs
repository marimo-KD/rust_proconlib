/// $B$3$N%b%8%e!<%k$O$J$,$?$+$J$5$s$N%i%$%V%i%j$KBg$-$J1F6A$r<u$1$F$$$^$9!#(B
/// github.com/ngtkana/ac-adapter-rs
use std::{fmt, ops};
/// $BBe?tE*$J9=B$$N4pK\(B
pub trait Element: Sized + Clone + PartialEq + fmt::Debug {}
impl<T: Sized + Clone + PartialEq + fmt::Debug> Element for T {}

/// $B2CK!$,Dj5A$5$l$F$$$^$9!#(B
pub trait Zero: ops::Add<Output = Self> + ops::AddAssign + Element {
    // $BC10L85$G$9!#(B
    fn zero() -> Self;
}

/// $B>hK!$G$9!#(B
pub trait One: ops::Mul<Output = Self> + ops::MulAssign + Element {
    fn one() -> Self;
}

/// $B2CK!$H>hK!$,$"$l$P4D$,M_$7$/$J$j$^$9!#(B
/// $B2CK!$K4X$7$F5U85$,I,MW$J$N$G!"(BNeg,Sub,SubAssign$B$bMW5a$7$^$9!#(B
pub trait Ring: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign {}
impl<T: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign> Ring for T {}

/// $BFs9`1i;;$,$"$j$^$9!#(B
pub trait Magma: Element {
    fn op(lhs: Self, rhs: Self) -> Self;
    fn op_from_left(&mut self, left: Self) {
        *self = Self::op(left, self.clone());
    }
    fn op_from_right(&mut self, right: Self) {
        *self = Self::op(self.clone(), right);
    }
}
/// $BH>72$G$9!#(B($B7k9g@-$G$9$M!#(B)
pub trait Semigroup: Magma {}

/// $BC10L85$G$9!#(B
pub trait Identity {
    fn identity() -> Self;
}

/// $B5<72$G$9!#(B
/// $B5U85$,$"$k$3$H$G$9!#(B
pub trait Quasigroup: Magma {
    fn inv(self) -> Self;
}

/// Loop$B$G$9!#(B
/// $B5<72$KC10L85$,$D$-$^$7$?!#(B
pub trait Loop: Quasigroup + Identity {}
impl<T: Quasigroup + Identity> Loop for T {}

/// $BC10L85$N$"$kFs9`1i;;$r;}$D9=B$$G$9!#(B
/// $B$D$^$j$O%b%N%$%I$N$3$H$G$9!#(B
pub trait Monoid: Semigroup + Identity {}
impl<T: Semigroup + Identity> Monoid for T {}

/// $B7k9g!"2D49$NFs9`1i;;$,$"$j$^$9!#(B
pub trait Commut: Semigroup {}

/// $B$O$$!"72$G$9!#(B
/// $B$D$^$j$O!"7k9g!"C10L85!"5U85$,$"$j$^$9!#(B
pub trait Group: Monoid + Loop {}
impl<T: Monoid + Loop> Group for T {}

/// $B2D49$J72$G$9!#(B
pub trait Abel: Group {}

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
                fn op(lhs: Self, rhs: Self) ->Self {
                    lhs + rhs
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

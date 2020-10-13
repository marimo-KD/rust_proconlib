use cargo_snippet::snippet;

#[snippet("alge")]
pub mod alge {
    /// $B$3$N%b%8%e!<%k$O$J$,$?$+$J$5$s$N%i%$%V%i%j$KBg$-$J1F6A$r<u$1$F$$$^$9!#(B
    /// github.com/ngtkana/ac-adapter-rs
    ///
    use std::{fmt, ops};
    /// $BBe?tE*$J9=B$$N4pK\(B
    pub trait Element: Sized + Clone + PartialEq + fmt::Debug {}
    impl<T: Sized + Clone + PartialEq + fmt::Debug> Element for T {}

    /// $B2CK!$,Dj5A$5$l$F$$$^$9!#(B
    /// $B$5$9$,$K2CK!$KC10L85$,L5$$$H7y$J5$;}$A$K$J$j$^$9$h$M!)(B
    /// $B$J$N$GC10L85$b$D$$$F$^$9!#(B
    pub trait Zero: ops::Add<Output = Self> + ops::AddAssign + Element {
        /// $BC10L85$G$9!#(B
        fn zero() -> Self;
    }

    /// $B$5$C$-$H$@$$$?$$F1$8$G$9!#(B
    /// $B>hK!$G$9!#(B
    pub trait One: ops::Mul<Output = Self> + ops::MulAssign + Element {
        fn one() -> Self;
    }

    /// $B2CK!$H>hK!$,$"$l$P4D$,M_$7$/$J$j$^$9!#(B
    /// $B2CK!$K4X$7$F5U85$,I,MW$J$N$G!"(BNeg,Sub,SubAssign$B$bMW5a$7$^$9!#(B
    pub trait Ring: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign {}
    impl<T: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign> Ring for T {}

    /// $BFs9`1i;;$r;}$D9=B$$N4pK\$G$9!#(B
    /// $B7k9gK!B'$0$i$$$O;}$C$F$$$F$[$7$$$G$9$M!#(B
    /// $B$D$^$j$OH>72$N$3$H$G$9!#(B
    pub trait Semigroup: Element {
        fn op(self, rhs: Self) -> Self;
        fn op_from_left(&mut self, left: &Self) {
            *self = Self::op(left.clone(), self.clone());
        }
        fn op_from_right(&mut self, right: &Self) {
            *self = Self::op(self.clone(), right.clone());
        }
    }

    /// $BC10L85$N$"$kFs9`1i;;$r;}$D9=B$$G$9!#(B
    /// $B$D$^$j$O%b%N%$%I$N$3$H$G$9!#(B
    pub trait Monoid: Semigroup {
        fn identity() -> Self;
    }

    /// $B$J$s$H2D49$NFs9`1i;;$r;}$D9=B$$G$9!#(B
    /// Wikipedia$B$5$s$r8+$F$bNI$$46$8$N8@MU$O$J$5$=$&$G$9!#(B
    pub trait Commut: Semigroup {}

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
    impl_one_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
    impl_zero_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
}

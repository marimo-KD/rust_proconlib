use cargo_snippet::snippet;

#[snippet]
pub mod alge {
    // このモジュールはながたかなさんのライブラリに大きな影響を受けています。
    // github.com/ngtkana/ac-adapter-rs
    //
    use std::{fmt, ops};
    // 代数的な構造の基本
    pub trait Element: Sized + Clone + PartialEq + fmt::Debug {}
    impl<T: Sized + Clone + PartialEq + fmt::Debug> Element for T {}

    // 加法が定義されています。
    // さすがに加法に単位元が無いと嫌な気持ちになりますよね？
    // なので単位元もついてます。
    pub trait Zero: ops::Add<Output = Self> + ops::AddAssign + Element {
        // 単位元です。
        fn zero() -> Self;
    }

    // さっきとだいたい同じです。
    // 乗法です。
    pub trait One: ops::Mul<Output = Self> + ops::MulAssign + Element {
        fn one() -> Self;
    }

    // 加法と乗法があれば環が欲しくなります。
    // 加法に関して逆元が必要なので、Neg,Sub,SubAssignも要求します。
    pub trait Ring: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign {}
    impl<T: Zero + One + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign> Ring for T {}

    // 二項演算を持つ構造の基本です。
    // 結合法則ぐらいは持っていてほしいですね。
    // つまりは半群のことです。
    pub trait Semigroup: Element {
        fn op(self, rhs: Self) -> Self;
        fn op_from_left(&mut self, left: &Self) {
            *self = Self::op(left.clone(), self.clone());
        }
        fn op_from_right(&mut self, right: &Self) {
            *self = Self::op(self.clone(), right.clone());
        }
    }

    // 単位元のある二項演算を持つ構造です。
    // つまりはモノイドのことです。
    pub trait Monoid: Semigroup {
        fn identity() -> Self;
    }

    // 結合、可換の二項演算があります。
    pub trait Commut: Semigroup {}

    // はい、群です。
    // つまりは、結合、単位元、逆元があります。
    pub trait Group: Monoid {
        fn inv(self) -> Self;
    }

    // 可換な群です。
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
            impl Semigroup for $t {
                fn op(self, rhs: Self) ->Self {
                    self + rhs
                }
            }
            impl Monoid for $t {
                fn identity() -> Self {
                    0 as $t
                }
            }
            impl Group for $t {
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
}

use cargo_snippet::snippet;

#[snippet("static_modint_montgomery")]
mod modint {
    use std::ops::*;
    pub trait Mod: Copy {
        const M: u64;
        fn S() -> u64;
        fn X() -> u64;
    }
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct Modint<M> {
        x: u64,
        phantom: std::marker::PhantomData<M>,
    }
    impl<M: Mod> Modint<M> {
        pub fn new(x: u64) -> Self {
            Modint::new_internal(Self::modulo(x))
        }
        fn new_internal(x: u64) -> Self {
            Self {
                x,
                phantom: std::marker::PhantomData,
            }
        }
        fn div(x: u64) -> u64 {
            ((x as u128 * M::X() as u128 >> M::S()) >> 64) as u64
        }
        fn modulo(x: u64) -> u64 {
            x - Self::div(x) * M::M
        }
        pub fn pow(self, mut e: u64) -> Self {
            let mut res = Modint::new_internal(1);
            let mut acc = self;
            while e > 0 {
                if e % 2 != 0 {
                    res *= acc;
                }
                acc *= acc;
                e /= 2;
            }
            res
        }
        pub fn inv(self) -> Self {
            self.pow(M::M - 2)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Add<T> for Modint<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let mut sum = self.x + other.into().x;
            if sum >= M::M {
                sum -= M::M;
            }
            Modint::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Sub<T> for Modint<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let mut diff = self.x as i64 - other.into().x as i64;
            if diff < 0 {
                diff += M::M as i64;
            }
            Modint::new_internal(diff as u64)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Mul<T> for Modint<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self {
            Self::new_internal(Self::modulo(self.x*other.into().x))
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> AddAssign<T> for Modint<M> {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> SubAssign<T> for Modint<M> {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> MulAssign<T> for Modint<M> {
        fn mul_assign(&mut self, other: T) {
            *self = *self * other;
        }
    }
    impl<M> std::fmt::Display for Modint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> From<i64> for Modint<M> {
        fn from(x: i64) -> Self {
            Self::new((x % M::M) + M::M)
        }
    }
    impl<M: Mod> From<i32> for Modint<M> {
        fn from(x: i32) -> Self {
            Self::new(x as i64)
        }
    }
    impl<M: Mod> From<usize> for Modint<M> {
        fn from(x: usize) -> Self {
            Self::new(x as i64)
        }
    }
}

#[snippet("static_modint")]
macro_rules! define_mod {
    ($struct_name:ident, $modulo:expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        struct $struct_name {}
        impl modint::Mod for $struct_name {
            const M: u64 = $modulo;
            fn S() -> u64 {
                if (Self::M - 1).is_power_of_two() {
                    (Self::M - 1).trailing_zeros() as u64
                } else {
                    (Self::M - 1).trailing_zeros() as u64 - 1
                }
            }
            fn X() -> u64 {
                ((((1 as u128) << (Self::S() + 64)) + Self::M as u128 - 1) / Self::M as u128) as u64
            }
        }
    };
}

define_mod!(P, 1_000_000_007);
type ModInt = modint::Modint<P>;

#[test]
fn modint_test() {
    let a = ModInt::new(5);
    let b = ModInt::new(10);
    assert_eq!(a + b, ModInt::new(15));
    assert_eq!(a * b, ModInt::new(50));
}

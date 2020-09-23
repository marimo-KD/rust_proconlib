use cargo_snippet::snippet;

#[snippet("static_modint")]
mod modint {
    use std::ops::*;
    pub trait Mod: Copy {
        const M: u64;
        const S: u64;
        const X: u64;
        fn div(x: u64) -> u64;
        fn modulo(x: u64) -> u64;
    }
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct Modint<M> {
        x: u64,
        phantom: std::marker::PhantomData<M>,
    }
    impl<M: Mod> Modint<M> {
        pub fn new(x: u64) -> Self {
            Modint::new_internal(M::modulo(x))
        }
        fn new_internal(x: u64) -> Self {
            Self {
                x,
                phantom: std::marker::PhantomData,
            }
        }
        pub fn value(self) -> u64 {
            self.x
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
            Self::new(self.x.wrapping_mul(other.into().x))
            // Self::new_internal((self.x * other.into().x) % M::M)
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
            Self::new((x % M::M as i64) as u64 + M::M)
        }
    }
    impl<M: Mod> From<i32> for Modint<M> {
        fn from(x: i32) -> Self {
            Self::from(x as i64)
        }
    }
    impl<M: Mod> From<usize> for Modint<M> {
        fn from(x: usize) -> Self {
            Self::new(x as u64)
        }
    }
}

#[snippet("static_modint")]
const fn _next_power_of_two(mut x: u64) -> u64 {
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x |= x >> 32;
    x += 1;
    x
}

#[snippet("static_modint")]
macro_rules! define_mod {
    ($struct_name:ident, $modulo:expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        struct $struct_name {}
        impl modint::Mod for $struct_name {
            const M: u64 = $modulo;
            const S: u64 = {
                let log = Self::M.wrapping_sub(1);
                let log = _next_power_of_two(log).trailing_zeros() as u64;
                let s =
                    [log.wrapping_sub(1), log][Self::M.wrapping_sub(1).is_power_of_two() as usize];
                [s + 64, 0][(Self::M == 1) as usize]
            };
            const X: u64 = {
                let s = Self::S as u32;
                let m = Self::M as u128;
                (((1 as u128).wrapping_shl(s).wrapping_add(m).wrapping_sub(1)) / m) as u64
            };
            fn div(x: u64) -> u64 {
                (((x as u128) * Self::X as u128).wrapping_shr(Self::S as u32)) as u64
            }
            fn modulo(x: u64) -> u64 {
                x.wrapping_sub(Self::div(x) * Self::M)
            }
            // $B5U?t>h;;(B
            // Barrett reduction$B$J$k$b$N$K6a$$$s$@$H$+(B
        }
    };
}

define_mod!(P, 1_000_000_007);
define_mod!(A, 1_000_000);
type ModInt = modint::Modint<P>;

#[test]
fn fast_mod_test() {
    use crate::static_modint::modint::Mod;
    let a = 1_000_000_007;
    assert_eq!(A::modulo(a), 7);
}

#[test]
fn modint_test() {
    let a = ModInt::new(5);
    let b = ModInt::new(10);
    let c = ModInt::new(1_000_000_006);
    assert_eq!((a + b).value(), 15);
    assert_eq!((a * b).value(), 50);
    assert_eq!((a * c).value(), (1_000_000_006 * 5) % 1_000_000_007);
}
#[test]
fn modint_test_fac() {
    use crate::static_modint::modint::Mod;
    let mut a: u64 = 1;
    let mut b = ModInt::new(1);
    for i in 1..=10_000_000 {
        a = a * i % 1_000_000_007;
        b *= ModInt::new(i);
        assert_eq!(a, b.value(), "a:{} b:{} i:{} s:{} x{}", a, b, i, P::S, P::X);
    }
}

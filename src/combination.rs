use cargo_snippet::snippet;
#[snippet(name = "combination")]
mod modint {
    use std::ops::*;
    pub trait Mod: Copy {
        fn m() -> u64;
    }
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct Modint<M> {
        x: u64,
        phantom: std::marker::PhantomData<M>,
    }
    impl<M: Mod> Modint<M> {
        pub fn new(x: i64) -> Self {
            Modint::new_internal(((x % (M::m() as i64) + M::m() as i64) % M::m() as i64) as u64)
        }
        fn new_internal(x: u64) -> Self {
            Modint {
                x,
                phantom: std::marker::PhantomData,
            }
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
            self.pow(M::m() - 2)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Add<T> for Modint<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let mut sum = self.x + other.into().x;
            if sum >= M::m() {
                sum -= M::m();
            }
            Modint::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Sub<T> for Modint<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let mut diff = self.x as i64 - other.into().x as i64;
            if diff < 0 {
                diff += M::m() as i64;
            }
            Modint::new_internal(diff as u64)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Mul<T> for Modint<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self {
            Modint::new((self.x * other.into().x) as i64)
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
            Self::new(x)
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
macro_rules! define_mod {
    ($ struct_name : ident , $ modulo : expr ) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        struct $struct_name {}
        impl modint::Mod for $struct_name {
            #[inline]
            fn m() -> u64 {
                $modulo
            }
        }
    };
}
#[snippet(name = "combination")]
mod combination {
    use super::modint;
    pub struct CombinationCalculator<M: modint::Mod> {
        fac: Vec<modint::Modint<M>>,
        facinv: Vec<modint::Modint<M>>,
    }
    impl<M: modint::Mod> CombinationCalculator<M> {
        pub fn new(max: usize) -> Self {
            let max = max + 1;
            let mut fac = vec![modint::Modint::new(1); max];
            let mut facinv = vec![modint::Modint::new(1); max];
            for i in 2..max {
                fac[i] = fac[i - 1] * i;
            }
            facinv[max - 1] = fac[max - 1].inv();
            for i in (0..max - 1).rev() {
                facinv[i] = facinv[i + 1] * (i + 1);
            }
            Self { fac, facinv }
        }
        pub fn calc(&self, n: usize, k: usize) -> modint::Modint<M> {
            if n < k {
                return modint::Modint::new(0);
            }
            self.fac[n] * self.facinv[k] * self.facinv[n - k]
        }
    }
}

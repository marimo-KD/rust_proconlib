use algebra::{One, Zero};
pub mod from;
pub mod ops;
pub trait Mod: Copy + std::fmt::Debug + PartialEq {
    const M: u64;
    const S: u64;
    const X: u128;
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
    #[inline(always)]
    pub fn value(self) -> u64 {
        self.x
    }
    pub fn pow(self, mut e: u64) -> Self {
        let mut res = Modint::one();
        let mut acc = self;
        e = M::div(e) + M::modulo(e);
        while e > 0 {
            if e & 1 == 1 {
                res *= acc;
            }
            acc *= acc;
            e >>= 1;
        }
        res
    }
    #[inline]
    pub fn inv(self) -> Self {
        self.pow(M::M - 2)
    }
}
impl<M: Mod> std::fmt::Display for Modint<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.x.fmt(f)
    }
}
impl<M: Mod> Zero for Modint<M> {
    fn zero() -> Self {
        Self::new_internal(0)
    }
}
impl<M: Mod> One for Modint<M> {
    fn one() -> Self {
        Self::new_internal(1)
    }
}
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
pub const fn _calc_s(m: u64) -> u64 {
    let log = m.wrapping_sub(1);
    let log = _next_power_of_two(log).trailing_zeros() as u64;
    let s = [log.wrapping_sub(1), log][m.wrapping_sub(1).is_power_of_two() as usize];
    [s + 64, 0][(m == 1) as usize]
}

#[macro_export]
macro_rules! define_mod {
    ($struct_name:ident, $modulo:expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $struct_name {}
        impl Mod for $struct_name {
            const M: u64 = $modulo;
            const S: u64 = $crate::_calc_s(Self::M);
            const X: u128 = {
                let s = Self::S as u32;
                let m = Self::M as u128;
                ((1 as u128).wrapping_shl(s).wrapping_add(m).wrapping_sub(1)) / m
            };
            fn div(x: u64) -> u64 {
                (((x as u128) * Self::X).wrapping_shr(Self::S as u32)) as u64
            }
            fn modulo(x: u64) -> u64 {
                x.wrapping_sub(Self::div(x) * Self::M)
            }
            // 逆数乗算
        }
    };
}
define_mod!(P1000000007, 1_000_000_007);
define_mod!(P998244353, 998244353);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modint_test() {
        type ModInt = Modint<P1000000007>;
        let a = ModInt::new(5);
        let b = ModInt::new(10);
        let c = ModInt::new(1_000_000_006);
        assert_eq!((a + b).value(), 15);
        assert_eq!((a * b).value(), 50);
        assert_eq!((a * c).value(), (1_000_000_006 * 5) % 1_000_000_007);
    }
    #[test]
    fn modint_test_fac() {
        type ModInt = Modint<P1000000007>;
        let mut a: u64 = 1;
        let mut b = ModInt::new(1);
        for i in 1..=100000 {
            a = a * i % 1_000_000_007;
            b *= ModInt::new(i);
            assert_eq!(
                a,
                b.value(),
                "a:{} b:{} i:{} s:{} x{}",
                a,
                b,
                i,
                P1000000007::S,
                P1000000007::X
            );
        }
    }
}

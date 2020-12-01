use algebra::{One, Zero};
use std::ops::*;
pub trait Mod: Copy + std::fmt::Debug + PartialEq {
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
        let mut res = Modint::one();
        let mut acc = self;
        while e > 0 {
            if e & 1 == 1 {
                res *= acc;
            }
            acc *= acc;
            e <<= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        self.pow(M::M - 2)
    }
}
impl<M: Mod> Neg for Modint<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * Self::new_internal(M::M - 1)
    }
}
// {{{ operation
// {{{ binary operation
impl<M: Mod, T: Into<Modint<M>>> Add<T> for Modint<M> {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        let mut ret = self.clone();
        ret.add_assign(rhs);
        ret
    }
}
impl<M: Mod, T: Into<Modint<M>>> Sub<T> for Modint<M> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        let mut ret = self.clone();
        ret.sub_assign(rhs);
        ret
    }
}
impl<M: Mod, T: Into<Modint<M>>> Mul<T> for Modint<M> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut ret = self.clone();
        ret.mul_assign(rhs);
        ret
    }
}
// }}}
// {{{ compound
impl<M: Mod, T: Into<Modint<M>>> AddAssign<T> for Modint<M> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.into().x;
        if self.x >= M::M {
            self.x -= M::M;
        }
    }
}
impl<M: Mod, T: Into<Modint<M>>> SubAssign<T> for Modint<M> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.x < rhs.x {
            self.x += M::M;
        }
        self.x -= rhs.x;
    }
}
impl<M: Mod, T: Into<Modint<M>>> MulAssign<T> for Modint<M> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.into().x;
        self.x = M::modulo(self.x);
    }
}
// }}}
// }}}
impl<M: Mod> std::fmt::Display for Modint<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.x.fmt(f)
    }
}
// {{{ from
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
//}}}
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
const fn _calc_s(m: u64) -> u64 {
    let log = m.wrapping_sub(1);
    let log = _next_power_of_two(log).trailing_zeros() as u64;
    let s = [log.wrapping_sub(1), log][m.wrapping_sub(1).is_power_of_two() as usize];
    [s + 64, 0][(m == 1) as usize]
}
macro_rules! define_mod {
    ($struct_name:ident, $modulo:expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $struct_name {}
        impl Mod for $struct_name {
            const M: u64 = $modulo;
            const S: u64 = _calc_s(Self::M);
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
            // 逆数乗算
            // Barrett reductionなるものに近いんだとか
        }
    };
}
define_mod!(P1000000007, 1_000_000_007);
define_mod!(P998244353, 998244353);
define_mod!(P1224736769, 1224736769);
define_mod!(P469762049, 469762049);
define_mod!(P167772161, 167772161);

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

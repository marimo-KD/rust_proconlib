use std::io::*;
use std::*;

// {{{
pub mod scanner {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::io;
    use std::io::BufRead;
    use std::str::FromStr;
    struct Tokenizer<T: BufRead> {
        source: T,
        tokens: VecDeque<String>,
        buf: String,
    }
    impl<T: BufRead> Tokenizer<T> {
        fn new(source: T) -> Self {
            Self {
                source,
                tokens: VecDeque::new(),
                buf: String::new(),
            }
        }
    }
    impl<T: BufRead> Iterator for Tokenizer<T> {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            while self.tokens.is_empty() {
                self.buf.clear();
                self.source.read_line(&mut self.buf).unwrap();
                for i in self.buf.split_whitespace() {
                    self.tokens.push_back(String::from(i));
                }
            }
            Some(self.tokens.pop_front().unwrap())
        }
    }
    pub struct Scanner<T: BufRead> {
        tokenizer: Tokenizer<T>,
    }
    impl<T: BufRead> Scanner<T> {
        pub fn new(source: T) -> Self {
            Self {
                tokenizer: Tokenizer::new(source),
            }
        }
        pub fn scan<U: FromStr>(&mut self) -> U {
            self.tokenizer.next().unwrap().parse::<U>().ok().unwrap()
        }
    }
    pub fn build_scanner() -> Scanner<io::StdinLock<'static>> {
        let stdin = Box::leak(Box::new(io::stdin()));
        Scanner {
            tokenizer: Tokenizer::new(stdin.lock()),
        }
    }
}
macro_rules ! scan {($ scanner : ident ; [char ] ) => {$ scanner . scan ::< String > () . chars () . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [u8 ] ) => {$ scanner . scan ::< String > () . bytes . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [$ ($ t : tt ) ,+; $ n : expr ] ) => {(0 ..$ n ) . map (| _ | ($ (scan ! ($ scanner ;$ t ) ) ,* ) ) . collect ::< Vec < _ >> () } ; ($ scanner : ident ; $ t : ty ) => {$ scanner . scan ::<$ t > () } ; ($ scanner : ident ; $ ($ t : tt ) ,+ ) => {($ (scan ! ($ scanner ; $ t ) ) ,* ) } ; }

// }}}

//{{{
//
#[macro_use]
pub mod static_modint {
    use super::alge::{One, Zero};
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
    impl<M: Mod> Neg for Modint<M> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            self * Self::new(M::M - 1)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Add<T> for Modint<M> {
        type Output = Self;
        fn add(self, rhs: T) -> Self {
            let mut sum = self.x + rhs.into().x;
            if sum >= M::M {
                sum -= M::M;
            }
            Modint::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Sub<T> for Modint<M> {
        type Output = Self;
        fn sub(self, rhs: T) -> Self {
            let mut diff = self.x as i64 - rhs.into().x as i64;
            if diff < 0 {
                diff += M::M as i64;
            }
            Modint::new_internal(diff as u64)
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> Mul<T> for Modint<M> {
        type Output = Self;
        fn mul(self, rhs: T) -> Self {
            Self::new(self.x.wrapping_mul(rhs.into().x))
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> AddAssign<T> for Modint<M> {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs;
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> SubAssign<T> for Modint<M> {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs;
        }
    }
    impl<M: Mod, T: Into<Modint<M>>> MulAssign<T> for Modint<M> {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }
    impl<M: Mod> std::fmt::Display for Modint<M> {
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
    impl<M: Mod> Zero for Modint<M> {
        fn zero() -> Self {
            Self::new(0)
        }
    }
    impl<M: Mod> One for Modint<M> {
        fn one() -> Self {
            Self::new(1)
        }
    }
    #[macro_export]
    macro_rules! define_mod {
        ($ struct_name : ident , $ modulo : expr ) => {
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            pub struct $struct_name {}
            impl $struct_name {
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
            }
            impl Mod for $struct_name {
                const M: u64 = $modulo;
                const S: u64 = {
                    let log = Self::M.wrapping_sub(1);
                    let log = Self::_next_power_of_two(log).trailing_zeros() as u64;
                    let s = [log.wrapping_sub(1), log]
                        [Self::M.wrapping_sub(1).is_power_of_two() as usize];
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
            }
        };
    }
    define_mod!(P1000000007, 1_000_000_007);
}
pub mod alge {
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
    pub trait Semigroup: Element {
        fn op(self, rhs: Self) -> Self;
        fn op_from_left(&mut self, left: &Self) {
            *self = Self::op(left.clone(), self.clone());
        }
        fn op_from_right(&mut self, right: &Self) {
            *self = Self::op(self.clone(), right.clone());
        }
    }
    pub trait Monoid: Semigroup {
        fn identity() -> Self;
    }
    pub trait Commut: Semigroup {}
    pub trait Group: Monoid {
        fn inv(self) -> Self;
    }
    pub trait Abel: Group {}
    macro_rules ! impl_one_integer {($ t : ty ) => {impl One for $ t {fn one () -> Self {1 as $ t } } } ; ($ ($ t : ty ) ,+ ) => {$ (impl_one_integer ! ($ t ) ; ) + } ; }
    macro_rules ! impl_zero_integer {($ t : ty ) => {impl Zero for $ t {fn zero () -> Self {0 as $ t } } } ; ($ ($ t : ty ) ,+ ) => {$ (impl_zero_integer ! ($ t ) ; ) + } ; }
    macro_rules ! impl_abel_integer {($ t : ty ) => {impl Semigroup for $ t {fn op (self , rhs : Self ) -> Self {self + rhs } } impl Monoid for $ t {fn identity () -> Self {0 as $ t } } impl Group for $ t {fn inv (self ) -> Self {- self } } impl Abel for $ t {} } ; ($ ($ t : ty ) ,+ ) => {$ (impl_abel_integer ! ($ t ) ; ) + } ; }
    impl_one_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
    impl_zero_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
    impl_abel_integer!(i8, i16, i32, i64, i128);
}
#[macro_use]
mod ntt {
    use super::alge::{One, Zero};
    use super::static_modint::{Mod, Modint};
    pub trait NTTableMod: Mod {
        const PRIMITIVE_ROOT: u64;
    }
    fn _ntt<M: NTTableMod>(a: &mut [Modint<M>], g: Modint<M>) {
        let n = a.len();
        assert!(n.is_power_of_two());
        let mask = n - 1;
        let lgn = n.trailing_zeros();
        let mut a = a;
        let mut b = vec![Modint::zero(); n].into_boxed_slice();
        let mut b: &mut [Modint<M>] = &mut b;
        let root = g.pow((M::M - 1) / n as u64);
        let mut roots = vec![Modint::zero(); lgn as usize + 1];
        roots[0] = root;
        for i in 1..lgn as usize + 1 {
            roots[i] = roots[i - 1] * roots[i - 1];
        }
        for _i in (0..lgn).rev() {
            std::mem::swap(&mut a, &mut b);
            let i = 1 << _i;
            let d = roots[_i as usize];
            let mut w = Modint::one();
            for j in (0..n).step_by(i) {
                for k in 0..i {
                    a[j + k] = b[(j * 2 & mask) + k] + b[((j * 2 + i) & mask) + k] * w;
                }
                w = w * d;
            }
        }
        if lgn % 2 == 1 {
            b.copy_from_slice(a);
        }
    }
    pub fn ntt<M: NTTableMod>(a: &mut [Modint<M>]) {
        _ntt(a, Modint::new(M::PRIMITIVE_ROOT));
    }
    pub fn intt<M: NTTableMod>(a: &mut [Modint<M>]) {
        _ntt(a, Modint::new(M::PRIMITIVE_ROOT).inv());
        let n = a.len();
        let ninv = Modint::from(n).inv();
        a.iter_mut().for_each(|x| {
            *x *= ninv;
        });
    }
    pub fn convolution<M: NTTableMod>(
        mut x: Vec<Modint<M>>,
        mut y: Vec<Modint<M>>,
    ) -> Vec<Modint<M>> {
        let n = x.len() + y.len() - 1;
        let sz = n.next_power_of_two();
        x.resize(sz, Modint::zero());
        y.resize(sz, Modint::zero());
        ntt(&mut x);
        ntt(&mut y);
        for i in 0..sz {
            x[i] = x[i] * y[i];
        }
        intt(&mut x);
        x.truncate(n);
        x
    }
    #[macro_export]
    macro_rules! define_nttable_mod {
        ($ struct_name : ident , $ modulo : expr , $ root : expr ) => {
            define_mod!($struct_name, $modulo);
            impl NTTableMod for $struct_name {
                const PRIMITIVE_ROOT: u64 = $root;
            }
        };
    }
    define_nttable_mod!(P998244353, 998244353, 3);
    define_nttable_mod!(P1224736769, 1224736769, 3);
    define_nttable_mod!(P469762049, 469762049, 3);
    define_nttable_mod!(P167772161, 167772161, 3);
}
//}}}
//
type ModInt = static_modint::Modint<ntt::P1224736769>;
use alge::*;
fn main() {
    let mut stdin = scanner::build_scanner();
    let n = scan!(stdin; usize);
    let mut a = vec![ModInt::zero(); n];
    let mut b = vec![ModInt::zero(); n];
    for i in 0..n {
        let (ai, bi) = scan!(stdin; u64, u64);
        a[i] = ModInt::new(ai);
        b[i] = ModInt::new(bi);
    }
    let res = ntt::convolution(a, b);
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    writeln!(stdout, "0").unwrap();
    for i in res {
        writeln!(stdout, "{}", i).unwrap();
    }
}

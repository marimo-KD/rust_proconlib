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
macro_rules ! scan {($ scanner : ident ; [char ] ) => {$ scanner . scan ::< String > () . chars () . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [u8 ] ) => {$ scanner . scan ::< String > () . bytes . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [$ t : tt ; $ n : expr ] ) => {(0 ..$ n ) . map (| _ | scan ! ($ scanner ; $ t ) ) . collect ::< Vec < _ >> () } ; ($ scanner : ident ; $ t : ty ) => {$ scanner . scan ::<$ t > () } ; ($ scanner : ident ; $ ($ t : tt ) ,+ ) => {($ (scan ! ($ scanner ; $ t ) ) ,* ) } ; }
mod segment_tree {
    pub trait Monoid: Clone {
        fn id() -> Self;
        fn op(&self, x: &Self) -> Self;
    }
    pub struct SegmentTree<T: Monoid> {
        data: Vec<T>,
        len: usize,
    }
    impl<T: Monoid> SegmentTree<T> {
        fn get_proper_size(len: usize) -> usize {
            len.next_power_of_two()
        }
        pub fn new(len: usize) -> Self {
            let len = Self::get_proper_size(len);
            Self {
                data: vec![T::id(); 2 * len],
                len,
            }
        }
        pub fn build(&mut self, initializer: &Vec<T>) {
            let len = Self::get_proper_size(initializer.len());
            let mut data = vec![T::id(); 2 * len];
            for (idx, val) in initializer.iter().enumerate() {
                data[idx + len] = val.clone();
            }
            for i in (1..len).rev() {
                data[i] = data[2 * i].op(&data[2 * i + 1]);
            }
            self.len = len;
            self.data = data;
        }
        pub fn set(&mut self, mut idx: usize, x: &T) {
            idx += self.len;
            self.data[idx] = x.clone();
            idx /= 2;
            while idx > 0 {
                self.data[idx] = self.data[2 * idx].op(&self.data[2 * idx + 1]);
                idx /= 2;
            }
        }
        pub fn query(&self, a: usize, b: usize) -> T {
            let (mut vl, mut vr) = (T::id(), T::id());
            let (mut l, mut r) = (a + self.len, b + self.len);
            while l < r {
                if l % 2 == 1 {
                    vl = vl.op(&self.data[l]);
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    vr = self.data[r].op(&vr);
                }
                l /= 2;
                r /= 2;
            }
            vl.op(&vr)
        }
    }
    impl<T: Monoid> std::ops::Index<usize> for SegmentTree<T> {
        type Output = T;
        fn index(&self, idx: usize) -> &T {
            &self.data[idx + self.len]
        }
    }
}
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
macro_rules! define_mod {
    ($ struct_name : ident , $ modulo : expr ) => {
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
        }
    };
}
define_mod!(P, 998244353);
type Modint = modint::Modint<P>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AffineMonoid(Modint, Modint);
impl segment_tree::Monoid for AffineMonoid {
    fn id() -> Self {
        Self(Modint::new(1), Modint::new(0))
    }
    fn op(&self, x: &Self) -> Self {
        Self(x.0 * self.0, x.0 * self.1 + x.1)
    }
}

fn main() {
    let mut stdin = scanner::build_scanner();
    let (n, q) = scan!(stdin;usize, usize);
    let a = scan!(stdin; [[u64;2]; n]);
    let a: Vec<_> = a
        .iter()
        .map(|a| AffineMonoid(Modint::new(a[0]), Modint::new(a[1])))
        .collect();
    let mut st = segment_tree::SegmentTree::<AffineMonoid>::new(n);
    st.build(&a);
    for _ in 0..q {
        let t = scan!(stdin; i32);
        if t == 0 {
            let (p, c, d) = scan!(stdin; usize,i32,i32);
            let m = AffineMonoid(Modint::from(c), Modint::from(d));
            st.set(p, &m);
        } else {
            let (l, r, x) = scan!(stdin; usize, usize,i32);
            let x = Modint::from(x);
            let m = st.query(l, r);
            println!("{}", m.0 * x + m.1);
        }
    }
}

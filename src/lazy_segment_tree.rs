use cargo_snippet::snippet;

#[snippet("lazy_segment_tree")]
mod lazy_segment_tree {
    pub trait Element<T: Effecter>: Clone {
        fn id() -> Self;
        fn op(&self, x: Self) -> Self;
        fn affect(&self, x: T) -> Self;
    }
    pub trait Effecter: Clone + Eq {
        fn id() -> Self;
        fn op(&self, x: Self) -> Self;
    }
    pub struct LazySegmentTree<T: Element<S>, S: Effecter> {
        data: Vec<T>,
        lazy: Vec<S>,
        len: usize,
        height: usize,
    }
    impl<T: Element<S>, S: Effecter> LazySegmentTree<T, S> {
        #[inline]
        fn get_proper_size(len: usize) -> (usize, usize) {
            let ret = len.next_power_of_two();
            (ret, ret.trailing_zeros() as usize)
        }
        #[inline]
        fn propagate(&mut self, k: usize) {
            if self.lazy[k] != S::id() {
                self.lazy[2 * k] = self.lazy[2 * k].op(self.lazy[k].clone());
                self.lazy[2 * k + 1] = self.lazy[2 * k + 1].op(self.lazy[k].clone());
                self.data[k] = self.apply(k);
                self.lazy[k] = S::id();
            }
        }
        #[inline]
        fn apply(&mut self, k: usize) -> T {
            if self.lazy[k] != S::id() {
                self.data[k].clone()
            } else {
                self.data[k].affect(self.lazy[k].clone())
            }
        }
        #[inline]
        fn recalc(&mut self, mut k: usize) {
            k >>= 1;
            while k > 0 {
                self.data[k] = self.apply(2 * k).op(self.apply(2 * k + 1));
                k >>= 1;
            }
        }
        #[inline]
        fn thrust(&mut self, k: usize) {
            (1..=self.height).rev().for_each(|i| self.propagate(k >> i));
        }
        pub fn new(len: usize) -> Self {
            let (len, height) = Self::get_proper_size(len);
            Self {
                data: vec![T::id(); 2 * len],
                lazy: vec![S::id(); 2 * len],
                len,
                height,
            }
        }
        pub fn update(&mut self, l: usize, r: usize, x: &S) {
            if l >= r {
                return;
            }
            let (mut l, mut r) = (l + self.len, r + self.len);
            self.thrust(l);
            self.thrust(r - 1);
            while l < r {
                if l % 2 == 1 {
                    self.lazy[l] = x.op(self.lazy[l].clone());
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    self.lazy[r] = self.lazy[r].op(x.clone());
                }
            }
        }
        pub fn query(&mut self, l: usize, r: usize) -> T {
            if l >= r {
                return T::id();
            }
            let (mut l, mut r) = (l + self.len, r + self.len);
            self.thrust(l);
            self.thrust(r);
            let (mut L, mut R) = (T::id(), T::id());
            while l < r {
                if l % 2 == 1 {
                    L = L.op(self.apply(l));
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    R = R.op(self.apply(r));
                }
            }
            L.op(R)
        }
    }
}

#[snippet("lazy_segment_tree")]
mod monoids {
    use super::lazy_segment_tree::*;
    #[derive(Clone, Eq, PartialEq)]
    struct SumMonoid(i64);
    impl Element<SetMonoid> for SumMonoid {
        fn id() -> Self {
            Self(0)
        }
        fn op(&self, other: Self) -> Self {
            Self(self.0 + other.0)
        }
        fn affect(&self, x: SetMonoid) -> Self {
            Self(x.0)
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    struct SetMonoid(i64);
    impl Effecter for SetMonoid {
        fn id() -> Self {
            Self(-1)
        }
        fn op(&self, other: Self) -> Self {
            Self(other.0)
        }
    }
}

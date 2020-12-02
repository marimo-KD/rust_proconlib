//ATTENTION
//まだです。
use algebra::*;
pub struct LazySegmentTree<T, S>
where
    T: Monoid,
    S: Monoid,
{
    data: Vec<T>,
    lazy: Vec<S>,
    affecter: fn(T, S) -> T,
    len: usize,
    height: usize,
}
impl<T, S> LazySegmentTree<T, S>
where
    T: Monoid,
    S: Monoid,
{
    fn propagate(&mut self, k: usize) {
        if self.lazy[k] != S::identity() {
            self.lazy[2 * k] = S::op(self.lazy[2 * k].clone(), self.lazy[k].clone());
            self.lazy[2 * k + 1] = S::op(self.lazy[2 * k + 1].clone(), self.lazy[k].clone());
            self.data[k] = self.apply(k);
            self.lazy[k] = S::identity();
        }
    }
    fn apply(&mut self, k: usize) -> T {
        if self.lazy[k] != S::identity() {
            self.data[k].clone()
        } else {
            (self.affecter)(self.data[k].clone(), self.lazy[k].clone())
        }
    }
    fn recalc(&mut self, mut k: usize) {
        k >>= 1;
        while k > 0 {
            self.data[k] = T::op(self.apply(2 * k), self.apply(2 * k + 1));
            k >>= 1;
        }
    }
    fn thrust(&mut self, k: usize) {
        (1..=self.height).rev().for_each(|i| self.propagate(k >> i));
    }
    pub fn new(len: usize, affecter: fn(T, S) -> T) -> Self {
        let len = len.next_power_of_two();
        let height = len.trailing_zeros() as usize;
        Self {
            data: vec![T::identity(); 2 * len],
            lazy: vec![S::identity(); 2 * len],
            affecter,
            len,
            height,
        }
    }
    pub fn update(&mut self, a: usize, b: usize, x: &S) {
        if a >= b {
            return;
        }
        let (mut l, mut r) = (a + self.len, b + self.len);
        self.thrust(l);
        self.thrust(r - 1);
        while l < r {
            if l % 2 == 1 {
                self.lazy[l].op_from_left(x.clone());
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                self.lazy[r].op_from_right(x.clone());
            }
        }
        self.recalc(a);
        self.recalc(b);
    }
    pub fn query(&mut self, l: usize, r: usize) -> T {
        if l >= r {
            return T::identity();
        }
        let (mut l, mut r) = (l + self.len, r + self.len);
        self.thrust(l);
        self.thrust(r);
        let (mut L, mut R) = (T::identity(), T::identity());
        while l < r {
            if l % 2 == 1 {
                L.op_from_right(self.apply(l));
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                R.op_from_left(self.apply(r));
            }
        }
        T::op(L, R)
    }
}

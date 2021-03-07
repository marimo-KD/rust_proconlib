//ATTENTION
//まだです。
use algebra::*;
use std::ops::Range;
#[derive(Debug, Clone)]
pub struct LazySegmentTree<T, E>
where
    T: Monoid,
    E: Monoid,
{
    data: Box<[T]>,
    lazy: Box<[E]>,
    affecter: fn(T, E) -> T,
    len: usize,
    log: usize,
}
impl<T, E> LazySegmentTree<T, E>
where
    T: Monoid + Copy,
    E: Monoid + Copy,
{
    fn propagate(&mut self, k: usize) {
        self.apply(k << 1, self.lazy[k]);
        self.apply((k << 1) + 1, self.lazy[k]);
        self.lazy[k] = E::identity();
    }
    fn apply(&mut self, k: usize, e: E) {
        self.data[k] = (self.affecter)(self.data[k], e);
        self.lazy[k] = E::op(self.lazy[k], e);
    }
    fn recalc(&mut self, k: usize) {
        let mut k = k >> (k.trailing_zeros() as usize);
        k >>= 1;
        while k != 0 {
            self.data[k] = T::op(self.data[k << 1], self.data[(k << 1) + 1]);
            k >>= 1;
        }
    }
    fn thrust(&mut self, k: usize) {
        (1..=self.log).rev().for_each(|i| self.propagate(k >> i));
    }
    pub fn new(len: usize, affecter: fn(T, E) -> T) -> Self {
        let len = len.next_power_of_two();
        let height = len.trailing_zeros() as usize;
        Self {
            data: vec![T::identity(); 2 * len].into_boxed_slice(),
            lazy: vec![E::identity(); 2 * len].into_boxed_slice(),
            affecter,
            len,
            log: height,
        }
    }
    pub fn new_with_init(initializer: &[T], affecter: fn(T, E) -> T) -> Self {
        let len = initializer.len().next_power_of_two();
        let mut data = vec![T::identity(); 2 * len].into_boxed_slice();
        for (idx, val) in initializer.iter().enumerate() {
            data[idx + len] = val.clone();
        }
        for i in (1..len).rev() {
            data[i] = T::op(data[i << 1], data[(i << 1) + 1]);
        }
        Self {
            data,
            lazy: vec![E::identity(); 2 * len].into_boxed_slice(),
            affecter,
            len,
            log: len.trailing_zeros() as usize,
        }
    }
    pub fn update(&mut self, q: Range<usize>, x: E) {
        let (mut l, mut r) = (q.start + self.len, q.end + self.len);
        self.thrust(l);
        self.thrust(r - 1);
        while l < r {
            if l & 1 != 0 {
                self.apply(l, x);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.apply(r, x);
            }
            l >>= 1;
            r >>= 1;
        }
        self.recalc(q.start + self.len);
        self.recalc(q.end + self.len);
    }
    pub fn query(&mut self, q: Range<usize>) -> T {
        let (mut l, mut r) = (q.start + self.len, q.end + self.len);
        self.thrust(l);
        self.thrust(r - 1);
        let (mut fl, mut fr) = (T::identity(), T::identity());
        while l < r {
            if l & 1 != 0 {
                fl = T::op(fl, self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                fr = T::op(self.data[r], fr);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(fl, fr)
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

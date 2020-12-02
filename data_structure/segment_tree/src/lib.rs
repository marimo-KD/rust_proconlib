use algebra::*;
use std::ops::{Range, RangeBounds};
pub struct SegmentTree<T: Monoid> {
    data: Box<[T]>,
    len: usize,
}
impl<T: Monoid + Copy> SegmentTree<T> {
    pub fn new(len: usize) -> Self {
        let len = len.next_power_of_two();
        Self {
            data: vec![T::identity(); 2 * len].into_boxed_slice(),
            len,
        }
    }
    pub fn new_with_init(&mut self, initializer: &Vec<T>) {
        let len = initializer.len().next_power_of_two();
        let mut data = vec![T::identity(); 2 * len].into_boxed_slice();
        for (idx, val) in initializer.iter().enumerate() {
            data[idx + len] = val.clone();
        }
        for i in (1..len).rev() {
            data[i] = T::op(data[2 * i], data[2 * i + 1]);
        }
        self.len = len;
        self.data = data;
    }
    pub fn set(&mut self, mut idx: usize, x: &T) {
        idx += self.len;
        self.data[idx] = x.clone();
        idx /= 2;
        while idx > 0 {
            self.data[idx] = T::op(self.data[2 * idx], self.data[2 * idx + 1]);
            idx /= 2;
        }
    }
    pub fn query(&self, a: usize, b: usize) -> T {
        let (mut vl, mut vr) = (T::identity(), T::identity());
        let (mut l, mut r) = (a + self.len, b + self.len);
        while l < r {
            if l % 2 == 1 {
                vl.op_from_right(self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                vr.op_from_left(self.data[r]);
            }
            l /= 2;
            r /= 2;
        }
        T::op(vl, vr)
    }
}
impl<T: Monoid> std::ops::Index<usize> for SegmentTree<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.data[idx + self.len]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use algebra_struct::*;
    #[test]
    fn rsq() {
        let mut st = SegmentTree::new(10);
        st.set(1, &algebra_struct::AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(4));
        st.set(8, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(4));
        st.set(0, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(8));
        st.set(4, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(12));
    }
}

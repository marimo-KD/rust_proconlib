use super::alge_struct::alge;
use cargo_snippet::snippet;
#[snippet]
#[snippet(include = "alge")]
mod segment_tree {
    use super::alge;
    use alge::Monoid;
    pub struct SegmentTree<T: Monoid> {
        data: Vec<T>,
        len: usize,
    }
    impl<T: Monoid + Copy> SegmentTree<T> {
        fn get_proper_size(len: usize) -> usize {
            len.next_power_of_two()
        }
        pub fn new(len: usize) -> Self {
            let len = Self::get_proper_size(len);
            Self {
                data: vec![T::identity(); 2 * len],
                len,
            }
        }
        pub fn new_with_init(&mut self, initializer: &Vec<T>) {
            let len = Self::get_proper_size(initializer.len());
            let mut data = vec![T::identity(); 2 * len];
            for (idx, val) in initializer.iter().enumerate() {
                data[idx + len] = val.clone();
            }
            for i in (1..len).rev() {
                data[i] = data[2 * i].op(data[2 * i + 1]);
            }
            self.len = len;
            self.data = data;
        }
        pub fn set(&mut self, mut idx: usize, x: &T) {
            idx += self.len;
            self.data[idx] = x.clone();
            idx /= 2;
            while idx > 0 {
                self.data[idx] = self.data[2 * idx].op(self.data[2 * idx + 1]);
                idx /= 2;
            }
        }
        pub fn query(&self, a: usize, b: usize) -> T {
            let (mut vl, mut vr) = (T::identity(), T::identity());
            let (mut l, mut r) = (a + self.len, b + self.len);
            while l < r {
                if l % 2 == 1 {
                    vl = vl.op(self.data[l]);
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    vr = self.data[r].op(vr);
                }
                l /= 2;
                r /= 2;
            }
            vl.op(vr)
        }
    }
    impl<T: Monoid> std::ops::Index<usize> for SegmentTree<T> {
        type Output = T;
        fn index(&self, idx: usize) -> &T {
            &self.data[idx + self.len]
        }
    }
}
#[snippet("segment_tree")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddMonoid(i32);
#[snippet("segment_tree")]
impl alge::Semigroup for AddMonoid {
    fn op(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
#[snippet("segment_tree")]
impl alge::Monoid for AddMonoid {
    fn identity() -> Self {
        Self(0)
    }
}

#[snippet("segment_tree")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaxMonoid(i32);
#[snippet("segment_tree")]
impl alge::Semigroup for MaxMonoid {
    fn op(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }
}
#[snippet("segment_tree")]
impl alge::Monoid for MaxMonoid {
    fn identity() -> Self {
        Self(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rsq() {
        let mut st = segment_tree::SegmentTree::new(10);
        st.set(1, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(4));
        st.set(8, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(4));
        st.set(0, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(8));
        st.set(4, &AddMonoid(4));
        assert_eq!(st.query(0, 5), AddMonoid(12));
    }
}

use cargo_snippet::snippet;
#[snippet("segment_tree")]
mod segment_tree {
    pub trait Monoid: Clone {
        type Item: Clone + std::fmt::Debug;
        fn id() -> Self;
        fn op(&self, x: &Self) -> Self;
    }
    pub struct SegmentTree<T: Monoid> {
        data: Vec<T>,
        len: usize,
    }
    impl<T: Monoid> SegmentTree<T> {
        fn get_proper_size(len: usize) -> usize {
            let mut ret = 1;
            while ret < len {
                ret *= 2;
            }
            ret
        }
        pub fn new(len: usize) -> Self {
            let len = Self::get_proper_size(len);
            Self {
                data: vec![T::id(); 2 * len],
                len,
            }
        }
        pub fn init(&mut self, initializer: &Vec<T>) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct add_monoid(i32);
    impl segment_tree::Monoid for add_monoid {
        type Item = i32;
        fn id() -> Self {
            add_monoid(0)
        }
        fn op(&self, x: &Self) -> Self {
            add_monoid(self.0 + x.0)
        }
    }
    #[test]
    fn rsq() {
        let mut st = segment_tree::SegmentTree::new(10);
        st.set(1, &add_monoid(4));
        assert_eq!(st.query(0, 5), add_monoid(4));
        st.set(8, &add_monoid(4));
        assert_eq!(st.query(0, 5), add_monoid(4));
        st.set(0, &add_monoid(4));
        assert_eq!(st.query(0, 5), add_monoid(8));
        st.set(4, &add_monoid(4));
        assert_eq!(st.query(0, 5), add_monoid(12));
    }
}

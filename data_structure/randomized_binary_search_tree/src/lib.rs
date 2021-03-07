use algebra::Monoid;
use my_range::MyRange;
use std::cmp::Ordering;
use std::mem::replace;
use std::ops::Range;
pub struct RBST<T: Monoid>(Option<Box<Node<T>>>);
impl<T: Monoid> RBST<T> {
    #[inline(always)]
    pub fn none() -> Self {
        Self(None)
    }
    pub fn build(data: &[T], range: Range<usize>) -> Self {
        match range.len() {
            0 => Self::none(),
            1 => Self(Some(Box::new(Node::new(data[range.start].clone())))),
            x => {
                let i = Self::rand() as usize % x + range.start;
                let mut ret = Node {
                    left: Self::build(data, range.start..i),
                    right: Self::build(data, i + 1..range.end),
                    size: 1,
                    data: data[i].clone(),
                    sum: T::identity(),
                };
                ret.update();
                Self(Some(Box::new(ret)))
            }
        }
    }
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.as_ref().size
    }
    pub fn access(&self, k: usize) -> &T {
        assert!(!self.is_none());
        let rank = self.as_ref().left.size();
        match k.cmp(&rank) {
            Ordering::Equal => &self.as_ref().data,
            Ordering::Less => self.as_ref().left.access(k),
            Ordering::Greater => self.as_ref().right.access(k - rank - 1),
        }
    }
    pub fn merge(x: Self, y: Self) -> Self {
        match (x, y) {
            (Self(None), y) => y,
            (x, Self(None)) => x,
            (mut x, mut y) => {
                if Self::rand() % ((x.size() + y.size()) as u64) < x.size() as u64 {
                    x.as_mut().right = Self::merge(replace(&mut x.as_mut().right, Self::none()), y);
                    x.as_mut().update();
                    x
                } else {
                    y.as_mut().left = Self::merge(x, replace(&mut y.as_mut().left, Self::none()));
                    y.as_mut().update();
                    y
                }
            }
        }
    }
    pub fn split(self, k: usize) -> (Self, Self) {
        match self {
            Self(None) => (Self::none(), Self::none()),
            mut slf => {
                if k <= slf.as_ref().left.size() {
                    let x = replace(&mut slf.as_mut().left, Self::none()).split(k);
                    slf.as_mut().left = x.1;
                    slf.as_mut().update();
                    (x.0, slf)
                } else {
                    let x = replace(&mut slf.as_mut().right, Self::none())
                        .split(k - slf.as_ref().left.size() - 1);
                    slf.as_mut().right = x.0;
                    slf.as_mut().update();
                    (slf, x.1)
                }
            }
        }
    }
    pub fn query(&mut self, range: Range<usize>) -> T {
        assert!((0..self.size()).contains_range(&range));
        let sel = replace(self, Self::none());
        let (a, b) = sel.split(range.start);
        let (b, c) = b.split(range.end - range.start);
        let ret = b.sum();
        *self = Self::merge(Self::merge(a, b), c);
        ret
    }
    pub fn insert(&mut self, k: usize, val: T) {
        let sel = replace(self, Self::none());
        let (l, r) = sel.split(k);
        let a = Self(Some(Box::new(Node::new(val))));
        *self = Self::merge(Self::merge(l, a), r);
    }
    pub fn erase(&mut self, k: usize) -> T {
        let sel = replace(self, Self::none());
        let (l, r) = sel.split(k);
        let (del, r) = r.split(1);
        *self = Self::merge(l, r);
        del.unwrap().data
    }
    #[inline(always)]
    fn as_ref(&self) -> &Box<Node<T>> {
        self.0.as_ref().unwrap()
    }
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Box<Node<T>> {
        self.0.as_mut().unwrap()
    }
    #[inline(always)]
    fn unwrap(self) -> Box<Node<T>> {
        self.0.unwrap()
    }
    #[inline(always)]
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
    fn sum(&self) -> T {
        if let Some(ref x) = self.0 {
            x.sum.clone()
        } else {
            T::identity()
        }
    }
    fn rand() -> u64 {
        static mut RAND_X: u64 = 123456789;
        static mut RAND_Y: u64 = 987654321;
        static mut RAND_Z: u64 = 1000000007;
        static mut RAND_W: u64 = 1145141919;
        unsafe {
            let t = RAND_X ^ (RAND_X << 11);
            RAND_X = RAND_Y;
            RAND_Y = RAND_Z;
            RAND_Z = RAND_W;
            RAND_W = (RAND_W ^ (RAND_W >> 19)) ^ (t ^ (t >> 8));
            RAND_W
        }
    }
}
struct Node<T: Monoid> {
    left: RBST<T>,
    right: RBST<T>,
    size: usize,
    data: T,
    sum: T,
}
impl<T: Monoid> Node<T> {
    fn new(val: T) -> Self {
        Self {
            left: RBST::none(),
            right: RBST::none(),
            size: 1,
            data: val.clone(),
            sum: val,
        }
    }
    fn update(&mut self) {
        self.size = self.left.size() + self.right.size() + 1;
        self.sum = T::op(T::op(self.left.sum(), self.data.clone()), self.right.sum());
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

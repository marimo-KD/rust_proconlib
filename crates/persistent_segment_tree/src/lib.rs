use algebra::Monoid;
use my_range::MyRange;
use std::ops::Range;
use std::rc::Rc;

#[derive(Clone)]
struct Tree<T: Monoid>(Option<Rc<Node<T>>>);
impl<T: Monoid> Tree<T> {
    pub fn new(x: Node<T>) -> Self {
        Self(Some(Rc::new(x)))
    }
    pub fn merge(left: Self, right: Self) -> Self {
        let data = T::op(
            left.0.as_ref().unwrap().data.clone(),
            right.0.as_ref().unwrap().data.clone(),
        );
        Self::new(Node::new(data, left, right))
    }
    pub fn build(range: Range<usize>, data: &[T]) -> Self {
        //! [l,r)以下のTreeを作ります。
        //! 一番下にはdataが入ります。
        if range.len() == 1 {
            Self::new(Node::new(data[range.start].clone(), Self(None), Self(None)) )
        } else {
            let (left, right) = range.half();
            Self::merge(Self::build(left, data), Self::build(right, data))
        }
    }
    pub fn update(&self, idx: usize, x: &T, range: Range<usize>) -> Self {
        if !range.contains(&idx) {
            self.clone()
        } else if range.len() == 1 {
            Self::new(Node::new(x.clone(), Self(None), Self(None)))
        } else {
            let (left, right) = range.half();
            Self::merge(
                self.0.as_ref().unwrap().left.update(idx, x, left),
                self.0.as_ref().unwrap().right.update(idx, x, right),
            )
        }
    }
    pub fn query(&self, q: Range<usize>, range: Range<usize>) -> T {
        if !range.contains_range(&q) {
            T::identity()
        } else if q.contains_range(&range) {
            self.0.as_ref().unwrap().data.clone()
        } else {
            let (left, right) = range.half();
            T::op(
                self.0.as_ref().unwrap().left.query(q.clone(), left),
                self.0.as_ref().unwrap().right.query(q, right),
            )
        }
    }
}
struct Node<T: Monoid> {
    data: T,
    left: Tree<T>,
    right: Tree<T>,
}
impl<T: Monoid> Node<T> {
    pub fn new(data: T, left: Tree<T>, right: Tree<T>) -> Self {
        Self { data, left, right }
    }
}
pub struct PersistentSegmentTree<T: Monoid> {
    root: Tree<T>,
    len: usize,
}

impl<T: Monoid> PersistentSegmentTree<T> {
    pub fn new(len: usize) -> Self {
        let root = Tree::build(0..len, &vec![T::identity(); len]);
        Self { root, len }
    }
    pub fn new_with_init(initializer: &[T]) -> Self {
        let len = initializer.len();
        let root = Tree::build(0..len, initializer);
        Self { root, len }
    }
    pub fn set(&self, idx: usize, x: &T) -> Self {
        let nr = self.root.update(idx, x, 0..self.len);
        Self {
            root: nr,
            len: self.len,
        }
    }
    pub fn query(&self, q: Range<usize>) -> T {
        self.root.query(q, 0..self.len)
    }
}

use algebra::Monoid;
use my_range::MyRange;
use std::mem::{replace, swap};
use std::ops::Range;

// 列をあつかうやーつ
#[derive(Debug)]
pub struct SplayTree<T: Monoid>(Option<Box<Node<T>>>);
impl<T: Monoid> SplayTree<T> {
    pub fn size(&self) -> usize {
        if let Some(ref x) = self.0 {
            x.size
        } else {
            0
        }
    }
    pub fn access(&mut self, k: usize) -> &T {
        assert!(self.size() > k);
        self.splay(k);
        &self.as_ref().data
    }
    pub fn split(mut self, k: usize) -> (Self, Self) {
        match self {
            Self(None) => (Self::none(), Self::none()),
            _ => {
                if k <= self.as_ref().left.size() {
                    // 左に行きます。
                    let x = replace(&mut self.as_mut().left, Self::none()).split(k);
                    self.as_mut().left = x.1;
                    self.as_mut().update();
                    (x.0, self)
                } else {
                    // 右に行きます。
                    let x = replace(&mut self.as_mut().right, Self::none())
                        .split(k - self.as_mut().left.size() - 1);
                    self.as_mut().right = x.0;
                    self.as_mut().update();
                    (self, x.1)
                }
            }
        }
    }
    pub fn merge(x: Self, y: Self) -> Self {
        match (x, y) {
            (Self(None), y) => y,
            (x, Self(None)) => x,
            (mut x, y) => {
                x.splay(x.size() - 1);
                x.as_mut().right = y;
                x.as_mut().update();
                x
            }
        }
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
    pub fn query(&mut self, range: Range<usize>) -> T {
        assert!((0..self.size()).contains_range(&range));
        let sel = replace(self, Self::none());
        let (a, b) = sel.split(range.start);
        let (b, c) = b.split(range.end - range.start);
        let ret = b.sum();
        *self = Self::merge(Self::merge(a, b), c);
        ret
    }
    pub fn none() -> Self {
        Self(None)
    }
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
    fn as_ref(&self) -> &Box<Node<T>> {
        self.0.as_ref().unwrap()
    }
    fn as_mut(&mut self) -> &mut Box<Node<T>> {
        self.0.as_mut().unwrap()
    }
    fn take(&mut self) -> Box<Node<T>> {
        self.0.take().unwrap()
    }
    fn unwrap(self) -> Box<Node<T>> {
        self.0.unwrap()
    }
    fn sum(&self) -> T {
        if let Some(ref x) = self.0 {
            x.sum.clone()
        } else {
            T::identity()
        }
    }
    fn splay(&mut self, k: usize) {
        if self.is_none() {
            return;
        }
        let mut left_tree: SplayTree<T> = Self(Some(Box::new(Node::new(T::identity()))));
        let mut right_tree: SplayTree<T> = Self(Some(Box::new(Node::new(T::identity()))));
        let mut lnode = left_tree.as_mut();
        let mut rnode = right_tree.as_mut();
        let mut now = self.take();
        let mut cnt = 0;
        loop {
            if k == now.left.size() + cnt {
                break;
            } else if k < now.left.size() + cnt {
                // 左の方に潜ります。
                if now.left.is_none() {
                    break;
                }
                if k < now.left.as_ref().left.size() + cnt - 1 {
                    now = Self::rotr(now);
                    if now.left.is_none() {
                        break;
                    }
                }
                let mut nl = now.left.take();
                swap(&mut now, &mut nl);
                nl.update();
                rnode.left = Self(Some(nl));
                rnode.update();
                rnode = rnode.left.as_mut();
            } else {
                // 右の方に潜ります。
                cnt += now.left.size() + 1;
                if now.right.is_none() {
                    break;
                }
                if k > now.right.as_ref().left.size() + cnt + 1 {
                    now = Self::rotl(now);
                    if now.right.is_none() {
                        break;
                    }
                }
                let mut nr = now.right.take();
                swap(&mut now, &mut nr);
                nr.update();
                lnode.right = Self(Some(nr));
                lnode.update();
                lnode = lnode.right.as_mut();
            }
        }
        rnode.left = now.right;
        rnode.update();
        lnode.right = now.left;
        lnode.update();
        now.left = left_tree.take().right;
        now.right = right_tree.take().left;
        now.update();
        *self = Self(Some(now));
    }
    fn rotr(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = root.left.take();
        root.left = new_root.right;
        root.update();
        new_root.right = Self(Some(root));
        new_root.update();
        new_root
    }
    fn rotl(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = root.right.take();
        root.right = new_root.left;
        root.update();
        new_root.left = Self(Some(root));
        new_root.update();
        new_root
    }
}
#[derive(Debug)]
struct Node<T: Monoid> {
    left: SplayTree<T>,
    right: SplayTree<T>,
    size: usize,
    data: T,
    sum: T,
}
impl<T: Monoid> Node<T> {
    fn new(init: T) -> Self {
        Self {
            left: SplayTree::none(),
            right: SplayTree::none(),
            size: 1,
            data: init.clone(),
            sum: init,
        }
    }
    fn update(&mut self) {
        self.size = self.left.size() + self.right.size() + 1;
        self.sum = T::op(T::op(self.left.sum(), self.data.clone()), self.right.sum());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use algebra_struct::AddMonoid;
    #[test]
    fn splay_tree_test() {
        let mut st: SplayTree<AddMonoid> = SplayTree::none();
        st.insert(0, AddMonoid(32));
        st.insert(0, AddMonoid(16));
        st.insert(0, AddMonoid(8));
        st.insert(0, AddMonoid(4));
        st.insert(0, AddMonoid(2));
        st.insert(0, AddMonoid(1));
        assert_eq!(st.query(0..6), AddMonoid(63));
        eprintln!("{:#?}", st);
    }
}

use std::rc::Rc;

#[derive(Clone)]
struct SkewList<T: Clone>(Option<Rc<SkewListNode<T>>>);
impl<T: Clone> SkewList<T> {
    pub fn none() -> Self {
        Self(None)
    }
    pub fn new_with_node(size: usize, root: SkewTree<T>, next: SkewList<T>) -> Self {
        Self(Some(Rc::new(SkewListNode { size, root, next })))
    }
    pub fn access(&self, idx: usize) -> &T {
        let size = self.0.as_ref().unwrap().size;
        let inner = self.0.as_ref().unwrap();
        if idx < size {
            inner.root.access(size, idx)
        } else {
            inner.next.access(idx - size)
        }
    }
    pub fn update(&self, idx: usize, val: T) -> Self {
        let inner = self.0.as_ref().unwrap();
        let size = inner.size;
        if idx < size {
            Self::new_with_node(size, inner.root.update(size, idx, val), inner.next.clone())
        } else {
            Self::new_with_node(size, inner.root.clone(), inner.next.update(idx - size, val))
        }
    }
}
struct SkewListNode<T: Clone> {
    size: usize,
    root: SkewTree<T>,
    next: SkewList<T>,
}

/// 完全2分木です。
#[derive(Clone)]
struct SkewTree<T: Clone>(Option<Rc<SkewTreeNode<T>>>);
impl<T: Clone> SkewTree<T> {
    pub fn new_with_node(value: T, left: SkewTree<T>, right: SkewTree<T>) -> Self {
        Self(Some(Rc::new(SkewTreeNode { value, left, right })))
    }
    pub fn new_with_leaf(value: T) -> Self {
        Self(Some(Rc::new(SkewTreeNode {
            value,
            left: Self::none(),
            right: Self::none(),
        })))
    }
    pub fn none() -> Self {
        Self(None)
    }
    pub fn access(&self, size: usize, idx: usize) -> &T {
        let inner = self.0.as_ref().unwrap();
        if idx == 0 {
            &inner.value
        } else {
            assert!(size != 1);
            let rem = idx - 1;
            let ch_size = size / 2;
            if rem < ch_size {
                inner.left.access(ch_size, rem)
            } else {
                inner.right.access(ch_size, rem - ch_size)
            }
        }
    }
    pub fn update(&self, size: usize, idx: usize, val: T) -> Self {
        let inner = self.0.as_ref().unwrap();
        if idx == 0 {
            Self::new_with_leaf(val)
        } else {
            assert!(size != 1);
            let rem = idx - 1;
            let ch_size = size / 2;
            if rem < ch_size {
                Self::new_with_node(
                    inner.value.clone(),
                    inner.left.update(ch_size, rem, val),
                    inner.right.clone(),
                )
            } else {
                Self::new_with_node(
                    inner.value.clone(),
                    inner.left.clone(),
                    inner.right.update(ch_size, rem, val),
                )
            }
        }
    }
}

struct SkewTreeNode<T: Clone> {
    value: T,
    left: SkewTree<T>,
    right: SkewTree<T>,
}

pub struct SkewBinaryList<T: Clone>(SkewList<T>);
impl<T: Clone> SkewBinaryList<T> {
    pub fn new() -> Self {
        Self(SkewList::none())
    }
    pub fn update(&self, idx: usize, val: T) -> Self {
        Self(self.0.update(idx, val))
    }
    pub fn access(&self, idx: usize) -> &T {
        self.0.access(idx)
    }
    pub fn push_front(&self, val: T) -> Self {
        match self.0 .0 {
            None => Self(SkewList::new_with_node(
                1,
                SkewTree::new_with_leaf(val),
                SkewList::none(),
            )),
            Some(ref root) => {
                let rsize = root.size;
                match root.next.0 {
                    Some(ref nxt) if rsize == nxt.size => Self(SkewList::new_with_node(
                        1 + rsize + nxt.size,
                        SkewTree::new_with_node(val, root.root.clone(), nxt.root.clone()),
                        nxt.next.clone(),
                    )),
                    _ => Self(SkewList::new_with_node(
                        1,
                        SkewTree::new_with_leaf(val),
                        self.0.clone(),
                    )),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::*;
    #[test]
    fn skew_binary_list_test() {
        let mut rng = thread_rng();
        let n = 1<<10;
        let seq = (0..n).map(|_| rng.gen()).collect::<Vec<i32>>();
        let mut sbl = SkewBinaryList::new();
        for i in (0..n).rev() {
            sbl = sbl.push_front(seq[i]);
        }
        for i in 0..n {
            assert_eq!(*sbl.access(i), seq[i]);
        }
    }
}

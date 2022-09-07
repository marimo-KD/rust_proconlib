struct Node {
    size: usize,
    child: [Tree; 2],
}
impl Node {
    fn none() -> Self {
        Self {
            size: 0,
            child: [Tree::none(), Tree::none()],
        }
    }
}
struct Tree(Option<Box<Node>>);
impl Tree {
    fn none() -> Self {
        Tree(None)
    }
    fn new(val: Node) -> Self {
        Tree(Some(Box::new(val)))
    }
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
    fn is_some(&self) -> bool {
        self.0.is_some()
    }
    fn size(&self) -> usize {
        if let Some(x) = self.0.as_ref() {
            x.size
        } else {
            0
        }
    }
    fn insert(&mut self, val: u64, bit: i32) {
        // 下からbit目のところです。
        if self.is_none() {
            *self = Tree::new(Node::none());
        }
        let node = self.0.as_mut().unwrap();
        node.size += 1;
        if bit >= 0 {
            let f = (val >> bit) & 1;
            node.child[f as usize].insert(val, bit - 1);
        }
    }
    fn remove(&mut self, val: u64, bit: i32) {
        assert!(self.is_some());
        let node = self.0.as_mut().unwrap();
        node.size -= 1;
        if node.size == 0 {
            *self = Tree::none();
            return;
        }
        if bit >= 0 {
            let f = (val >> bit) & 1;
            node.child[f as usize].remove(val, bit - 1);
        }
    }
    fn min(&self, bit: i32, bias: u64) -> u64 {
        //! min in ^bias
        assert!(self.is_some());
        let node = self.0.as_ref().unwrap();
        if bit < 0 {
            0
        } else {
            let f = (bias >> bit) & 1;
            if f == 0 {
                if node.child[0].is_some() {
                    node.child[0].min(bit - 1, bias)
                } else {
                    node.child[1].min(bit - 1, bias) | (1 << bit)
                }
            } else {
                if node.child[1].is_some() {
                    node.child[1].min(bit - 1, bias) | (1 << bit)
                } else {
                    node.child[0].min(bit - 1, bias)
                }
            }
        }
    }
    fn max(&self, bit: i32, bias: u64) -> u64 {
        assert!(self.is_some());
        let node = self.0.as_ref().unwrap();
        if bit < 0 {
            0
        } else {
            let f = (bias >> bit) & 1;
            if f == 0 {
                if node.child[1].is_some() {
                    node.child[1].min(bit - 1, bias) | (1 << bit)
                } else {
                    node.child[0].min(bit - 1, bias)
                }
            } else {
                if node.child[0].is_some() {
                    node.child[0].min(bit - 1, bias)
                } else {
                    node.child[1].min(bit - 1, bias) | (1 << bit)
                }
            }
        }
    }
    fn kth(&self, k: usize, bit: i32) -> u64 {
        //! k is 0-indexed.
        assert!(self.is_some());
        let node = self.0.as_ref().unwrap();
        if bit < 0 {
            0
        } else {
            let m = node.child[0].size();
            if k < m {
                node.child[0].kth(k, bit - 1)
            } else {
                node.child[1].kth(k - m, bit - 1) | (1 << bit)
            }
        }
    }
    fn count_lower(&self, val: u64, bit: i32) -> usize {
        //! count elements s.t. < val
        if self.is_some() || bit < 0 {
            0
        } else {
            let node = self.0.as_ref().unwrap();
            let f = (val >> bit) & 1;
            node.child[f as usize].count_lower(val, bit - 1)
                + if f == 1 { node.child[0].size() } else { 0 }
        }
    }
    fn count(&self, val: u64, bit: i32) -> usize {
        if bit < 0 {
            self.size()
        } else if self.is_none() {
            0
        } else {
            let node = self.0.as_ref().unwrap();
            let f = (val >> bit) & 1;
            node.child[f as usize].count(val, bit - 1)
        }
    }
}

pub struct BinaryTrie {
    root: Tree,
    bitlen: i32,
}
impl BinaryTrie {
    pub fn new(bitlen: i32) -> Self {
        BinaryTrie {
            root: Tree::none(),
            bitlen,
        }
    }
    pub fn size(&self) -> usize {
        self.root.size()
    }
    pub fn insert(&mut self, val: u64) {
        self.root.insert(val, self.bitlen - 1);
    }
    pub fn remove(&mut self, val: u64) {
        self.root.remove(val, self.bitlen - 1);
    }
    pub fn max(&self) -> u64 {
        self.root.max(self.bitlen - 1, 0)
    }
    pub fn min(&self) -> u64 {
        self.root.min(self.bitlen - 1, 0)
    }
    pub fn max_xor(&self, x: u64) -> u64 {
        self.root.max(self.bitlen - 1, x)
    }
    pub fn min_xor(&self, x: u64) -> u64 {
        self.root.min(self.bitlen - 1, x)
    }
    pub fn lower_bound(&self, val: u64) -> usize {
        self.root.count_lower(val, self.bitlen - 1)
    }
    pub fn upper_bound(&self, val: u64) -> usize {
        self.root.count_lower(val + 1, self.bitlen - 1)
    }
    pub fn kth(&self, k: usize) -> u64 {
        self.root.kth(k, self.bitlen - 1)
    }
    pub fn count(&self, val: u64) -> usize {
        self.root.count(val, self.bitlen - 1)
    }
}

macro_rules! get {
    ($($t:tt),*; $n:expr) => {
        {
            let stdin = std::io::stdin();
            let ret = std::io::BufRead::lines(stdin.lock()).take($n).map(|line| {
                let line = line.unwrap();
                let mut it = line.split_whitespace();
                _get!(it; $($t),*)
            }).collect::<Vec<_>>();
            ret
        }
    };
    ($($t:tt),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();
            _get!(it; $($t),*)
        }
    };
}
macro_rules! _get {
    ($it:ident; [char]) => {
        _get!($it; String).chars().collect::<Vec<_>>()
    };
    ($it:ident; [u8]) => {
        _get!($it; String).bytes().collect::<Vec<_>>()
    };
    ($it:ident; usize1) => {
        $it.next().unwrap().parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1
    };
    ($it:ident; [usize1]) => {
        $it.map(|s| s.parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1).collect::<Vec<_>>()
    };
    ($it:ident; [$t:ty]) => {
        $it.map(|s| s.parse::<$t>().unwrap_or_else(|e| panic!("{}", e))).collect::<Vec<_>>()
    };
    ($it:ident; $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap_or_else(|e| panic!("{}", e))
    };
    ($it:ident; $($t:tt),+) => {
        ($(_get!($it; $t)),*)
    };
}
mod segment_tree {
    pub trait Monoid: Clone {
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
    impl<T: Monoid> std::ops::Index<usize> for SegmentTree<T> {
        type Output = T;
        fn index(&self, idx: usize) -> &T {
            &self.data[idx + self.len]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct add_monoid(i64);
impl segment_tree::Monoid for add_monoid {
    fn id() -> Self {
        add_monoid(0)
    }
    fn op(&self, x: &Self) -> Self {
        add_monoid(self.0 + x.0)
    }
}
fn main() {
    let (n, q) = get!(i32, i32);
    let a = get!([i64]);
    let query = get!([i64];q as usize);
    let a: Vec<_> = a.iter().map(|x| add_monoid(*x)).collect();
    let mut st = segment_tree::SegmentTree::new(n as usize);
    st.init(&a);
    for i in 0..q {
        let i = i as usize;
        if query[i][0] == 0 {
            let (p, x) = (query[i][1] as usize, query[i][2]);
            st.set(p, &add_monoid(st[p].0 + x));
        } else {
            let (l, r) = (query[i][1] as usize, query[i][2] as usize);
            println!("{}", st.query(l, r).0);
        }
    }
}

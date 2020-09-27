mod mo {
    pub trait State {
        type Answer: std::clone::Clone + std::default::Default;
        fn add_left(&mut self, idx: usize);
        fn add_right(&mut self, idx: usize);
        fn erase_left(&mut self, idx: usize);
        fn erase_right(&mut self, idx: usize);
        fn answer(&mut self, idx: usize) -> Self::Answer;
    }
    pub struct Mo<T: State> {
        state: T,
        n: usize,
        query: Vec<(usize, usize)>,
    }
    impl<T: State> Mo<T> {
        pub fn new(state: T, n: usize) -> Self {
            Self {
                state,
                n,
                query: vec![(0, 0); 0],
            }
        }
        pub fn add(&mut self, l: usize, r: usize) {
            self.query.push((l, r));
        }
        pub fn run(&mut self) -> Vec<T::Answer> {
            let q = self.query.len();
            let w = (self.n as f64).sqrt() as usize;
            let mut ret = vec![T::Answer::default(); q];
            let mut order: Vec<_> = (0..q).collect();
            order.sort_by(|&i, &j| {
                if self.query[i].0 / w != self.query[j].0 / w {
                    self.query[i].0.cmp(&self.query[j].0)
                } else {
                    self.query[i].1.cmp(&self.query[j].1)
                }
            });
            let (mut lb, mut rb) = (0, 0);
            for i in order {
                let (li, ri) = self.query[i];
                while lb > li {
                    lb -= 1;
                    self.state.add_left(lb);
                }
                while rb < ri {
                    self.state.add_right(rb);
                    rb += 1;
                }
                while lb < li {
                    self.state.erase_left(lb);
                    lb += 1;
                }
                while rb > ri {
                    rb -= 1;
                    self.state.erase_right(rb);
                }
                ret[i] = self.state.answer(i);
            }
            ret
        }
    }
}

struct FenwickTree<T> {
    data: Vec<T>,
}
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::default::Default
            + std::marker::Copy,
    > FenwickTree<T>
{
    pub fn new(n: usize) -> Self {
        let mut ret = Self {
            data: vec![std::default::Default::default(); n + 1],
        };
        ret.build();
        ret
    }
    pub fn new_with_init(mut data: Vec<T>) -> Self {
        let mut ret = Self { data };
        ret.build();
        ret
    }
    fn build(&mut self) {
        for i in 1..self.data.len() as i32 {
            let j = (i + (i & -i)) as usize;
            if j < self.data.len() {
                self.data[j] = self.data[j] + self.data[i as usize];
            }
        }
    }
    pub fn add(&mut self, mut k: usize, x: T) {
        k += 1;
        while k < self.data.len() {
            self.data[k] = self.data[k] + x;
            k += {
                let k = k as i32;
                (k & -k) as usize
            }
        }
    }
    pub fn query0(&mut self, mut k: usize) -> T {
        let mut ret: T = std::default::Default::default();
        while k > 0 {
            ret = ret + self.data[k];
            k -= {
                let k = k as i32;
                (k & -k) as usize
            };
        }
        ret
    }
    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.query0(r) - self.query0(l)
    }
    pub fn len(&self) -> usize {
        self.data.len() - 1
    }
}

pub mod scanner {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::io;
    use std::io::BufRead;
    use std::str::FromStr;
    struct Tokenizer<T: BufRead> {
        source: T,
        tokens: VecDeque<String>,
        buf: String,
    }
    impl<T: BufRead> Tokenizer<T> {
        fn new(source: T) -> Self {
            Self {
                source,
                tokens: VecDeque::new(),
                buf: String::new(),
            }
        }
    }
    impl<T: BufRead> Iterator for Tokenizer<T> {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            while self.tokens.is_empty() {
                self.buf.clear();
                self.source.read_line(&mut self.buf).unwrap();
                for i in self.buf.split_whitespace() {
                    self.tokens.push_back(String::from(i));
                }
            }
            Some(self.tokens.pop_front().unwrap())
        }
    }
    pub struct Scanner<T: BufRead> {
        tokenizer: Tokenizer<T>,
    }
    impl<T: BufRead> Scanner<T> {
        pub fn new(source: T) -> Self {
            Self {
                tokenizer: Tokenizer::new(source),
            }
        }
        pub fn scan<U: FromStr>(&mut self) -> U {
            self.tokenizer.next().unwrap().parse::<U>().ok().unwrap()
        }
    }
    pub fn build_scanner() -> Scanner<io::StdinLock<'static>> {
        let stdin = Box::leak(Box::new(io::stdin()));
        Scanner {
            tokenizer: Tokenizer::new(stdin.lock()),
        }
    }
}
macro_rules ! scan {($ scanner : ident ; [char ] ) => {$ scanner . scan ::< String > () . chars () . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [u8 ] ) => {$ scanner . scan ::< String > () . bytes . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [$ ($ t : tt ) ,+; $ n : expr ] ) => {(0 ..$ n ) . map (| _ | ($ (scan ! ($ scanner ;$ t ) ) ,* ) ) . collect ::< Vec < _ >> () } ; ($ scanner : ident ; $ t : ty ) => {$ scanner . scan ::<$ t > () } ; ($ scanner : ident ; $ ($ t : tt ) ,+ ) => {($ (scan ! ($ scanner ; $ t ) ) ,* ) } ; }

struct MoState {
    bit: FenwickTree<i64>,
    a: Vec<usize>,
    inv: i64,
}
impl mo::State for MoState {
    type Answer = i64;
    #[inline]
    fn add_left(&mut self, idx: usize) {
        // eprintln!("add_left called");
        self.inv += self.bit.query0(self.a[idx]);
        self.bit.add(self.a[idx], 1);
    }
    #[inline]
    fn add_right(&mut self, idx: usize) {
        // eprintln!("add_right called");
        self.inv += self.bit.query(self.a[idx], self.bit.len());
        self.bit.add(self.a[idx], 1);
    }
    #[inline]
    fn erase_left(&mut self, idx: usize) {
        // eprintln!("erase_left called");
        self.bit.add(self.a[idx], -1);
        self.inv -= self.bit.query0(self.a[idx]);
    }
    #[inline]
    fn erase_right(&mut self, idx: usize) {
        // eprintln!("erase_right called");
        self.bit.add(self.a[idx], -1);
        self.inv -= self.bit.query(self.a[idx], self.bit.len());
    }
    #[inline]
    fn answer(&mut self, _: usize) -> Self::Answer {
        self.inv
    }
}

fn main() {
    let mut stdin = scanner::build_scanner();
    let (n, q) = scan!(stdin; usize, usize);
    let mut a: Vec<_> = (0..n).map(|i| (scan!(stdin; i64), i)).collect();
    a.sort();
    let mut vs = vec![0; n];
    for i in 0..n {
        vs[a[i].1] = i;
    }
    let state = MoState {
        bit: FenwickTree::new(n),
        a: vs,
        inv: 0,
    };
    let mut mo = mo::Mo::new(state, n);
    for _ in 0..q {
        let (l, r) = scan!(stdin; usize, usize);
        mo.add(l, r);
    }
    let ans = mo.run();
    for i in ans {
        println!("{}", i);
    }
}

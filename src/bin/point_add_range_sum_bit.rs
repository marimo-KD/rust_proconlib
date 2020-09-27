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
        pub fn scan2<U: FromStr, V: FromStr>(&mut self) -> (U, V) {
            (self.scan::<U>(), self.scan::<V>())
        }
        pub fn scan3<U: FromStr, V: FromStr, W: FromStr>(&mut self) -> (U, V, W) {
            let (a, b) = self.scan2::<U, V>();
            (a, b, self.scan::<W>())
        }
        pub fn scan4<U: FromStr, V: FromStr, W: FromStr, X: FromStr>(&mut self) -> (U, V, W, X) {
            let (a, b, c) = self.scan3::<U, V, W>();
            (a, b, c, self.scan::<X>())
        }
        pub fn scan5<U: FromStr, V: FromStr, W: FromStr, X: FromStr, Y: FromStr>(
            &mut self,
        ) -> (U, V, W, X, Y) {
            let (a, b, c, d) = self.scan4::<U, V, W, X>();
            (a, b, c, d, self.scan::<Y>())
        }
    }
    pub fn build_scanner() -> Scanner<io::StdinLock<'static>> {
        let stdin = Box::leak(Box::new(io::stdin()));
        Scanner {
            tokenizer: Tokenizer::new(stdin.lock()),
        }
    }
}
macro_rules ! scan {($ scanner : ident ; [char ] ) => {$ scanner . scan ::< String > () . chars () . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [u8 ] ) => {$ scanner . scan ::< String > () . bytes . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [$ t : tt ; $ n : expr ] ) => {(0 ..$ n ) . map (| _ | scan ! ($scanner; $t ) ) . collect ::< Vec < _ >> () } ; ($ scanner : ident ; $ t : ty ) => {$ scanner . scan ::<$ t > () } ; ($ scanner : ident ; $ ($ t : tt ) ,+ ) => {($ (scan ! ($ scanner ; $ t ) ) ,* ) } ; }
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
    fn new(n: usize) -> Self {
        let mut ret = Self {
            data: vec![std::default::Default::default(); n + 1],
        };
        ret.build();
        ret
    }
    fn new_with_init(mut init: Vec<T>) -> Self {
        init.insert(0, std::default::Default::default());
        let mut ret = Self { data: init };
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
    fn add(&mut self, mut k: usize, x: T) {
        k += 1;
        while k < self.data.len() {
            self.data[k] = self.data[k] + x;
            k += {
                let k = k as i32;
                (k & -k) as usize
            }
        }
    }
    fn query0(&mut self, mut k: usize) -> T {
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
    fn query(&mut self, l: usize, r: usize) -> T {
        self.query0(r) - self.query0(l)
    }
}
fn main() {
    let mut stdin = scanner::build_scanner();
    let (n, q) = scan!(stdin; usize, usize);
    let a = scan!(stdin; [i64; n]);
    let mut bit: FenwickTree<i64> = FenwickTree::new_with_init(a);
    for _ in 0..q {
        let ty = scan!(stdin; usize);
        if ty == 0 {
            let (p, x) = scan!(stdin; usize, i64);
            bit.add(p, x);
        } else {
            let (l, r) = scan!(stdin; usize, usize);
            println!("{}", bit.query(l, r));
        }
    }
}

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
            (self.scan::<U>(), self.scan::<V>(), self.scan::<W>())
        }
        pub fn scan4<U: FromStr, V: FromStr, W: FromStr, X: FromStr>(&mut self) -> (U, V, W, X) {
            (
                self.scan::<U>(),
                self.scan::<V>(),
                self.scan::<W>(),
                self.scan::<X>(),
            )
        }
        pub fn scan5<U: FromStr, V: FromStr, W: FromStr, X: FromStr, Y: FromStr>(
            &mut self,
        ) -> (U, V, W, X, Y) {
            (
                self.scan::<U>(),
                self.scan::<V>(),
                self.scan::<W>(),
                self.scan::<X>(),
                self.scan::<Y>(),
            )
        }
        pub fn scan_vec<U: FromStr>(&mut self, n: usize) -> Vec<U> {
            (0..n).map(|_| self.scan::<U>()).collect()
        }
        pub fn scan_vec2<U: FromStr, V: FromStr>(&mut self, n: usize) -> Vec<(U, V)> {
            (0..n).map(|_| self.scan2::<U, V>()).collect()
        }
        pub fn scan_vec3<U: FromStr, V: FromStr, W: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(U, V, W)> {
            (0..n).map(|_| self.scan3::<U, V, W>()).collect()
        }
        pub fn scan_vec4<U: FromStr, V: FromStr, W: FromStr, X: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(U, V, W, X)> {
            (0..n).map(|_| self.scan4::<U, V, W, X>()).collect()
        }
        pub fn scan_vec5<U: FromStr, V: FromStr, W: FromStr, X: FromStr, Y: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(U, V, W, X, Y)> {
            (0..n).map(|_| self.scan5::<U, V, W, X, Y>()).collect()
        }
    }
    pub fn build_scanner() -> Scanner<io::StdinLock<'static>> {
        let stdin = Box::leak(Box::new(io::stdin()));
        Scanner {
            tokenizer: Tokenizer::new(stdin.lock()),
        }
    }
}
fn main() {
    let mut stdin = scanner::build_scanner();
    let a = stdin.scan::<u64>();
    let s = [
        (a - 1).next_power_of_two().trailing_zeros() - 1,
        (a - 1).next_power_of_two().trailing_zeros(),
    ][(a - 1).is_power_of_two() as usize];
    println!("{}", [s as u64, 0][(a == 1) as usize]);
    println!("{}", (a-1).is_power_of_two() as usize);
}

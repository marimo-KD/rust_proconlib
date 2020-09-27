use cargo_snippet::snippet;
#[snippet("scanner")]
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
#[snippet("scanner")]
macro_rules! scan {
    ($scanner:ident; [char]) => {
        $scanner.scan::<String>().chars().collect::<Vec<_>>()
    };
    ($scanner:ident; [u8]) => {
        $scanner.scan::<String>().bytes.collect::<Vec<_>>()
    };
    ($scanner:ident; [$($t:tt),+; $n:expr]) => {
        (0..$n).map(|_| ($(scan!($scanner;$t)),*)).collect::<Vec<_>>()
    };
    ($scanner:ident; $t:ty) => {
        $scanner.scan::<$t>()
    };
    ($scanner:ident; $($t:tt),+) => {
        ($(scan!($scanner; $t)),*)
    };
}

#[test]
fn scanner_test() {
    let mut stdin = scanner::build_scanner();
    println!("{}", scan!(stdin; i32));
    println!("{:?}", scan!(stdin; [i32,i32;10]));
}

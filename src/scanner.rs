use cargo_snippet::snippet;
#[snippet]
pub mod scanner {
    use std::collections::VecDeque;
    use std::io;
    use std::io::BufRead;
    use std::iter::Peekable;
    use std::str::{FromStr, SplitWhitespace};

    struct Tokenizer<T: BufRead> {
        source: T,
        buf: Box<str>,
        token: Peekable<SplitWhitespace<'static>>,
    }
    impl<T: BufRead> Tokenizer<T> {
        fn new(source: T) -> Self {
            Self {
                source,
                buf: "".to_string().into_boxed_str(),
                token: "".split_whitespace().peekable(),
            }
        }
        fn read(&mut self) {
            while self.token.peek().is_none() {
                let mut line = String::new();
                self.source
                    .read_line(&mut line)
                    .expect("Failed to get a line. Maybe an IO error occured");
                self.buf = line.into_boxed_str();
                // This code looks very dangerous.
                // self.buf isn't move, so self.buf's address won't be changed.
                // And self.buf's lifetime is longer than self.token.
                // Therefore, actually, this code is safe.
                // Self reference struct can't be defined safely. :(
                self.token = unsafe { std::mem::transmute::<_, &'static str>(&*self.buf) }
                    .split_whitespace()
                    .peekable();
            }
        }
        fn next(&mut self) -> Option<&str> {
            self.read();
            self.token.next()
        }
    }
    // struct Tokenizer<T: BufRead> {
    //     source: T,
    //     tokens: VecDeque<String>,
    //     buf: String,
    // }
    // impl<T: BufRead> Tokenizer<T> {
    //     fn new(source: T) -> Self {
    //         Self {
    //             source,
    //             tokens: VecDeque::new(),
    //             buf: String::new(),
    //         }
    //     }
    // }
    // impl<T: BufRead> Iterator for Tokenizer<T> {
    //     type Item = String;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         while self.tokens.is_empty() {
    //             self.buf.clear();
    //             self.source.read_line(&mut self.buf).unwrap();
    //             for i in self.buf.split_whitespace() {
    //                 self.tokens.push_back(String::from(i));
    //             }
    //         }
    //         Some(self.tokens.pop_front().unwrap())
    //     }
    // }
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
    impl Scanner<io::StdinLock<'static>> {
        pub fn new_stdin() -> Self {
            let stdin = Box::leak(Box::new(io::stdin()));
            Scanner::new(stdin.lock())
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
    let mut stdin = scanner::Scanner::new_stdin();
    let n = scan!(stdin; usize);
    let a = scan!(stdin; [i32; n]);
}

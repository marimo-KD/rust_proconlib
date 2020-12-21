use std::io::BufRead;
use std::str::{FromStr, SplitWhitespace};
use std::thread_local;
use std::{cell::RefCell, io};

pub trait Readable {
    type Output;
    fn read(input: &str) -> Self::Output;
}
impl<T: FromStr> Readable for T {
    type Output = Self;
    fn read(input: &str) -> Self::Output {
        input.parse().ok().unwrap()
    }
}
struct Tokenizer<T: BufRead> {
    source: T,
    token: SplitWhitespace<'static>,
}
impl<T: BufRead> Tokenizer<T> {
    fn read(&mut self) {
        let mut line = String::new();
        self.source
            .read_line(&mut line)
            .expect("an IO error occured");
        self.token = Box::leak(line.into_boxed_str()).split_whitespace();
    }
    fn next(&mut self) -> &str {
        if let Some(x) = self.token.next() {
            x
        } else {
            self.read();
            self.next()
        }
    }
}
pub struct Scanner<T: BufRead> {
    tokenizer: Tokenizer<T>,
}
impl<T: BufRead> Scanner<T> {
    pub fn new(source: T) -> Self {
        Self {
            tokenizer: Tokenizer {
                source,
                token: "".split_whitespace(),
            },
        }
    }
    pub fn scan<U: Readable>(&mut self) -> U::Output {
        U::read(self.tokenizer.next())
    }
}

thread_local! (
    pub static STDIN: RefCell<Scanner<std::io::StdinLock<'static>>> = {
        let stdin = Box::leak(Box::new(io::stdin()));
        RefCell::new(Scanner::new(stdin.lock()))
    }
);
#[macro_export]
macro_rules! scan {
    ([char]) => {
        STDIN.with(|stdin| stdin.borrow_mut().scan::<String>().chars().collect::<Vec<_>>())
    };
    ([u8]) => {
        STDIN.with(|stdin| stdin.borrow_mut().scan::<String>().bytes().collect::<Vec<_>>())
    };
    ([$($t:tt),*; $n:expr]) => {
        (0..$n).map(|_| ($(scan!($t)),*)).collect::<Vec<_>>()
    };
    ($t:ty) => {
        STDIN.with(|stdin| stdin.borrow_mut().scan::<$t>())
    };
    ($($t:ty),+) => {
        ($(scan!($t)),*)
    };
}

#[allow(non_camel_case_types)]
pub struct usize1();
impl Readable for usize1 {
    type Output = usize;
    fn read(input: &str) -> Self::Output {
        input.parse::<usize>().unwrap() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scanner_test() {
        let a = scan!(usize);
        let _a = scan!([usize1; a]);
    }
}

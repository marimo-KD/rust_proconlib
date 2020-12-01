use std::io;
use std::io::BufRead;
use std::str::{FromStr, SplitWhitespace};

struct Tokenizer<T: BufRead> {
    source: T,
    token: SplitWhitespace<'static>,
}
impl<T: BufRead> Tokenizer<T> {
    fn new(source: T) -> Self {
        Self {
            source,
            token: "".split_whitespace(),
        }
    }
    fn read(&mut self) {
        let mut line = String::new();
        self.source
            .read_line(&mut line)
            .expect("Failed to get a line. Maybe an IO error occured");
        let buf: &'static str = Box::leak(line.into_boxed_str());
        self.token = buf.split_whitespace();
    }
    fn next(&mut self) -> Option<&str> {
        if let Some(x) = self.token.next() {
            Some(x)
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
#[macro_export]
macro_rules! scan {
    ($scanner:ident; [char]) => {
        $scanner.scan::<String>().chars().collect::<Vec<_>>()
    };
    ($scanner:ident; [u8]) => {
        $scanner.scan::<String>().bytes().collect::<Vec<_>>()
    };
    ($scanner:ident; [$($t:tt),+; $n:expr]) => {
        (0..$n).map(|_| ($(scan!($scanner;$t)),*)).collect::<Vec<_>>()
    };
    ($scanner:ident; usize1) => {
        $scanner.scan::<usize>() - 1
    };
    ($scanner:ident; $t:ty) => {
        $scanner.scan::<$t>()
    };
    ($scanner:ident; $($t:tt),+) => {
        ($(scan!($scanner; $t)),*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scanner_test() {
        let mut stdin = Scanner::new_stdin();
    }
}

//! # Bundled libraries
//!
//! - `mario v0.1.0` → `crate::mario` (source: local filesystem, license: **missing**)

use mario::*;
use std::collections::HashMap;
use std::io::*;

fn main() {
    let mut input = MarIo::new_stdio();
    let q = input.usize();
    let mut map = HashMap::new();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for _ in 0..q {
        let ty = input.usize();
        match ty {
            0 => {
                let (k, v) = read!(input: i64, i64);
                map.insert(k, v);
            }
            1 => {
                let k = input.i64();
                writeln!(out, "{}", map.get(&k).unwrap_or(&0)).unwrap();
            }
            _ => {
                unreachable!();
            }
        }
    }
}

// The following code was expanded by `cargo-equip`.

#[allow(unused)]
pub mod mario {
    pub use crate::read;
    use std::io;
    use std::io::{BufRead, BufReader, ErrorKind, Result};

    pub mod token {
        use std::str::FromStr;
        pub trait Token {
            type Output;
            fn parse(s: &str) -> Self::Output;
        }

        impl<T: FromStr> Token for T {
            type Output = T;
            fn parse(s: &str) -> Self::Output {
                s.parse::<T>().unwrap_or_else(|_| panic!("Parse Error"))
            }
        }

        #[allow(non_camel_case_types)]
        pub struct usize1();
        impl Token for usize1 {
            type Output = usize;
            fn parse(s: &str) -> Self::Output {
                let i = s.parse::<usize>().unwrap_or_else(|_| panic!("Parse Error"));
                i.checked_sub(1).unwrap()
            }
        }

        #[allow(non_camel_case_types)]
        pub struct isize1();
        impl Token for isize1 {
            type Output = isize;
            fn parse(s: &str) -> Self::Output {
                let i = s.parse::<isize>().unwrap_or_else(|_| panic!("Parse Error"));
                i.checked_sub(1).unwrap()
            }
        }
    }
    use token::Token;

    fn read_until_whitespace<R: BufRead + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
        let mut read = 0;
        loop {
            let (done, used) = {
                let available = match r.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                match available
                    .iter()
                    .enumerate()
                    .find(|(_, x)| x.is_ascii_whitespace())
                {
                    Some((i, _)) => {
                        buf.extend_from_slice(&available[..=i]);
                        (true, i + 1)
                    }
                    None => {
                        buf.extend_from_slice(available);
                        (false, available.len())
                    }
                }
            };
            r.consume(used);
            read += used;
            if done || used == 0 {
                return Ok(read);
            }
        }
    }

    macro_rules! prim_method {
        ($name:ident: $T:ty) => {
            pub fn $name(&mut self) -> $T {
                self.parse::<$T>()
            }
        };
        ($name:ident) => {
            prim_method!($name: $name);
        };
    }
    macro_rules! prim_methods {
        ($name:ident: $T:ty; $($rest:tt)*) => {
            prim_method!($name:$T);
            prim_methods!($($rest)*);
        };
        ($name:ident; $($rest:tt)*) => {
            prim_method!($name);
            prim_methods!($($rest)*);
        };
        () => ()
    }

    // MARimo + IO -> MarIo
    // work as a tokenizer.
    pub struct MarIo<I: BufRead> {
        reader: I,
    }
    impl<I: BufRead> MarIo<I> {
        pub fn new(reader: I) -> Self {
            Self { reader }
        }
        fn token(&mut self) -> String {
            let mut buf = Vec::new();
            loop {
                buf.clear();
                read_until_whitespace(&mut self.reader, &mut buf)
                    .expect("困ってしまいましたね。どうやら入力がうまくいかないようです。");
                let len = buf.len();
                match len {
                    0 => panic!("もう入力終わってますよ。"),
                    1 if buf[0].is_ascii_whitespace() => (),
                    _ => {
                        if buf[len - 1].is_ascii_whitespace() {
                            buf.truncate(len - 1);
                        }
                        break;
                    }
                }
            }
            unsafe { String::from_utf8_unchecked(buf) }
        }
        pub fn parse<T: Token>(&mut self) -> T::Output {
            T::parse(&self.token())
        }
        prim_methods! {
            u8; u16; u32; u64; u128; usize;
            i8; i16; i32; i64; i128; isize;
            f32; f64;
            char; string: String;
        }
    }

    // MarIo with stdin/out
    impl MarIo<BufReader<io::Stdin>> {
        pub fn new_stdio() -> Self {
            Self::new(BufReader::new(io::stdin()))
        }
    }

    #[macro_export]
    macro_rules! read {
        ($stdin:ident: [char]) => {
            $stdin.string().chars().collect::<Vec<_>>()
        };
        ($stdin:ident: [u8]) => {
            $stdin.string().bytes().collect::<Vec<_>>()
        };
        ($stdin:ident: [$($t:tt),*; $n:expr]) => {
            (0..$n).map(|_| ($(read!($stdin: $t)),*)).collect::<Vec<_>>()
        };
        ($stdin:ident: $t:ty) => {
            $stdin.parse::<$t>()
        };
        ($stdin:ident: $($t:ty),+) => {
            ($(read!($stdin: $t)),*)
        };
    }
}

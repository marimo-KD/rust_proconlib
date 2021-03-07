use std::io;
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Result, Write};

pub mod token;
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
pub struct MarIo<I: BufRead, O: Write> {
    reader: I,
    writer: BufWriter<O>,
}
impl<I: BufRead, O: Write> MarIo<I, O> {
    pub fn new(reader: I, writer: O) -> Self {
        Self {
            reader,
            writer: BufWriter::new(writer),
        }
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
impl MarIo<BufReader<io::Stdin>, io::Stdout> {
    pub fn new_stdio() -> Self {
        Self::new(BufReader::new(io::stdin()), io::stdout())
    }
}

// MarIo with stdinlock/stdoutlock
impl MarIo<io::StdinLock<'static>, io::StdoutLock<'static>> {
    pub unsafe fn new_stdinlock() -> Self {
        //! UNSAFE!!!
        //! This function makes StdinLock<'static> by Box::leak.
        //! Dropping MarIo made by this func causes memory leak and locking stdin.
        let stdin = Box::leak(Box::new(io::stdin()));
        let stdout = Box::leak(Box::new(io::stdout()));
        Self::new(stdin.lock(), stdout.lock())
    }
}

impl<I: BufRead, O: Write> Write for MarIo<I, O> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tuple() {
        let mut input = MarIo::new_stdio();
        let _ = input.i32();
    }
}

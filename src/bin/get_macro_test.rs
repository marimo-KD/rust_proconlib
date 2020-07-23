use cargo_snippet::snippet;
use std::io::*;
use std::iter;

struct StdinReader<R: BufRead> {
    pub reader: R,
    pub buf: String,
}
impl<R: BufRead> StdinReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::new(),
        }
    }
}

macro_rules! get {
    ($r:expr, $t:ty) => {
        {
            let mut line = &mut $r.buf;
            line.clear();
            $r.reader.read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($r:expr, $($t:ty),*) => {
        {
            let mut line = &mut $r.buf;
            line.clear();
            $r.reader.read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($r:expr, $t:ty; $n:expr) => {
        // items in n lines
        (0..$n).map(|_| get!($r, $t)).collect::<Vec<_>>()
    };
    ($r:expr, $t:ty; one, $n:expr) => {
        iter::once(Default::default()).chain((0..$n).map(|_| get!($r, $t))).collect::<Vec<_>>()
    };
    ($r:expr, $($t:ty),*; $n:expr) => {
        (0..$n).map(|_| get!($r, $($t),*)).collect::<Vec<_>>()
    };
    ($r:expr, $($t:ty),*; one, $n:expr) => {
        iter::once(Default::default()).chain((0..$n).map(|_| get!($r, $($t),*))).collect::<Vec<_>>()
    };
    ($r:expr, $t:ty ;;) => {
        // items in a line
        {
            let mut line = &mut $r.buf;
            line.clear();
            $r.reader.read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($r:expr, $t:ty ;; $n:expr) => {
        (0..$n).map(|_| get!($r, $t ;;)).collect::<Vec<_>>()
    };
}

fn main() {
    let reader = stdin();
    let mut reader = StdinReader::new(reader.lock());
    let a = get!(reader, i32; one, 5);
    println!("{:?}", a);
}

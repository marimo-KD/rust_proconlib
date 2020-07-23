use std::io::*;
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
        (0..$n).map(|_|
                    get!($r, $t)
                   ).collect::<Vec<_>>()
    };
    ($r:expr, $($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    get!($r, $($t),*)
                   ).collect::<Vec<_>>()
    };
    ($r:expr, $t:ty ;;) => {
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
    let reader = std::io::stdin();
    let mut reader = StdinReader::new(reader.lock());
    let writer = std::io::stdout();
    let mut writer = std::io::BufWriter::new(writer.lock());
    let (n, q) = get!(reader, usize, usize);
    let a = get!(reader, i64;;);
    let cumsum = std::iter::once(0)
        .chain(a)
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect::<Vec<i64>>();
    for _ in 0..q {
        let (l, r) = get!(reader, usize, usize);
        writeln!(writer, "{}", cumsum[r] - cumsum[l]).unwrap();
    }
}

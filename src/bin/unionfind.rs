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

mod unionfind {
    pub struct Unionfind {
        par: Vec<i32>,
        group_count: usize,
    }
    impl Unionfind {
        pub fn new(n: usize) -> Self {
            Unionfind {
                par: vec![-1; n as usize],
                group_count: n as usize,
            }
        }
        fn find_root(&mut self, mut x: usize) -> usize {
            while self.par[x] >= 0 {
                x = self.par[x] as usize;
            }
            x
        }
        pub fn unite(&mut self, x: usize, y: usize) {
            let mut rx = self.find_root(x);
            let mut ry = self.find_root(y);
            if rx == ry {
                return;
            }
            if self.par[rx] > self.par[ry] {
                std::mem::swap(&mut rx, &mut ry);
            }
            self.par[rx] += self.par[ry];
            self.par[ry] = rx as i32;
            self.group_count -= 1;
        }
        pub fn is_same_group(&mut self, x: usize, y: usize) -> bool {
            self.find_root(x) == self.find_root(y)
        }
        pub fn get_group_size(&mut self, x: usize) -> usize {
            let rx = self.find_root(x);
            (-self.par[rx]) as usize
        }
        pub fn get_num_of_groups(&self) -> usize {
            self.group_count
        }
    }
}
type Unionfind = unionfind::Unionfind;

fn main() {
    let reader = std::io::stdin();
    let mut reader = StdinReader::new(reader.lock());
    let writer = std::io::stdout();
    let mut writer = BufWriter::new(writer.lock());
    let (n, q) = get!(reader, usize, i32);
    let mut uf = Unionfind::new(n);
    for _ in 0..q {
        let (t, u, v) = get!(reader, usize, usize, usize);
        match t {
            0 => uf.unite(u, v),
            1 => {
                let res = uf.is_same_group(u, v);
                let res = if res { 1 } else { 0 };
                writeln!(writer, "{}", res).unwrap();
            }
            _ => (),
        }
    }
}

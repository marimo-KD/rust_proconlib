use std::io::*;
macro_rules! get {
    ($($t:tt),*; $n:expr) => {
        {
            let stdin = std::io::stdin();
            let ret = std::io::BufRead::lines(stdin.lock()).take($n).map(|line| {
                let line = line.unwrap();
                let mut it = line.split_whitespace();
                _get!(it; $($t),*)
            }).collect::<Vec<_>>();
            ret
        }
    };
    ($($t:tt),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();
            _get!(it; $($t),*)
        }
    };
}
macro_rules! _get {
    ($it:ident; [char]) => {
        _get!($it; String).chars().collect::<Vec<_>>()
    };
    ($it:ident; [u8]) => {
        _get!($it; String).bytes().collect::<Vec<_>>()
    };
    ($it:ident; usize1) => {
        $it.next().unwrap().parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1
    };
    ($it:ident; [usize1]) => {
        $it.map(|s| s.parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1).collect::<Vec<_>>()
    };
    ($it:ident; [$t:ty]) => {
        $it.map(|s| s.parse::<$t>().unwrap_or_else(|e| panic!("{}", e))).collect::<Vec<_>>()
    };
    ($it:ident; $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap_or_else(|e| panic!("{}", e))
    };
    ($it:ident; $($t:tt),+) => {
        ($(_get!($it; $t)),*)
    };
}
use std::ops::{Add, Mul, Sub};
// Complex {{{
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Complex {
    re: f64,
    im: f64,
}
impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.im * other.re + self.re * other.im,
        }
    }
}
impl Complex {
    fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}
// }}}
pub struct FFT {
    omega: Vec<Complex>,
}
impl FFT {
    fn new() -> Self {
        Self { omega: Vec::new() }
    }
    fn stockham(&self, n: usize, s: usize, x: &mut Vec<Complex>, y: &mut Vec<Complex>, inv: bool) {
        let m = n / 2;
        let theta0 = 2.0 * std::f64::consts::PI / n as f64 * if inv { -1.0 } else { 1.0 };
        for p in 0..m {
            let wp = Complex {
                re: (p as f64 * theta0).cos(),
                im: -(p as f64 * theta0).sin(),
            };
            for q in 0..s {
                let a = x[q + s * (p + 0)];
                let b = x[q + s * (p + m)];
                y[q + s * (2 * p + 0)] = a + b;
                y[q + s * (2 * p + 1)] = (a - b) * wp;
            }
        }
    }

    fn _fft(&self, x: &mut Vec<Complex>, inv: bool) {
        // let x.len() is power of 2
        let mut n = x.len();
        let mut s = 1;
        let y = &mut vec![Complex::default(); n];
        while n > 1 {
            self.stockham(n, s, x, y, inv);
            n >>= 1;
            s <<= 1;
            std::mem::swap(x, y);
        }
    }
    pub fn fft(&self, x: &mut Vec<Complex>) {
        self._fft(x, false);
    }
    pub fn ifft(&self, x: &mut Vec<Complex>) {
        self._fft(x, true);
        let n = x.len();
        for i in x.iter_mut() {
            i.re /= n as f64;
            i.im /= n as f64;
        }
    }
    pub fn convolution(&self, mut x: Vec<Complex>, mut y: Vec<Complex>) -> Vec<Complex> {
        let n = x.len() + y.len() - 1;
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        x.resize_with(sz, Default::default);
        y.resize_with(sz, Default::default);
        self.fft(&mut x);
        self.fft(&mut y);
        for i in 0..sz {
            x[i] = x[i] * y[i];
        }
        self.ifft(&mut x);
        x.truncate(n);
        x
    }
}
fn main() {
    let writer = stdout();
    let mut writer = BufWriter::new(writer.lock());
    let n = get!(usize);
    let input = get!(i32,i32;n);
    let mut a = vec![Complex::default(); n];
    let mut b = vec![Complex::default(); n];
    for i in 0..input.len() {
        a[i] = Complex {
            re: input[i].0 as f64,
            im: 0.0,
        };
        b[i] = Complex {
            re: input[i].1 as f64,
            im: 0.0,
        };
    }
    let fft = FFT::new();
    let res = fft.convolution(a, b);
    writeln!(writer, "{}", 0);
    for i in 0..res.len() {
        writeln!(writer, "{}", (res[i].re + 0.1).round());
    }
}

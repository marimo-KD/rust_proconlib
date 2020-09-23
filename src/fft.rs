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
fn stockham(n: usize, s: usize, x: &mut Vec<Complex>, y: &mut Vec<Complex>) {
    let m = n / 2;
    let theta0 = 2.0 * std::f64::consts::PI / n as f64;
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

pub fn fft(x: &mut Vec<Complex>) {
    // let x.len() is power of 2
    let mut n = x.len();
    let mut s = 1;
    let y = &mut vec![Complex::default(); n];
    while n > 1 {
        stockham(n, s, x, y);
        n >>= 1;
        s <<= 1;
        std::mem::swap(x, y);
    }
}
pub fn ifft(x: &mut Vec<Complex>) {
    let n = x.len();
    for i in x.iter_mut() {
        *i = i.conj();
    }
    fft(x);
    for i in x.iter_mut() {
        *i = i.conj();
        i.re /= n as f64;
        i.im /= n as f64;
    }
}
pub fn convolution(mut x: Vec<Complex>, mut y: Vec<Complex>) -> Vec<Complex> {
    let n = x.len() + y.len() - 1;
    let mut sz = 1;
    while sz < n {
        sz *= 2;
    }
    x.resize_with(sz, Default::default);
    y.resize_with(sz, Default::default);
    fft(&mut x);
    fft(&mut y);
    for i in 0..sz {
        x[i] = x[i] * y[i];
    }
    ifft(&mut x);
    x.truncate(n);
    x
}

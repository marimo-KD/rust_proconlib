use std::ops::{Add, Div, Mul, Sub};

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
fn _fft(a: &mut [Complex], inv: bool) {
    //! https://satanic0258.github.io/snippets/math/FFT.html
    //! Stockhamの変種とおもわれる
    let n = a.len();
    assert!(n.is_power_of_two());
    let mask = n - 1;
    let lgn = n.trailing_zeros();
    let mut a = a; // あとのbとライフタイムを揃える
    let mut b = vec![Complex::default(); n].into_boxed_slice();
    let mut b: &mut [Complex] = &mut b;
    let theta0 = 2.0 * std::f64::consts::PI * if inv { -1.0 } else { 1.0 } / n as f64;

    for _i in (0..lgn).rev() {
        std::mem::swap(&mut a, &mut b);
        let i = 1 << _i;
        let theta = theta0 * i as f64;
        let root = Complex {
            re: theta.cos(),
            im: theta.sin(),
        };
        let mut w = Complex { re: 1.0, im: 0.0 };
        for j in (0..n).step_by(i) {
            for k in 0..i {
                a[j + k] = b[(j * 2 & mask) + k] + b[((j * 2 + i) & mask) + k] * w;
            }
            w = w * root;
        }
    }
    if lgn % 2 == 1 {
        b.copy_from_slice(a);
    }
}
pub fn fft(a: &mut [Complex]) {
    _fft(a, false);
}
pub fn ifft(a: &mut [Complex]) {
    _fft(a, true);
    let n = a.len();
    a.iter_mut().for_each(|x| {
        x.re /= n as f64;
        x.im /= n as f64;
    });
}
pub fn convolution(mut x: Vec<Complex>, mut y: Vec<Complex>) -> Vec<Complex> {
    let n = x.len() + y.len() - 1;
    let sz = n.next_power_of_two();
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

use std::io::*;
use std::*;

// {{{
pub mod scanner {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::io;
    use std::io::BufRead;
    use std::str::FromStr;
    struct Tokenizer<T: BufRead> {
        source: T,
        tokens: VecDeque<String>,
        buf: String,
    }
    impl<T: BufRead> Tokenizer<T> {
        fn new(source: T) -> Self {
            Self {
                source,
                tokens: VecDeque::new(),
                buf: String::new(),
            }
        }
    }
    impl<T: BufRead> Iterator for Tokenizer<T> {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            while self.tokens.is_empty() {
                self.buf.clear();
                self.source.read_line(&mut self.buf).unwrap();
                for i in self.buf.split_whitespace() {
                    self.tokens.push_back(String::from(i));
                }
            }
            Some(self.tokens.pop_front().unwrap())
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
    pub fn build_scanner() -> Scanner<io::StdinLock<'static>> {
        let stdin = Box::leak(Box::new(io::stdin()));
        Scanner {
            tokenizer: Tokenizer::new(stdin.lock()),
        }
    }
}
macro_rules ! scan {($ scanner : ident ; [char ] ) => {$ scanner . scan ::< String > () . chars () . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [u8 ] ) => {$ scanner . scan ::< String > () . bytes . collect ::< Vec < _ >> () } ; ($ scanner : ident ; [$ ($ t : tt ) ,+; $ n : expr ] ) => {(0 ..$ n ) . map (| _ | ($ (scan ! ($ scanner ;$ t ) ) ,* ) ) . collect ::< Vec < _ >> () } ; ($ scanner : ident ; $ t : ty ) => {$ scanner . scan ::<$ t > () } ; ($ scanner : ident ; $ ($ t : tt ) ,+ ) => {($ (scan ! ($ scanner ; $ t ) ) ,* ) } ; }

// }}}
mod fft {
    use std::ops::{Add, Mul, Sub};
    #[derive(PartialEq, Copy, Clone, Debug, Default)]
    pub struct Complex {
        pub re: f64,
        pub im: f64,
    }
    impl Complex {
        pub fn new(re: f64, im: f64) -> Self {
            Self { re, im }
        }
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
    fn _fft(a: &mut [Complex], inv: bool) {
        let n = a.len();
        assert!(n.is_power_of_two());
        let mask = n - 1;
        let lgn = n.trailing_zeros();
        let mut a = a;
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
                let l = (j * 2) & mask;
                let r = (l + i) & mask;
                for k in 0..i {
                    a[j + k] = b[l + k] + b[r + k] * w;
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
        for i in 0..n {
            a[i].re /= n as f64;
            a[i].im /= n as f64;
        }
    }
    pub fn convolution(mut x: Vec<Complex>, mut y: Vec<Complex>) -> Vec<Complex> {
        let n = x.len() + y.len() - 1;
        let sz = n.next_power_of_two();
        x.resize(sz, Complex::new(0.0, 0.0));
        y.resize(sz, Complex::new(0.0, 0.0));
        fft(&mut x);
        fft(&mut y);
        x.iter_mut().zip(y.iter()).for_each(|(x, y)| {
            *x = *x * *y;
        });
        ifft(&mut x);
        x.truncate(n);
        x
    }
}
fn main() {
    use fft::Complex;
    let mut stdin = scanner::build_scanner();
    let n = scan!(stdin; usize);
    let mut a = vec![Complex::default(); n];
    let mut b = vec![Complex::default(); n];
    for i in 0..n {
        let (ai, bi) = scan!(stdin; f64, f64);
        a[i] = Complex::new(ai, 0.0);
        b[i] = Complex::new(bi, 0.0);
    }
    let res = fft::convolution(a, b);
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    writeln!(stdout, "0").unwrap();
    for i in res {
        writeln!(stdout, "{}", i.re.round() as i64).unwrap();
    }
}

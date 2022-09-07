// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod
use ntt::*;
use scanner::*;
use static_modint::Modint;
use std::io::*;

type Mint = Modint<ntt::P998244353>;
fn main() {
    let (n, m) = scan!(usize, usize);
    let a = scan!([Mint; n]);
    let b = scan!([Mint; m]);
    let c = convolution(a, b);
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for i in c {
        write!(out, "{} ", i).unwrap();
    }
    writeln!(out).unwrap();
}

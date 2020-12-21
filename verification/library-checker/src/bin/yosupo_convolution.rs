use std::io::*;

use scanner::*;
use ntt::*;
use static_modint::Modint;

fn main(){
    let (n, m) = scan!(usize, usize);
    let a = scan!([Modint<ntt::P998244353>; n]);
    let b = scan!([Modint<ntt::P998244353>; m]);
    let c = convolution(a, b);
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for i in c {
        write!(out, "{} ", i).unwrap();
    }
    writeln!(out).unwrap();
}

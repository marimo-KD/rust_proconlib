use scanner::*;
use std::io::*;
use wavelet_matrix::WaveletMatrix;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    let (n, q) = scan!(usize, usize);
    let a = scan!([u64; n]);
    let wm = WaveletMatrix::new(30, a);
    for _ in 0..q {
        let (l, r, k) = scan!(usize, usize, usize);
        write!(out, "{}\n", wm.quantile(l..r, k)).unwrap();
    }
}

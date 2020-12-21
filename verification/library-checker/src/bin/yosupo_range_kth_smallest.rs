use scanner::*;
use std::io::*;
use wavelet_matrix::WaveletMatrix;

fn main() {
    let mut stdin = Scanner::new_stdin();
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    let (n, q) = scan!(stdin; usize, usize);
    let a = scan!(stdin; [u64; n]);
    let wm = WaveletMatrix::new(30, a);
    for _ in 0..q {
        let (l, r, k) = scan!(stdin; usize, usize, usize);
        write!(out, "{}\n", wm.quantile(l..r, k));
    }
}

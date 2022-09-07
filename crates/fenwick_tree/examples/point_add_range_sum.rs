use mario::*;
use fenwick_tree::FenwickTree;
use std::io::*;

fn main() {
    let mut stdio = MarIo::new_stdio();
    let (n, q) = read!(stdio: usize, usize);
    let a = read!(stdio: [i64; n]);
    let mut bit = FenwickTree::new_with_init(a);
    for _ in 0..q {
        let ty = read!(stdio: u32);
        match ty {
            0 => {
                let (p, x) = read!(stdio: usize, i64);
                bit.add(p, x);
            },
            1 => {
                let (l, r) = read!(stdio: usize, usize);
                writeln!(stdio, "{}", bit.query(l, r));
            },
            _ => {unreachable!()}
        }
    }
}

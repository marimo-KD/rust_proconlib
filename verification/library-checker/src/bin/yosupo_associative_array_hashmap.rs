use mario::*;
use std::collections::HashMap;
use std::io::*;

fn main() {
    let mut input = MarIo::new_stdio();
    let q = input.usize();
    let mut map = HashMap::new();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for _ in 0..q {
        let ty = input.usize();
        match ty {
            0 => {
                let (k, v) = read!(input: i64, i64);
                map.insert(k, v);
            }
            1 => {
                let k = input.i64();
                writeln!(out, "{}", map.get(&k).unwrap_or(&0)).unwrap();
            }
            _ => {
                unreachable!();
            }
        }
    }
}

use cuckoo_hashing::CuckooHashMap;
use scanner::*;
use std::io::*;

fn main() {
    let q = scan!(usize);
    let mut map = CuckooHashMap::new();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for _ in 0..q {
        let ty = scan!(usize);
        match ty {
            0 => {
                let (k, v) = scan!(i64, i64);
                map.insert(k, v);
            }
            1 => {
                let k = scan!(i64);
                writeln!(out, "{}", map.get(&k).unwrap_or(&0)).unwrap();
            }
            _ => {
                unreachable!();
            }
        }
    }
}

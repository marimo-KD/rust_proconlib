use std::io::*;
use binary_trie::*;
use scanner::*;

fn main() {
    let q = scan!(usize);
    let mut trie = BinaryTrie::new(32);
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    for _ in 0..q {
        let (t, x) = scan!(usize, u64);
        if t == 0 {
            if trie.count(x) == 0 {
                trie.insert(x);
            }
        } else if t == 1 {
            if trie.count(x) != 0 {
                trie.remove(x);
            }
        } else {
            writeln!(stdout, "{}", trie.min_xor(x) ^ x).unwrap();
        }
    }
}

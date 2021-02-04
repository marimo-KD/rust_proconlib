//! # Bundled libraries
//!
//! - `cuckoo_hashing v0.1.0` → `crate::cuckoo_hashing` (source: local filesystem, license: **missing**)
//! - `scanner v0.1.0` → `crate::scanner` (source: local filesystem, license: **missing**)

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

// The following code was expanded by `cargo-equip`.

#[allow(unused)]
pub mod cuckoo_hashing {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    use std::mem::MaybeUninit;

    const REDUN: usize = 2;
    pub struct CuckooHashMap<K: Hash + Eq, V> {
        table: [Box<[Option<(K, V)>]>; REDUN],
        hash: [RandomState; REDUN],
        size: usize,
        len: usize,
        len_log: usize,
    }
    impl<K: Hash + Eq, V> CuckooHashMap<K, V> {
        pub fn new() -> Self {
            let mut table: [MaybeUninit<Box<[Option<(K, V)>]>>; REDUN] =
                unsafe { MaybeUninit::uninit().assume_init() };
            let mut hash: [MaybeUninit<RandomState>; REDUN] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for i in 0..REDUN {
                let mut v = Vec::with_capacity(1);
                let array = unsafe {
                    v.set_len(1);
                    v.into_boxed_slice()
                };
                table[i] = MaybeUninit::new(array);
                hash[i] = MaybeUninit::new(RandomState::new());
            }
            let table = unsafe { std::mem::transmute::<_, [Box<[Option<(K, V)>]>; REDUN]>(table) };
            let hash = unsafe { std::mem::transmute::<_, [RandomState; REDUN]>(hash) };
            Self {
                table,
                hash,
                size: 0,
                len: 1,
                len_log: 0,
            }
        }
        pub fn get(&self, k: &K) -> Option<&V> {
            let h = self.hash(k);
            for i in 0..REDUN {
                if let Some((ref rk, ref v)) = self.table[i][h[i]] {
                    if rk == k {
                        return Some(v);
                    }
                }
            }
            None
        }
        pub fn insert(&mut self, mut k: K, mut v: V) -> Option<V> {
            self.size += 1;
            if !self.load_factor_is_ok() {
                self.double();
            }
            let h = self.hash(&k);
            for i in 0..REDUN {
                if self.table[i][h[i]].is_none() {
                    self.table[i][h[i]] = Some((k, v));
                    return None;
                } else {
                    if self.table[i][h[i]].as_ref().unwrap().0 == k {
                        let (_, v) = self.table[i][h[i]].replace((k, v)).unwrap();
                        return Some(v);
                    }
                }
            }
            loop {
                let mut j = 0;
                for _ in 0..self.len_log + 2 {
                    let h = self.hash_one(&k, j);
                    if self.table[j][h].is_none() {
                        self.table[j][h] = Some((k, v));
                        return None;
                    } else {
                        let kv = self.table[j][h].replace((k, v)).unwrap();
                        if self.table[j][h].as_ref().unwrap().0 == kv.0 {
                            return Some(kv.1);
                        }
                        k = kv.0;
                        v = kv.1;
                    }
                    j += 1;
                    if j >= REDUN {
                        j -= REDUN;
                    }
                }
                self.rehash();
            }
        }
        pub fn remove(&mut self, k: &K) -> Option<V> {
            let h = self.hash(k);
            for i in 0..REDUN {
                if self.table[i][h[i]].is_some() {
                    if self.table[i][h[i]].as_ref().unwrap().0 == *k {
                        let (_, v) = self.table[i][h[i]].take().unwrap();
                        self.size -= 1;
                        if self.size * 4 < self.len * REDUN {
                            // if load factor is less than 0.25
                            self.half();
                        }
                        return Some(v);
                    }
                }
            }
            None
        }
        #[inline(always)]
        fn load_factor_is_ok(&self) -> bool {
            // 50% is threshold
            self.size * 2 <= self.len * REDUN
        }
        fn rehash(&mut self) {
            let mut hash: [MaybeUninit<RandomState>; REDUN] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for i in 0..REDUN {
                hash[i] = MaybeUninit::new(RandomState::new());
            }
            let hash = unsafe { std::mem::transmute::<_, [RandomState; REDUN]>(hash) };
            self.hash = hash;
            for i in 0..REDUN {
                for j in 0..self.len {
                    if let Some((k, v)) = self.table[i][j].take() {
                        self.insert(k, v);
                    }
                }
            }
        }
        #[inline]
        fn double(&mut self) {
            self.resize(self.len_log + 1, self.len << 1);
        }
        #[inline]
        fn half(&mut self) {
            self.resize(self.len_log - 1, self.len >> 1);
        }
        fn resize(&mut self, new_len_log: usize, new_len: usize) {
            let mut table: [MaybeUninit<Box<[Option<(K, V)>]>>; REDUN] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for i in 0..REDUN {
                let mut v = Vec::with_capacity(new_len);
                let array = unsafe {
                    v.set_len(new_len);
                    v.into_boxed_slice()
                };
                table[i] = MaybeUninit::new(array);
            }
            let table = unsafe { std::mem::transmute::<_, [Box<[Option<(K, V)>]>; REDUN]>(table) };
            let mut table = std::mem::replace(&mut self.table, table);
            let old_len = std::mem::replace(&mut self.len, new_len);
            self.len_log = new_len_log;
            for i in 0..REDUN {
                for j in 0..old_len {
                    if let Some((k, v)) = table[i][j].take() {
                        self.insert(k, v);
                    }
                }
            }
        }
        fn hash(&self, k: &K) -> [usize; REDUN] {
            let mut ret = [0; REDUN];
            for i in 0..REDUN {
                let mut hasher = self.hash[i].build_hasher();
                k.hash(&mut hasher);
                ret[i] = hasher.finish() as usize & (self.len - 1);
            }
            ret
        }
        fn hash_one(&self, k: &K, idx: usize) -> usize {
            let mut hasher = self.hash[idx].build_hasher();
            k.hash(&mut hasher);
            hasher.finish() as usize & (self.len - 1)
        }
    }
}

#[allow(unused)]
pub mod scanner {
    pub use crate::scan;
    use std::io::BufRead;
    use std::str::{FromStr, SplitWhitespace};
    use std::thread_local;
    use std::{cell::RefCell, io};

    pub trait Readable {
        type Output;
        fn read(input: &str) -> Self::Output;
    }
    impl<T: FromStr> Readable for T {
        type Output = Self;
        fn read(input: &str) -> Self::Output {
            input.parse().ok().unwrap()
        }
    }
    struct Tokenizer<T: BufRead> {
        source: T,
        token: SplitWhitespace<'static>,
    }
    impl<T: BufRead> Tokenizer<T> {
        fn read(&mut self) {
            let mut line = String::new();
            self.source
                .read_line(&mut line)
                .expect("an IO error occured");
            self.token = Box::leak(line.into_boxed_str()).split_whitespace();
        }
        fn next(&mut self) -> &str {
            if let Some(x) = self.token.next() {
                x
            } else {
                self.read();
                self.next()
            }
        }
    }
    pub struct Scanner<T: BufRead> {
        tokenizer: Tokenizer<T>,
    }
    impl<T: BufRead> Scanner<T> {
        pub fn new(source: T) -> Self {
            Self {
                tokenizer: Tokenizer {
                    source,
                    token: "".split_whitespace(),
                },
            }
        }
        pub fn scan<U: Readable>(&mut self) -> U::Output {
            U::read(self.tokenizer.next())
        }
    }

    thread_local! (
        pub static STDIN: RefCell<Scanner<std::io::StdinLock<'static>>> = {
            let stdin = Box::leak(Box::new(io::stdin()));
            RefCell::new(Scanner::new(stdin.lock()))
        }
    );
    #[macro_export]
    macro_rules! scan {
        ([char]) => {
            STDIN.with(|stdin| stdin.borrow_mut().scan::<String>().chars().collect::<Vec<_>>())
        };
        ([u8]) => {
            STDIN.with(|stdin| stdin.borrow_mut().scan::<String>().bytes().collect::<Vec<_>>())
        };
        ([$($t:tt),*; $n:expr]) => {
            (0..$n).map(|_| ($(scan!($t)),*)).collect::<Vec<_>>()
        };
        ($t:ty) => {
            STDIN.with(|stdin| stdin.borrow_mut().scan::<$t>())
        };
        ($($t:ty),+) => {
            ($(scan!($t)),*)
        };
    }

    #[allow(non_camel_case_types)]
    pub struct usize1();
    impl Readable for usize1 {
        type Output = usize;
        fn read(input: &str) -> Self::Output {
            input.parse::<usize>().unwrap() - 1
        }
    }
}

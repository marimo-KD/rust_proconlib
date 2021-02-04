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
    #[inline(always)]
    fn double(&mut self) {
        self.resize(self.len_log + 1, self.len << 1);
    }
    #[inline(always)]
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut map = CuckooHashMap::new();
        map.insert(1, "a");
        map.insert(100, "a");
        map.insert(1000, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&100), Some("a"));
        assert_eq!(map.remove(&1000), Some("a"));
    }
}

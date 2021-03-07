use ahash::random_state::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

type HOPType = u32;
const HOP: usize = std::mem::size_of::<HOPType>();
pub struct HopscotchHashMap<K: Eq + Hash, V> {
    table: Box<[Entry<K, V>]>,
    hop: Box<[HOPType]>,
    hash: RandomState,
    size: usize,
}
impl<K: Eq + Hash, V> HopscotchHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            table: vec![Entry::Vacant()].into_boxed_slice(),
            hop: vec![0].into_boxed_slice(),
            hash: RandomState::with_rand_seeds(),
            size: 0,
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let h = self.hash(&key) as usize;
        if let (Some(_), idx) = self._get(&key) {
            if let Entry::Occupied(KVPair {
                key: _,
                value: ref mut v,
            }) = self.table[(h + idx) & self.capacity()]
            {
                let old = std::mem::replace(v, value);
                return Some(old);
            }
        }
        // we find vacancy.
        loop {
            let mut idx = usize::MAX;
            for i in 0..8 * HOP {
                if let Entry::Vacant() = self.table[(h + i) & self.capacity()] {
                    idx = i;
                    break;
                }
            }
            if idx == usize::MAX {
                self.double();
                continue;
            }
            if idx < HOP {
                self.table[(h + idx) & self.capacity()] = Entry::Occupied(KVPair { key, value });
                self.hop[h] |= 1 << idx;
                return None;
            } else {
                // we have to swap entrys and move vacancy to [h, h+HOP).
                let idx = h + idx;
                while idx >= h + HOP {
                    unimplemented!()
                }
            }
        }
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let h = self.hash(key) as usize;
        let mut hop = self.hop[h];
        while hop != 0 {
            let i = hop.trailing_zeros() as usize;
            if self.table[(h + i) & self.capacity()].eq_key(key) {
                if let Entry::Occupied(KVPair { key: _, value: v }) =
                    self.table[(h + i) & self.capacity()].take()
                {
                    self.size -= 1;
                    self.normalize();
                    return Some(v);
                }
            }
            hop ^= 1 << i;
        }
        None
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self._get(key).0
    }
    fn _get(&self, key: &K) -> (Option<&V>, usize) {
        let h = self.hash(key) as usize;
        let mut hop = self.hop[h];
        while hop != 0 {
            let i = hop.trailing_zeros() as usize;
            if let Entry::Occupied(KVPair {
                key: ref k,
                value: ref v,
            }) = self.table[(h + i) & self.capacity()]
            {
                if k == key {
                    return (Some(v), i);
                }
            }
            hop ^= 1 << i;
        }
        (None, 0)
    }
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.table.len()
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }
    #[inline]
    fn normalize(&mut self) {
        if self.len() * 4 < self.capacity() {
            // if load factor is less than 0.25
            self.resize(self.capacity() >> 1);
        } else if self.len() * 10 <= self.capacity() * 8 {
            // if load factor is larger than 0.8
            self.resize(self.capacity() << 1);
        }
    }
    #[inline(always)]
    fn double(&mut self) {
        self.resize(self.capacity() << 1);
    }
    fn resize(&mut self, new_len: usize) {
        let mut v = Vec::with_capacity(new_len);
        let v = unsafe {
            v.set_len(new_len);
            v.into_boxed_slice()
        };
        let mut old_table = std::mem::replace(&mut self.table, v);
        for i in 0..old_table.len() {
            if let Entry::Occupied(KVPair { key, value }) = old_table[i].take() {
                self.insert(key, value);
            }
        }
    }
    #[inline]
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = self.hash.build_hasher();
        key.hash(&mut hasher);
        hasher.finish() & self.capacity() as u64
    }
}
struct KVPair<K: Eq + Hash, V> {
    key: K,
    value: V,
}
enum Entry<K: Eq + Hash, V> {
    Occupied(KVPair<K, V>),
    Vacant(),
}
impl<K: Eq + Hash, V> Entry<K, V> {
    fn take(&mut self) -> Self {
        std::mem::replace(self, Entry::Vacant())
    }
    fn eq_key(&self, k: &K) -> bool {
        match self {
            Entry::Occupied(KVPair { key, value: _ }) => key == k,
            Entry::Vacant() => false,
        }
    }
}

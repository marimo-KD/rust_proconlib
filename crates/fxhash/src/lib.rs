use std::cell::RefCell;
use std::convert::TryInto;
use std::hash::{BuildHasher, Hasher};
use std::ops::BitXor;
use xorshift::Xorshift256;

#[derive(Debug, Clone)]
pub struct RandomState {
    key: u64,
}
impl RandomState {
    pub fn new() -> Self {
        thread_local!( static KEY: RefCell<Xorshift256> = RefCell::new(Xorshift256::new_with_date()) );
        KEY.with(|key| Self {
            key: key.borrow_mut().gen(),
        })
    }
}
impl BuildHasher for RandomState {
    type Hasher = FxHasher;
    fn build_hasher(&self) -> Self::Hasher {
        FxHasher::new_with_key(self.key)
    }
}

#[derive(Debug)]
pub struct FxHasher {
    hash: u64,
}

const K: u64 = 0x517cc1b727220a95;

impl FxHasher {
    pub fn new() -> Self {
        Self { hash: 0 }
    }
    pub fn new_with_key(hash: u64) -> Self {
        Self { hash }
    }
    #[inline]
    fn add_to_hash(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        let read_usize = |bytes: &[u8]| u64::from_ne_bytes(bytes[..8].try_into().unwrap());

        let mut hash = FxHasher { hash: self.hash };
        while bytes.len() >= 8 {
            hash.add_to_hash(read_usize(bytes));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            hash.add_to_hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as u64);
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            hash.add_to_hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as u64);
            bytes = &bytes[2..];
        }
        if bytes.len() >= 1 {
            hash.add_to_hash(bytes[0] as u64);
        }
        self.hash = hash.hash;
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_check() {
        for _ in 0..10 {
            let a = RandomState::new();
            dbg!(a);
        }
    }
}

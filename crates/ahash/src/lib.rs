use core::hash::{BuildHasherDefault, Hasher};
use std::collections::{HashMap, HashSet};

mod convert;
use convert::*;
pub mod random_state;
use random_state::*;
mod ops;
use ops::*;

pub(crate) const MULTIPLE: u64 = 6364136223846793005;
const ROT: u32 = 23; //17

type AHashBuilder = BuildHasherDefault<AHasher>;
pub type AHashMap<K, V> = HashMap<K, V, AHashBuilder>;
pub type AHashSet<K> = HashSet<K, AHashBuilder>;

#[derive(Debug, Clone)]
pub struct AHasher {
    buffer: u64,
    pad: u64,
    extra_keys: [u64; 2],
}

impl AHasher {
    #[inline]
    pub fn new_with_keys(key1: u128, key2: u128) -> AHasher {
        let pi: [u128; 2] = PI.convert();
        let key1: [u64; 2] = (key1 ^ pi[0]).convert();
        let key2: [u64; 2] = (key2 ^ pi[1]).convert();
        AHasher {
            buffer: key1[0],
            pad: key1[1],
            extra_keys: key2,
        }
    }
    #[inline(always)]
    fn update(&mut self, new_data: u64) {
        self.buffer = folded_multiply(new_data ^ self.buffer, MULTIPLE);
    }

    #[inline(always)]
    fn large_update(&mut self, new_data: u128) {
        let block: [u64; 2] = new_data.convert();
        let combined =
            folded_multiply(block[0] ^ self.extra_keys[0], block[1] ^ self.extra_keys[1]);
        self.buffer = (self.buffer.wrapping_add(self.pad) ^ combined).rotate_left(ROT);
    }
}

impl Hasher for AHasher {
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.update(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.update(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.update(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.update(i as u64);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.large_update(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write(&mut self, input: &[u8]) {
        let mut data = input;
        let length = data.len() as u64;
        self.buffer = self.buffer.wrapping_add(length).wrapping_mul(MULTIPLE);
        if data.len() > 8 {
            if data.len() > 16 {
                let tail = data.read_last_u128();
                self.large_update(tail);
                while data.len() > 16 {
                    let (block, rest) = data.read_u128();
                    self.large_update(block);
                    data = rest;
                }
            } else {
                self.large_update([data.read_u64().0, data.read_last_u64()].convert());
            }
        } else {
            let value = read_small(data);
            self.large_update(value.convert());
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        let rot = (self.buffer & 63) as u32;
        folded_multiply(self.buffer, self.pad).rotate_left(rot)
    }
}

use crate::AHasher;
use core::cell::{Cell, RefCell};
use core::hash::{BuildHasher, Hasher};
use xorshift::Xorshift256;
pub(crate) const PI: [u64; 4] = [
    0x243f_6a88_85a3_08d3,
    0x1319_8a2e_0370_7344,
    0xa409_3822_299f_31d0,
    0x082e_fa98_ec4e_6c89,
];

pub(crate) const PI2: [u64; 4] = [
    0x4528_21e6_38d0_1377,
    0xbe54_66cf_34e9_0c6c,
    0xc0ac_29b7_c97c_50dd,
    0x3f84_d5b5_b547_0917,
];

#[derive(Clone, Debug)]
pub struct RandomState {
    pub(crate) k0: u64,
    pub(crate) k1: u64,
    pub(crate) k2: u64,
    pub(crate) k3: u64,
}

impl RandomState {
    #[inline]
    pub fn new() -> RandomState {
        RandomState::from_keys(PI, PI2)
    }
    #[inline]
    pub const fn with_seeds(k0: u64, k1: u64, k2: u64, k3: u64) -> RandomState {
        RandomState {
            k0: k0 ^ PI2[0],
            k1: k1 ^ PI2[1],
            k2: k2 ^ PI2[2],
            k3: k3 ^ PI2[3],
        }
    }
    pub fn with_rand_seeds() -> RandomState {
        thread_local!(static RNG: RefCell<Xorshift256> = RefCell::new(Xorshift256::new_with_date()));
        RNG.with(|rng| {
            let mut rng = rng.borrow_mut();
            Self::with_seeds(rng.gen(), rng.gen(), rng.gen(), rng.gen())
        })
    }
    fn from_keys(a: [u64; 4], b: [u64; 4]) -> RandomState {
        let [k0, k1, k2, k3] = a;
        let mut hasher = RandomState { k0, k1, k2, k3 }.build_hasher();

        thread_local!(static COUNTER: Cell<usize> = Cell::new(0));
        COUNTER.with(|counter| {
            let count = counter.get();
            hasher.write_usize(count);
            counter.set(count + 1);
        });
        hasher.write_usize(&PI as *const _ as usize);
        let mix = |k: u64| {
            let mut h = hasher.clone();
            h.write_u64(k);
            h.finish()
        };

        RandomState {
            k0: mix(b[0]),
            k1: mix(b[1]),
            k2: mix(b[2]),
            k3: mix(b[3]),
        }
    }
}

impl Default for RandomState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RandomState {
    type Hasher = AHasher;

    #[inline]
    fn build_hasher(&self) -> AHasher {
        AHasher {
            buffer: self.k0,
            pad: self.k1,
            extra_keys: [self.k2, self.k3],
        }
    }
}

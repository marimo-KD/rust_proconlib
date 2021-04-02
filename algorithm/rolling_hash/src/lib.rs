use std::ops::Range;
pub struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

impl RollingHash {
    const MOD: u64 = (1 << 61) - 1;
    pub fn new(s: &[u8]) -> Self {
        let n = s.len();
        let mut hash = vec![0; n + 1];
        let mut pow = vec![1; n + 1];
        let base = 11;
        for i in 0..n {
            hash[i + 1] = Self::mul(hash[i], base) + s[i] as u64;
            pow[i + 1] = Self::mul(pow[i], base);
            if hash[i + 1] >= Self::MOD {
                hash[i + 1] -= Self::MOD;
            }
        }
        Self { hash, pow }
    }
    pub fn hash(&self, range: Range<usize>) -> u64 {
        let Range { start, end } = range;
        let ret = Self::MOD + self.hash[end] - Self::mul(self.hash[start], self.pow[end - start]);
        if ret < Self::MOD {
            ret
        } else {
            ret - Self::MOD
        }
    }
    fn mul(x: u64, y: u64) -> u64 {
        let t: u128 = x as u128 * y as u128;
        let t: u64 = ((t >> 61) + (t & Self::MOD as u128)) as u64;
        if t < Self::MOD {
            t
        } else {
            t - Self::MOD
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rollinghash_test() {
        let s = "abcabcd";
        let h = RollingHash::new(s.as_bytes());
        assert_eq!(h.hash(0..3), h.hash(3..6));
    }
}

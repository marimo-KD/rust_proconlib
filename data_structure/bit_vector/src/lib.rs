use std::iter::{FromIterator, IntoIterator, Iterator};
struct Node {
    bit: u64,
    sum: u32,
}
const LOGWORDSIZE: usize = 6;
const WORDSIZE: usize = 1 << LOGWORDSIZE;
/// NOT succinct, but compact.
pub struct BitVector {
    data: Box<[Node]>,
}

#[inline]
pub const fn access(bit: u64, idx: usize) -> bool {
    bit & (1 << idx) != 0
}
#[inline]
pub const fn rank1(bit: u64, idx: usize) -> usize {
    (bit & ((1 << idx) - 1)).count_ones() as usize
}
fn _select(bit: u64, idx: usize) -> usize {
    const M1: u64 = 0x5555555555555555;
    const M2: u64 = 0x3333333333333333;
    const M4: u64 = 0x0f0f0f0f0f0f0f0f;
    const M8: u64 = 0x00ff00ff00ff00ff;
    let c1 = bit;
    let c2 = c1 - ((c1 >> 1) & M1);
    let c4 = ((c2 >> 2) & M2) + (c2 & M2);
    let c8 = ((c4 >> 4) + c4) & M4;
    let c16 = ((c8 >> 8) + c8) & M8;
    let c32 = (c16 >> 16) + c16;
    let mut i = idx as u64;
    let mut r = 0;
    let mut t = c32 & 0x3f;
    if i >= t {
        r += 32;
        i -= t;
    }
    t = (c16 >> r) & 0x1f;
    if i >= t {
        r += 16;
        i -= t;
    }
    t = (c8 >> r) & 0x0f;
    if i >= t {
        r += 8;
        i -= t;
    }
    t = (c4 >> r) & 0x07;
    if i >= t {
        r += 4;
        i -= t;
    }
    t = (c2 >> r) & 0x03;
    if i >= t {
        r += 2;
        i -= t;
    }
    t = (c1 >> r) & 0x01;
    if i >= t {
        r += 1;
    }
    r
}

#[cfg(target_feature = "bmi2")]
pub fn select(bit: u64, idx: usize) -> usize {
    use std::arch::x86_64::_pdep_u64;
    unsafe { _pdep_u64(1 << idx, bit).trailing_zeros() as usize }
}
#[cfg(not(target_feature = "bmi2"))]
pub fn select(bit: u64, idx: usize) -> usize {
    _select(bit, idx)
}
impl BitVector {
    pub fn new(init: Vec<u64>) -> Self {
        init.into_iter().collect::<Self>()
    }
    pub fn access(&self, idx: usize) -> bool {
        access(
            self.data[idx >> LOGWORDSIZE].bit,
            idx & ((1 << LOGWORDSIZE) - 1),
        )
    }
    pub fn rank0(&self, idx: usize) -> usize {
        idx - self.rank1(idx)
    }
    pub fn rank1(&self, idx: usize) -> usize {
        //! idx is 0-indexed
        let a = &self.data[idx >> LOGWORDSIZE];
        a.sum as usize + rank1(a.bit, idx & ((1 << LOGWORDSIZE) - 1))
    }
    pub fn select0(&self, idx: usize) -> usize {
        //! idx is 0-indexed
        let (mut l, mut r) = (0, self.data.len());
        while r - l > 1 {
            let m = (l + r) >> 2;
            if (m << LOGWORDSIZE) - self.data[m].sum as usize <= idx {
                l = m;
            } else {
                r = m;
            }
        }
        let rest = idx - ((l << LOGWORDSIZE) - self.data[l].sum as usize);
        (l << LOGWORDSIZE) + select(!self.data[l].bit, rest)
    }
    pub fn select1(&self, idx: usize) -> usize {
        //! idx is 0-indexed
        let (mut l, mut r) = (0, self.data.len());
        while r - l > 1 {
            let m = (l + r) / 2;
            if self.data[m].sum as usize <= idx {
                l = m;
            } else {
                r = m;
            }
        }
        let rest = idx - self.data[l].sum as usize;
        (l << LOGWORDSIZE) + select(self.data[l].bit, rest)
    }
}
impl FromIterator<bool> for BitVector {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut v = Vec::new();
        let mut sum = 0;
        'a: loop {
            let mut bit = 0;
            for i in 0..WORDSIZE {
                match iter.next() {
                    Some(x) => {
                        if x {
                            bit |= 1 << i;
                        }
                    }
                    None => {
                        v.push(Node { bit, sum });
                        break 'a;
                    }
                }
            }
            v.push(Node { bit, sum });
            sum += bit.count_ones();
        }
        Self {
            data: v.into_boxed_slice(),
        }
    }
}
impl FromIterator<u64> for BitVector {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut data = Vec::new();
        let mut sum = 0;
        while let Some(bit) = iter.next() {
            data.push(Node { bit, sum });
            sum += bit.count_ones();
        }
        Self {
            data: data.into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bit_vector_test() {
        use std::iter::FromIterator;
        let n = 30;
        let seq: Vec<_> = "000010010011010111000101101001"
            .chars()
            .map(|c| c == '1')
            .collect();
        let bv = BitVector::from_iter(seq.iter().cloned());

        for i in 0..n {
            assert_eq!(bv.access(i), seq[i]);
        }

        // rank
        for i in 0..n {
            assert_eq!(bv.rank0(i), seq[..i].iter().filter(|&&x| !x).count());
            assert_eq!(bv.rank1(i), seq[..i].iter().filter(|&&x| x).count());
        }

        //select
        for i in 0..n {
            if seq[i] {
                assert_eq!(bv.select1(seq[..i].iter().filter(|&&x| x).count()), i);
            }
        }
    }
}

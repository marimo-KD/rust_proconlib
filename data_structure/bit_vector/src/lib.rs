use std::iter::{FromIterator, IntoIterator, Iterator};
struct Node {
    bit: u32,
    sum: u32, // 一つ前までの1の数
}
pub struct BitVector {
    // NOT succinct, but compact.
    data: Box<[Node]>,
}
const WORDSIZE: usize = 32;

fn access(bit: u32, idx: usize) -> bool {
    bit & (1 << idx) != 0
}
fn rank1(bit: u32, idx: usize) -> u32 {
    (bit & ((1 << idx) - 1)).count_ones()
}
#[cfg(target_arch = "x86_64")]
fn select(bit: u32, idx: usize) -> usize {
    if is_x86_feature_detected!("bmi2") {
        use std::arch::x86_64::_pdep_u32;
        unsafe { _pdep_u32(1 << idx, bit).trailing_zeros() as usize }
    } else {
        let mut count = -1;
        for i in 0..WORDSIZE {
            if access(bit, i) {
                count += 1;
            }
            if count == idx as i32 {
                return i;
            }
        }
        return 0;
    }
}
impl BitVector {
    pub fn access(&self, idx: usize) -> bool {
        access(self.data[idx / WORDSIZE].bit, idx % WORDSIZE)
    }
    pub fn rank0(&self, idx: usize) -> u32 {
        idx as u32 - self.rank1(idx)
    }
    pub fn rank1(&self, idx: usize) -> u32 {
        // idx桁まで
        // つまり[0, idx)の1の数
        let a = &self.data[idx / WORDSIZE];
        a.sum + rank1(a.bit, idx % WORDSIZE)
    }
    pub fn select1(&self, idx: usize) -> usize {
        // idx is 0-indexed
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
        l * WORDSIZE + select(self.data[l].bit, rest)
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
            assert_eq!(bv.rank0(i), seq[..i].iter().filter(|&&x| !x).count() as u32);
            assert_eq!(bv.rank1(i), seq[..i].iter().filter(|&&x| x).count() as u32);
        }

        //select
        for i in 0..n {
            if seq[i] {
                assert_eq!(bv.select1(seq[..i].iter().filter(|&&x| x).count()), i);
            }
        }
    }
}

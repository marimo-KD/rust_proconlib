use bit_vector::{access, BitVector};
use std::ops::Range;

/// WaveletMatrix です。
/// u64が入りますが、圧縮した方が良い気がします。
pub struct WaveletMatrix {
    /// Box<[(0の個数、ビット列)]です。
    /// 上位bitから入っています。気をつけましょう。
    data: Box<[(usize, BitVector)]>,
}

impl WaveletMatrix {
    pub fn new(bitlen: usize, seq: Vec<u64>) -> Self {
        //! bitlen: ceil(log2|alphabet|)ですね、つまり扱う対象が何bitで区別できるかです。
        //! set: 元データです。u64をください。
        let len = seq.len();
        let mut data = Vec::with_capacity(bitlen);
        let mut pre = seq;
        let mut nxt = Vec::with_capacity(len);
        for l in (0..bitlen).rev() {
            // 上位bitからですからね。
            let bv = pre.iter().map(|&x| access(x, l)).collect::<BitVector>();
            data.push((bv.rank0(len), bv));
            let zero = pre.iter().filter(|&&x| !access(x, l)).cloned();
            let one = pre.iter().filter(|&&x| access(x, l)).cloned();
            nxt.splice(.., zero.chain(one));
            std::mem::swap(&mut pre, &mut nxt);
        }
        Self {
            data: data.into_boxed_slice(),
        }
    }
    pub fn access(&self, mut idx: usize) -> u64 {
        let mut ret: u64 = 0;
        for (l, &(zero, ref bv)) in (0..self.data.len()).rev().zip(self.data.iter()) {
            if !bv.access(idx) {
                idx = bv.rank0(idx);
            } else {
                ret |= 1 << l;
                idx = zero + bv.rank1(idx);
            }
        }
        ret
    }

    pub fn rank(&self, value: u64, mut range: Range<usize>) -> usize {
        for (l, &(zero, ref bv)) in (0..self.data.len()).rev().zip(self.data.iter()) {
            if !access(value, l) {
                range.start = bv.rank0(range.start);
                range.end = bv.rank0(range.end);
            } else {
                range.start = zero + bv.rank1(range.start);
                range.end = zero + bv.rank1(range.end);
            }
        }
        range.end - range.start
    }

    pub fn select(&self, value: u64, k: usize) -> usize {
        //! Return: the index of k-th value
        // まず一番下(整列されたやつ)で、valueが一番最初に現れるのがどこか調べる
        // rankとだいたい同じ
        let mut idx = 0;
        for (l, &(zero, ref bv)) in (0..self.data.len()).rev().zip(self.data.iter()) {
            if !access(value, l) {
                idx = bv.rank0(idx);
            } else {
                idx = zero + bv.rank1(idx);
            }
        }
        idx += k;
        // ここから戻ってくる
        for &(zero, ref bv) in self.data.iter().rev() {
            // valueのbitで場合分けなのですが、場所が分かっているので、zeroとの比較でよいです。
            if idx < zero {
                idx = bv.select0(idx);
            } else {
                idx = bv.select1(idx - zero);
            }
        }
        idx
    }

    pub fn quantile(&self, mut range: Range<usize>, mut k: usize) -> u64 {
        //! Return: k-th min value in range
        //! Attention: k is 0-indexed.
        let mut ret = 0;
        for (l, &(zero, ref bv)) in (0..self.data.len()).rev().zip(self.data.iter()) {
            let z = bv.rank0(range.end) - bv.rank0(range.start);
            if z > k {
                // 求めるものの上からlbit目は0です。
                range.start = bv.rank0(range.start);
                range.end = bv.rank0(range.end);
            } else {
                k -= z;
                ret |= 1 << l;
                range.start = zero + bv.rank1(range.start);
                range.end = zero + bv.rank1(range.end);
            }
        }
        ret
    }

    fn _count(&self, mut range: Range<usize>, a: u64) -> usize {
        //! Return: the number of value∈[0, a) in range
        let mut ret = 0;
        for (l, &(zero, ref bv)) in (0..self.data.len()).rev().zip(self.data.iter()) {
            if !access(a, l) {
                range.start = bv.rank0(range.start);
                range.end = bv.rank0(range.end);
            } else {
                ret += bv.rank0(range.end) - bv.rank0(range.start);
                // aのl bit目は1ですから、range内でl bit目が0のものはa未満です。
                range.start = zero + bv.rank1(range.start);
                range.end = zero + bv.rank1(range.end);
            }
        }
        ret
    }
    pub fn rangefreq(&self, range: Range<usize>, val: Range<u64>) -> usize {
        self._count(range.clone(), val.end) - self._count(range, val.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_wavelet_matrix() {
        let n = 1 << 5;
        let bitlen = 6;
        let s = 1 << bitlen;
        let q = 1 << 10;
        let mut rng = rand::thread_rng();
        let seq = (0..n).map(|_| rng.gen_range(0, s)).collect::<Vec<_>>();
        let wm = WaveletMatrix::new(bitlen, seq.clone());

        dbg!(seq.clone());
        // access
        for i in 0..n {
            assert_eq!(wm.access(i), seq[i]);
        }

        // rank
        for _ in 0..q {
            let val = rng.gen_range(0, s);
            let range = n / 3..n / 2;
            assert_eq!(
                wm.rank(val, range.clone()),
                seq[range].iter().filter(|&&x| x == val).count()
            );
        }

        // select
        for i in 0..n {
            let val = seq[i];
            let tgt = seq[..i].iter().filter(|&&x| x == val).count();
            assert_eq!(wm.select(val, tgt), i, "select:{}, {}", val, tgt);
        }
    }
}

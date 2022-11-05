pub struct Factorizer {
    sieve: Vec<u64>,
}
impl Factorizer {
    fn new(sieve: Vec<u64>) -> Self {
        Self { sieve }
    }
    pub fn factor(&self, mut n: u64) -> Vec<u64> {
        let mut ret = Vec::new();
        while n > 1 {
            ret.push(self.sieve[n as usize]);
            n /= self.sieve[n as usize];
        }
        ret
    }
}
pub fn sieve(n: u64) -> Factorizer {
    //! O(nloglogn)
    //! make sieve for [0, n)
    let mut ret = vec![0u64; n as usize];
    for i in (2..n).take_while(|i| i*i < n) {
        if ret[i as usize] > 0 {
            continue;
        }
        ret[i as usize] = i;
        let mut j = i*i;
        while j < n {
            if ret[j as usize] == 0 {
                ret[j as usize] = i;
            }
            j += i;
        }
    }
    for i in 0..(n as usize) {
        if ret[i] == 0 {
            ret[i] = i as u64;
        }
    }
    Factorizer::new(ret)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sieve() {
        let s = sieve(100);
        println!("{:?}", s.factor(50));
        println!("{:?}", s.factor(97));
    }
}

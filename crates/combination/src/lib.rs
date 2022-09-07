use static_modint::*;
pub struct CombinationCalculator<M: Mod> {
    fac: Box<[Modint<M>]>,
    facinv: Box<[Modint<M>]>,
}
impl<M: Mod> CombinationCalculator<M> {
    pub fn new(max: usize) -> Self {
        let max = max + 1;
        let mut fac = vec![Modint::new(1); max].into_boxed_slice();
        let mut facinv = vec![Modint::new(1); max].into_boxed_slice();
        for i in 2..max {
            fac[i] = fac[i - 1] * i;
        }
        facinv[max - 1] = fac[max - 1].inv();
        for i in (0..max - 1).rev() {
            facinv[i] = facinv[i + 1] * (i + 1);
        }
        Self { fac, facinv }
    }
    pub fn calc(&self, n: usize, k: usize) -> Modint<M> {
        if n < k {
            Modint::new(0)
        } else {
            self.fac[n] * self.facinv[k] * self.facinv[n - k]
        }
    }
}

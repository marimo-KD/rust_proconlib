use cargo_snippet::snippet;
use super::static_modint::static_modint;
#[snippet]
#[snippet(include = "static_modint")]
mod combination {
    use super::static_modint;
    pub struct CombinationCalculator<M: static_modint::Mod> {
        fac: Vec<static_modint::Modint<M>>,
        facinv: Vec<static_modint::Modint<M>>,
    }
    impl<M: static_modint::Mod> CombinationCalculator<M> {
        pub fn new(max: usize) -> Self {
            let max = max + 1;
            let mut fac = vec![static_modint::Modint::new(1); max];
            let mut facinv = vec![static_modint::Modint::new(1); max];
            for i in 2..max {
                fac[i] = fac[i - 1] * i;
            }
            facinv[max - 1] = fac[max - 1].inv();
            for i in (0..max - 1).rev() {
                facinv[i] = facinv[i + 1] * (i + 1);
            }
            Self { fac, facinv }
        }
        pub fn calc(&self, n: usize, k: usize) -> static_modint::Modint<M> {
            if n < k {
                return static_modint::Modint::new(0);
            }
            self.fac[n] * self.facinv[k] * self.facinv[n - k]
        }
    }
}

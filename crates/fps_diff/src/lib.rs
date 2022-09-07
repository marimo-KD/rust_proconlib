use algebra::Ring;
use fps::FormalPowerSeries;
pub trait FpsDiff {
    fn diff(self) -> Self;
}
impl<T: Ring + Copy + From<usize>> FpsDiff for FormalPowerSeries<T> {
    fn diff(mut self) -> Self {
        let n = self.len();
        let v = self.leak_mut();
        for i in 1..n {
            v[i - 1] =  v[i] * i.into();
        }
        self
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use algebra::*;
use std::ops::*;
#[derive(Debug, Clone)]
pub struct FormalPowerSeries<T: Ring + Copy> {
    data: Vec<T>,
}
impl<T: Ring + Copy> FormalPowerSeries<T> {
    pub fn new(init: Vec<T>) -> Self {
        let mut ret = Self { data: init };
        ret.remove_trailing_zeros();
        ret
    }
    pub fn leak(&self) -> &Vec<T> {
        &self.data
    }
    fn extend(&mut self, x: &Self) {
        self.data
            .resize(self.data.len().max(x.data.len()), T::zero());
    }
    fn remove_trailing_zeros(&mut self) {
        while *self.data.last().unwrap() == T::zero() {
            self.data.pop();
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl<T: Ring + Copy> FormalPowerSeries<T> {
    pub fn naive_mul(&self, rhs: &Self) -> Self {
        if self.data.is_empty() && rhs.data.is_empty() {
            return Self::zero();
        }
        let mut ret = Self::new(vec![T::zero(); self.len() + rhs.len()]);
        for (i, &x) in self.data.iter().enumerate() {
            for (j, &y) in rhs.data.iter().enumerate() {
                ret.data[i + j] += x * y;
            }
        }
        ret.remove_trailing_zeros();
        ret
    }
}
impl<T: Ring + Copy> PartialEq for FormalPowerSeries<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if self.data.len() != rhs.data.len() {
            return false;
        }
        self.data.iter().zip(rhs.data.iter()).all(|(&x, &y)| x == y)
    }
}
// {{{ Ops
impl<T: Ring + Copy> Add for FormalPowerSeries<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();
        ret.add_assign(rhs);
        ret
    }
}
impl<T: Ring + Copy> AddAssign for FormalPowerSeries<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.extend(&rhs);
        for (i, &v) in rhs.data.iter().enumerate() {
            self.data[i] += v;
        }
        self.remove_trailing_zeros();
    }
}
impl<T: Ring + Copy> Sub for FormalPowerSeries<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();
        ret.sub_assign(rhs);
        ret
    }
}
impl<T: Ring + Copy> SubAssign for FormalPowerSeries<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.extend(&rhs);
        for (i, &v) in rhs.data.iter().enumerate() {
            self.data[i] -= v;
        }
        self.remove_trailing_zeros();
    }
}
// }}}
impl<T: Ring + Copy> Zero for FormalPowerSeries<T> {
    fn zero() -> Self {
        Self { data: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fps_add_test() {
        type FPS = FormalPowerSeries<i32>;
        let a = FPS::new(vec![1, 2, 3]);
        let b = FPS::new(vec![1, 2, 3]);
        let c = a + b;
        assert_eq!(*c.leak(), vec![2, 4, 6]);
    }
}

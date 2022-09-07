use super::*;
use algebra::Ring;
use ntt::{convolution, NTTableMod};
use static_modint::Modint;
use std::ops::*;

impl<T: Ring + Copy> Add for FormalPowerSeries<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}
impl<T: Ring + Copy> AddAssign for FormalPowerSeries<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.extend(&rhs);
        for (i, v) in rhs.into_inner().into_iter().enumerate() {
            self.data[i] += v;
        }
        self.remove_trailing_zeros();
    }
}
impl<T: Ring + Copy> Sub for FormalPowerSeries<T> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}
impl<T: Ring + Copy> SubAssign for FormalPowerSeries<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.extend(&rhs);
        for (i, v) in rhs.into_inner().into_iter().enumerate() {
            self.data[i] -= v;
        }
        self.remove_trailing_zeros();
    }
}
impl<T: Ring + Copy> Mul for FormalPowerSeries<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.convoluter;
        if self.is_empty() && rhs.is_empty() {
            fps!(Vec::new(), a)
        } else {
            let ret = a(self.into_inner(), rhs.into_inner());
            fps!(ret, a)
        }
    }
}
impl<T: Ring + Copy> MulAssign for FormalPowerSeries<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone().mul(rhs);
    }
}
pub fn naive_mul<T: Ring + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut ret = vec![T::zero(); a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            ret[i + j] += x * y;
        }
    }
    ret
}

pub fn fast_mul<M: NTTableMod>(a: Vec<Modint<M>>, b: Vec<Modint<M>>) -> Vec<Modint<M>> {
    convolution(a, b)
}

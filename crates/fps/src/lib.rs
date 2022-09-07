use algebra::Ring;

pub mod ops;
#[derive(Debug, Clone)]
pub struct FormalPowerSeries<T: Ring + Copy> {
    data: Vec<T>,
    convoluter: fn(Vec<T>, Vec<T>) -> Vec<T>,
}
impl<T: Ring + Copy> FormalPowerSeries<T> {
    pub fn leak(&self) -> &Vec<T> {
        &self.data
    }
    pub fn leak_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn new(init: Vec<T>, convoluter: fn(Vec<T>, Vec<T>) -> Vec<T>) -> Self {
        let mut ret = Self { data: init , convoluter};
        ret.remove_trailing_zeros();
        ret
    }
    fn extend(&mut self, x: &Self) {
        self.data.resize(self.len().max(x.len()), T::zero());
    }
    fn remove_trailing_zeros(&mut self) {
        while *self.leak().last().unwrap() == T::zero() {
            self.leak_mut().pop();
        }
    }
}
impl<T: Ring + Copy> PartialEq for FormalPowerSeries<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if self.len() != rhs.len() {
            return false;
        }
        self.leak()
            .iter()
            .zip(rhs.leak().iter())
            .all(|(&x, &y)| x == y)
    }
}

#[macro_export]
macro_rules! fps {
    ($vec:expr) => {
        FormalPowerSeries::new($vec, $crate::ops::naive_mul)
    };
    ($vec:expr, ntt) => {
        FormalPowerSeries::new($vec, $crate::ops::fast_mul)
    };
    ($vec:expr, $conv:ident) => {
        FormalPowerSeries::new($vec, $conv)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fps_add_test() {
        let a = fps!(vec![1, 2, 3]);
        let b = fps!(vec![1, 2, 3]);
        let _c = a + b;
    }
}

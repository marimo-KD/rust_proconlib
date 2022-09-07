use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use crate::{Mod, Modint};
impl<M, T> Add<T> for Modint<M>
where
    M: Mod,
    T: Into<Modint<M>>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self {
        self.add_assign(rhs);
        self
    }
}
impl<M, T> Sub<T> for Modint<M>
where
    M: Mod,
    T: Into<Modint<M>>,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        self.sub_assign(rhs);
        self
    }
}
impl<M, T> Mul<T> for Modint<M>
where
    M: Mod,
    T: Into<Modint<M>>,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        self.mul_assign(rhs);
        self
    }
}
impl<M, T> AddAssign<T> for Modint<M>
where
    M: Mod,
    T: Into<Modint<M>>,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.into().x;
        if self.x >= M::M {
            self.x -= M::M;
        }
    }
}
impl<M, T> SubAssign<T> for Modint<M>
where
    M: Mod,
    T: Into<Modint<M>>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.x < rhs.x {
            self.x += M::M;
        }
        self.x -= rhs.x;
    }
}
impl<M, T> MulAssign<T> for Modint<M>
where
    M: Mod,
    T: Into<Modint<M>>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.into().x;
        self.x = M::modulo(self.x);
    }
}

impl<M> Neg for Modint<M> where M: Mod {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * Self::new_internal(M::M - 1)
    }
}

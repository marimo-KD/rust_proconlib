use std::ops::*;
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>, R: Into<Point<T>>> Add<R> for Point<T> {
    type Output = Self;
    fn add(self, other: R) -> Self {
        let other = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Sub<Output = T>, R: Into<Point<T>>> Sub<R> for Point<T> {
    type Output = Self;
    fn sub(self, other: R) -> Self {
        let other = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl<T: Div<Output = T> + Copy> Div<T> for Point<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl<T: AddAssign, R: Into<Point<T>>> AddAssign<R> for Point<T> {
    fn add_assign(&mut self, other: R) {
        let other = other.into();
        self.x += other.x;
        self.y += other.y;
    }
}
impl<T: SubAssign, R: Into<Point<T>>> SubAssign<R> for Point<T> {
    fn sub_assign(&mut self, other: R) {
        let other = other.into();
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl<T: MulAssign + Copy> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}
impl<T: DivAssign + Copy> DivAssign<T> for Point<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}
impl<T> From<(T, T)> for Point<T> {
    fn from(a: (T, T)) -> Self {
        Self { x: a.0, y: a.1 }
    }
}
pub trait Abs {
    fn abs(self) -> Self;
}
impl<T: Default> Point<T> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Into<f64> + Abs + Ord + Copy>
    Point<T>
{
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }
    pub fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y).into().sqrt()
    }
    pub fn l1norm(self) -> T {
        self.x.abs() + self.y.abs()
    }
    pub fn linfnorm(self) -> T {
        self.x.abs().max(self.y.abs())
    }
}

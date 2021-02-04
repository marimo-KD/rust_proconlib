use std::ops::*;
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
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
    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
    #[inline]
    pub fn cross(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }
    #[inline]
    pub fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y).into().sqrt()
    }
    #[inline]
    pub fn l1norm(self) -> T {
        self.x.abs() + self.y.abs()
    }
    #[inline]
    pub fn linfnorm(self) -> T {
        self.x.abs().max(self.y.abs())
    }
}

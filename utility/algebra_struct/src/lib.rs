use algebra::*;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AddMonoid(pub i64);
impl Semigroup for AddMonoid {
    fn op(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl Monoid for AddMonoid {
    fn identity() -> Self {
        Self(0)
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MaxMonoid(pub i64);
impl Semigroup for MaxMonoid {
    fn op(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }
}
impl Monoid for MaxMonoid {
    fn identity() -> Self {
        Self(0)
    }
}

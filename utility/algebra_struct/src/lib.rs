use algebra::*;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AddMonoid(pub i64);
impl Magma for AddMonoid {
    fn op(lhs: Self, rhs: Self) -> Self {
        Self(lhs.0 + rhs.0)
    }
}
impl Semigroup for AddMonoid {}
impl Identity for AddMonoid {
    fn identity() -> Self {
        Self(0)
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MaxMonoid(pub i64);
impl Magma for MaxMonoid {
    fn op(lhs: Self, rhs: Self) -> Self {
        Self(lhs.0.max(rhs.0))
    }
}
impl Semigroup for MaxMonoid {}
impl Identity for MaxMonoid {
    fn identity() -> Self {
        Self(0)
    }
}

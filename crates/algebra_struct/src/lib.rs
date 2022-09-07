use algebra::*;
def_monoid! {
    derive(Copy),
    pub struct AddMonoid(pub i64),
    AddMonoid(0),
    fn op(lhs: AddMonoid, rhs: AddMonoid) -> AddMonoid {
        AddMonoid(lhs.0 + rhs.0)
    }
}
def_monoid! {
    derive(Copy),
    pub struct MaxMonoid(pub i64),
    MaxMonoid(0),
    fn op(lhs: MaxMonoid, rhs: MaxMonoid) -> MaxMonoid {
        MaxMonoid(lhs.0.max(rhs.0))
    }
}


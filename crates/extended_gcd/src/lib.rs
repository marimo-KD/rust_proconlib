pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    //! Return : (g, x, y) s.t. a*x+b*y = g = gcd(a, b)
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extgcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

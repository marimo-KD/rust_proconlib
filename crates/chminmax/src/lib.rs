#[macro_export]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a: expr, $($rem:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rem),+))
    }};
}

#[macro_export]
macro_rules! chmin {
    ($tgt: expr, $($cmp: expr),+ $(,)*) => {{
        let m = $crate::min!($($cmp),+);
        if $tgt > m {
            $tgt = m;
            true
        } else {
            false
        }
    }};
}

#[macro_export]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a: expr, $($rem:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rem),+))
    }};
}

#[macro_export]
macro_rules! chmax {
    ($tgt: expr, $($cmp: expr),+ $(,)*) => {{
        let m = $crate::max!($($cmp),+);
        if $tgt < m {
            $tgt = m;
            true
        } else {
            false
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chmin_test() {
        let mut a = 10;
        chmin!(a, 1, 1,);
    }
}

use crate::{Mod, Modint};
use std::{num::ParseIntError, str::FromStr};
macro_rules! impl_from_u {
    () => {};
    ($t:ty, $($rest:ty),*) => {
        impl_from_u!($t);
        impl_from_u!($($rest),*);
    };
    ($t:ty) => {
        impl<M> From<$t> for Modint<M>
        where
            M: Mod,
        {
            fn from(x: $t) -> Self {
                Self::new(x as u64)
            }
        }
    };
}
macro_rules! impl_from_i {
    () => {};
    ($t:ty, $($rest:ty),*) => {
        impl_from_i!($t);
        impl_from_i!($($rest),*);
    };
    ($t:ty) => {
        impl<M> From<$t> for Modint<M>
        where
            M: Mod,
        {
            fn from(x: $t) -> Self {
                if x >= 0 {
                    Self::new(x as u64)
                } else {
                    -Self::new((-x) as u64) 
                }
            }
        }
    };
}
impl_from_u!(u8, u16, u32, u64, usize);
impl_from_i!(i8, i16, i32, i64, isize);

impl<M> FromStr for Modint<M>
where
    M: Mod,
{
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<u64>()?;
        Ok(Self::new(x))
    }
}

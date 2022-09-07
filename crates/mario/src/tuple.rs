use crate::MarIo;
use crate::token::Token;
use std::io::BufRead;
use std::marker::phantomData;
pub trait Parser {
    type Output;
    fn parse<I: BufRead>(io: &mut MarIo<I>) -> Self::Output;
}

pub struct Leaf<T>(PhantomData<T>);
impl<T: Token> Parser for Leaf<T> {
    type Output = T::Output;
    fn parse<I: BufRead>(io: &mut MarIo<I>) -> Self::Output {
        io.parse::<T>()
    }
}

pub trait RawTuple{
    type LeafTuple;
    fn leaf_tuple() -> Self::LeafTuple;
}
pub struct Tuple<T>(T);

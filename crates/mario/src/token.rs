use std::str::FromStr;
pub trait Token {
    type Output;
    fn parse(s: &str) -> Self::Output;
}

impl<T: FromStr> Token for T {
    type Output = T;
    fn parse(s: &str) -> Self::Output {
        s.parse::<T>().unwrap_or_else(|_| panic!("Parse Error"))
    }
}

#[allow(non_camel_case_types)]
pub struct usize1();
impl Token for usize1 {
    type Output = usize;
    fn parse(s: &str) -> Self::Output {
        let i = s.parse::<usize>().unwrap_or_else(|_| panic!("Parse Error"));
        i.checked_sub(1).expect("usizeから1を引こうとしましたが0だったみたいです。")
    }
}

#[allow(non_camel_case_types)]
pub struct isize1();
impl Token for isize1 {
    type Output = isize;
    fn parse(s: &str) -> Self::Output {
        let i = s.parse::<isize>().unwrap_or_else(|_| panic!("Parse Error"));
        i.checked_sub(1).expect("isizeから1を引こうとしましたが0だったみたいです。")
    }
}


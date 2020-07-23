use cargo_snippet::snippet;
use std::io::*;
use std::iter;

#[snippet("get_macro")]
macro_rules! get {
    ($($t:tt),*; $n:expr) => {
        {
            let stdin = std::io::stdin();
            ret = std::io::BufRead::lines(stdin.lock()).take($n).map(|line| {
                let line = line.unwrap();
                let mut it = line.split_whitespace();
                _get!(it; $($t),*)
            }).collect::<Vec<_>>()
        }
    };
    ($($t:tt),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let it = line.split_whitespace();
            _get!(it; $($t),*)
        }
    };
}
#[snippet("get_macro")]
macro_rules! _get {
    ($it:ident; [char]) => {
        _get!($it; String).chars().collect::<Vec<_>>()
    };
    ($it:ident; [u8]) => {
        _get!($it; String).bytes().collect::<Vec<_>>()
    };
    ($it:ident; usize1) => {
        $it.next().unwrap().parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1
    };
    ($it:ident; [usize1]) => {
        $it.map(|s| s.parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1).collect::<Vec<_>>()
    };
    ($it:ident; [$t:ty]) => {
        $it.map(|s| s.parse::<$t>().unwrap_or_else(|e| panic!("{}", e))).collect::<Vec<_>>()
    };
    ($it:ident; $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap_or_else(|e| panic!("{}", e))
    };
    ($it:ident; $($t:tt),+) => {
        ($(_get!($it; $t)),*)
    };
}
#[test]
fn get_macro_test() {
    let a = get!([i64]);
    println!("{:?}", a);
}

use std::collections::*;
macro_rules! get {
    ($($t:tt),*; $n:expr) => {
        {
            let stdin = std::io::stdin();
            let ret = std::io::BufRead::lines(stdin.lock()).take($n).map(|line| {
                let line = line.unwrap();
                let mut it = line.split_whitespace();
                _get!(it; $($t),*)
            }).collect::<Vec<_>>();
            ret
        }
    };
    ($($t:tt),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();
            _get!(it; $($t),*)
        }
    };
}
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
fn main() {
    let q = get!(usize);
    let query = get!([i64];q);
    let mut array = BTreeMap::new();
    for i in 0..q {
        if query[i][0] == 0 {
            let (k, v) = (query[i][1] as usize, query[i][2]);
            array.insert(k, v);
        } else {
            let k = query[i][1] as usize;
            let ret = array.entry(k).or_insert(0);
            println!("{}", ret);
        }
    }
}

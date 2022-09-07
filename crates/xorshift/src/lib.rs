pub fn xorshift64() -> u64 {
    static mut X: u64 = 88172645463325252;
    unsafe {
        X = X ^ (X << 13);
        X = X ^ (X >> 7);
        X = X ^ (X << 17);
        X
    }
}

pub struct Xorshift256 {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}
impl Xorshift256 {
    pub fn new() -> Self {
        Self {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }
    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: seed,
        }
    }
    pub fn new_with_date() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        Self::new_with_seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
    pub fn gen(&mut self) -> u64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xorshift256_test() {
        let mut a = Xorshift256::new_with_date();
        for _ in 0..100 {
            println!("{}", a.gen());
        }
    }
}

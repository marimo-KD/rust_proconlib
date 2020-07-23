use cargo_snippet::snippet;
#[snippet("unionfind")]
mod unionfind {
    pub struct Unionfind {
        par: Vec<i32>,
        group_count: usize,
    }
    impl Unionfind {
        pub fn new(n: usize) -> Self {
            Unionfind {
                par: vec![-1; n as usize],
                group_count: n as usize,
            }
        }
        fn find_root(&mut self, mut x: usize) -> usize {
            while self.par[x] >= 0 {
                x = self.par[x] as usize;
            }
            x
        }
        pub fn unite(&mut self, x: usize, y: usize) {
            let mut rx = self.find_root(x);
            let mut ry = self.find_root(y);
            if rx == ry {
                return;
            }
            if self.par[rx] > self.par[ry] {
                std::mem::swap(&mut rx, &mut ry);
            }
            self.par[rx] += self.par[ry];
            self.par[ry] = rx as i32;
            self.group_count -= 1;
        }
        pub fn is_same_group(&mut self, x: usize, y: usize) -> bool {
            self.find_root(x) == self.find_root(y)
        }
        pub fn get_group_size(&mut self, x: usize) -> usize {
            let rx = self.find_root(x);
            (-self.par[rx]) as usize
        }
        pub fn get_num_of_groups(&self) -> usize {
            self.group_count
        }
    }
}

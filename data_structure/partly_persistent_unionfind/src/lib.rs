// ATTENTION: NOT VERIFIED
pub struct PartlyPersistentUnionfind {
    par: Box<[i32]>,
    time: Box<[usize]>,
    size_history: Box<[Vec<(usize, usize)>]>,
    now: usize,
}
impl PartlyPersistentUnionfind {
    pub fn new(n: usize) -> Self {
        PartlyPersistentUnionfind {
            par: vec![-1; n].into_boxed_slice(),
            time: vec![1 << 30; n].into_boxed_slice(),
            size_history: vec![vec![(0, 1)]; n].into_boxed_slice(),
            now: 0,
        }
    }
    fn find_root(&mut self, t: usize, mut x: usize) -> usize {
        assert!(x < self.par.len());
        while self.time[x] <= t {
            x = self.par[x] as usize;
        }
        x
    }
    pub fn is_same_group(&mut self, t: usize, x: usize, y: usize) -> bool {
        self.find_root(t, x) == self.find_root(t, y)
    }
    pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        //! return: Option<(time, new root)>
        let mut x = self.find_root(self.now, x);
        let mut y = self.find_root(self.now, y);
        if x == y {
            return None;
        }
        self.now += 1;
        if self.par[x] > self.par[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[x] += self.par[y];
        self.par[y] = x as i32;
        self.time[y] = self.now;
        self.size_history[x].push((self.now, (-self.par[x]) as usize));
        Some((self.now - 1, x))
    }
    pub fn get_group_size(&mut self, t: usize, x: usize) -> usize {
        let x = self.find_root(t, x);
        let (mut l, mut r) = (0, self.size_history[x].len());
        while r - l > 1 {
            let mid = (l + r) / 2;
            if self.size_history[x][mid].0 <= t {
                l = mid;
            } else {
                r = mid;
            }
        }
        self.size_history[x][l].1
    }
}

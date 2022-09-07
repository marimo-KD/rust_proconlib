// ATTENTION: NOT VERIFIED
use algebra::*;
#[derive(Debug)]
pub struct WeightedUnionfind<M: Abel> {
    par: Box<[i32]>,
    diff_weight: Box<[M]>,
    group_count: usize,
}
impl<G: Abel> WeightedUnionfind<G> {
    pub fn new(n: usize) -> Self {
        WeightedUnionfind {
            par: vec![-1; n].into_boxed_slice(),
            diff_weight: vec![G::identity(); n].into_boxed_slice(),
            group_count: 0,
        }
    }
    fn find_root(&mut self, x: usize) -> usize {
        if self.par[x] < 0 {
            x
        } else {
            let p = self.par[x] as usize;
            let r = self.find_root(p);
            self.diff_weight[x].op_from_right(self.diff_weight[p].clone());
            self.par[x] = r as i32;
            r
        }
    }
    pub fn weight(&mut self, x: usize) -> G {
        self.find_root(x);
        self.diff_weight[x].clone()
    }
    pub fn diff(&mut self, x: usize, y: usize) -> G {
        G::op(self.weight(y), self.weight(x).inv())
    }
    pub fn merge(&mut self, x: usize, y: usize, w: G) -> Option<usize> {
        let mut w = G::op(G::op(w, self.weight(x)), self.weight(y).inv());
        let mut x = self.find_root(x);
        let mut y = self.find_root(y);
        if x == y {
            return None;
        }
        if self.par[x] > self.par[y] {
            std::mem::swap(&mut x, &mut y);
            w = w.inv();
        }
        self.par[x] += self.par[y];
        self.par[y] = x as i32;
        self.diff_weight[y] = w;
        Some(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut uf = WeightedUnionfind::new(10);
        uf.merge(0, 1, 10);
        assert_eq!(uf.diff(0, 1), 10);
    }
}

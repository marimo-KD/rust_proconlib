use super::alge_struct::alge;
use cargo_snippet::snippet;

#[snippet]
#[snippet(include = "alge")]
mod weighted_unionfind {
    // ATTENTION: NOT VERIFIED
    use super::alge;
    #[derive(Debug)]
    pub struct WeightedUnionfind<M: alge::Abel> {
        par: Vec<i32>,
        diff_weight: Vec<M>,
        // 親との差です
        group_count: usize,
    }
    impl<M: alge::Abel> WeightedUnionfind<M> {
        pub fn new(n: usize) -> Self {
            WeightedUnionfind {
                par: vec![-1; n],
                diff_weight: vec![M::identity(); n],
                group_count: 0,
            }
        }
        fn find_root(&mut self, x: usize) -> usize {
            if self.par[x] < 0 {
                x
            } else {
                let r = self.find_root(self.par[x] as usize);
                self.diff_weight[x] = (self.diff_weight[x]).clone().op(self.diff_weight[self.par[x] as usize].clone());
                self.par[x] = r as i32;
                r
            }
        }
        pub fn weight(&mut self, x: usize) -> M {
            self.find_root(x);
            self.diff_weight[x].clone()
        }
        pub fn diff(&mut self, x: usize, y: usize) -> M {
            self.weight(y).op(self.weight(x).inv())
        }
        pub fn merge(&mut self, x: usize, y: usize, w: M) {
            let mut w = w.op(self.weight(x)).op(self.weight(y).inv());
            let mut x = self.find_root(x);
            let mut y = self.find_root(y);
            if x == y {
                return;
            }
            if self.par[x] > self.par[y] {
                std::mem::swap(&mut x, &mut y);
                w = w.inv();
            }
            self.par[x] += self.par[y];
            self.par[y] = x as i32;
            self.diff_weight[y] = w;
        }
    }
}

#[test]
fn test() {
    let mut uf = weighted_unionfind::WeightedUnionfind::new(10);
    uf.merge(0, 1, 10);
    assert_eq!(uf.diff(0, 1), 10);
}

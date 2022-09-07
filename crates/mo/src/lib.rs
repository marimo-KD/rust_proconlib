pub trait State {
    type Answer: std::clone::Clone + std::default::Default;
    fn add_left(&mut self, idx: usize);
    fn add_right(&mut self, idx: usize);
    fn erase_left(&mut self, idx: usize);
    fn erase_right(&mut self, idx: usize);
    fn answer(&mut self, idx: usize) -> Self::Answer;
}
pub struct Mo<T: State> {
    state: T,
    n: usize,
    query: Vec<(usize, usize)>,
}
impl<T: State> Mo<T> {
    pub fn new(state: T, n: usize) -> Self {
        Self {
            state,
            n,
            query: vec![(0, 0); 0],
        }
    }
    pub fn add(&mut self, l: usize, r: usize) {
        self.query.push((l, r));
    }
    pub fn run(&mut self) -> Vec<T::Answer> {
        let q = self.query.len();
        let w = (self.n as f64).sqrt() as usize;
        let mut ret = vec![T::Answer::default(); q];
        let mut order: Vec<_> = (0..q).collect();
        order.sort_by(|&i, &j| {
            if self.query[i].0 / w != self.query[j].0 / w {
                self.query[i].0.cmp(&self.query[j].0)
            } else {
                self.query[i].1.cmp(&self.query[j].1)
            }
        });
        let (mut lb, mut rb) = (0, 0);
        for i in order {
            let (li, ri) = self.query[i];
            while lb > li {
                lb -= 1;
                self.state.add_left(lb);
            }
            while rb < ri {
                self.state.add_right(rb);
                rb += 1;
            }
            while lb < li {
                self.state.erase_left(lb);
                lb += 1;
            }
            while rb > ri {
                rb -= 1;
                self.state.erase_right(rb);
            }
            ret[i] = self.state.answer(i);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
    }
}

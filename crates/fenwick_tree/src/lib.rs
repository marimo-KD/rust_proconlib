pub struct FenwickTree<T> {
    data: Box<[T]>,
}
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::default::Default
            + std::marker::Copy,
    > FenwickTree<T>
{
    pub fn new(n: usize) -> Self {
        let mut ret = Self {
            data: vec![std::default::Default::default(); n + 1].into_boxed_slice(),
        };
        ret.build();
        ret
    }
    pub fn new_with_init(mut data: Vec<T>) -> Self {
        data.insert(0, std::default::Default::default());
        let mut ret = Self {
            data: data.into_boxed_slice(),
        };
        ret.build();
        ret
    }
    fn build(&mut self) {
        for i in 1..self.data.len() as i32 {
            let j = (i + (i & -i)) as usize;
            if j < self.data.len() {
                self.data[j] = self.data[j] + self.data[i as usize];
            }
        }
    }
    pub fn add(&mut self, mut k: usize, x: T) {
        k += 1;
        while k < self.data.len() {
            self.data[k] = self.data[k] + x;
            k += {
                let k = k as i32;
                (k & -k) as usize
            }
        }
    }
    pub fn query0(&mut self, mut k: usize) -> T {
        // return: sum of [0,k)
        let mut ret: T = std::default::Default::default();
        while k > 0 {
            ret = ret + self.data[k];
            k -= {
                let k = k as i32;
                (k & -k) as usize
            };
        }
        ret
    }
    pub fn query(&mut self, l: usize, r: usize) -> T {
        // return: sum of [l, r)
        self.query0(r) - self.query0(l)
    }
    pub fn len(&self) -> usize {
        self.data.len() - 1
    }
}

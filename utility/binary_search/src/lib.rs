use std::cmp::*;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut l = 0;
        let mut r = self.len();
        while r - l > 1 {
            let m = (l + r) >> 1;
            match self[m].cmp(x) {
                Ordering::Less => {
                    l = m;
                }
                _ => {
                    r = m;
                }
            }
        }
        r
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut l = 0;
        let mut r = self.len();
        while r - l > 1 {
            let m = (l + r) >> 1;
            match self[m].cmp(x) {
                Ordering::Greater => {
                    r = m;
                }
                _ => {
                    l = m;
                }
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_search() {
        let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

        assert_eq!(vec.lower_bound(&4), 2);
        assert_eq!(vec.upper_bound(&4), 3);
    }
}

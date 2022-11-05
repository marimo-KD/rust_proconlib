use std::cmp::*;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut l = 0;
        let mut r = self.len();
        while l != r {
            let m = (l + r) >> 1;
            match self[m].cmp(x) {
                Ordering::Less => {
                    l = m + 1;
                }
                _ => {
                    r = m;
                }
            }
        }
        l
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut l = 0;
        let mut r = self.len();
        while l != r {
            let m = (l + r) >> 1;
            match self[m].cmp(x) {
                Ordering::Greater => {
                    r = m;
                }
                _ => {
                    l = m + 1;
                }
            }
        }
        l
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

        let vec = vec![1, 1, 1];
        assert_eq!(vec.lower_bound(&1), 0);
        assert_eq!(vec.upper_bound(&1), 3);
    }
}

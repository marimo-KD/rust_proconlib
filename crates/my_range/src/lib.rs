use std::ops::Range;

pub trait MyRange: Sized {
    fn new(start: usize, end: usize) -> Self;
    fn split(&self, border: usize) -> (Self, Self);
    fn half(&self) -> (Self, Self);
    fn contains_range(&self, x: &Self) -> bool;
}

impl MyRange for Range<usize> {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    fn split(&self, border: usize) -> (Self, Self) {
        assert!(self.contains(&border));
        (Self::new(self.start, border), Self::new(border, self.end))
    }
    fn half(&self) -> (Self, Self) {
        assert!(self.len() != 1);
        let m = (self.start + self.end) >> 1;
        self.split(m)
    }
    fn contains_range(&self, x: &Self) -> bool {
        self.start <= x.start && x.end <= self.end
    }
}

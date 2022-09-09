use std::cell::RefCell;
struct ThunkVec {
    dummy: RefCell<Vec<Option<usize>>>,
}
impl ThunkVec {
    fn lookup<F>(&self, n: usize, f: &F) -> usize 
        where F: Fn(usize, &dyn Fn(usize) -> usize) -> usize
    {
        if let Some(x) = self.dummy.borrow()[n] {
            eprintln!("cache hit: {}", n);
            return x;
        }
        eprintln!("called: {}", n);
        let a = f(n, &|n| self.lookup(n, f));
        self.dummy.borrow_mut()[n] = Some(a);
        a
    }
}

pub fn memoise<F>(f: F) -> impl Fn(usize) -> usize 
    where F: Fn(usize, &dyn Fn(usize) -> usize) -> usize
{
    let a = ThunkVec {dummy: RefCell::new(vec![None; 10])};
    return move |n| {
        a.lookup(n, &f)
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let fac = memoise(|n, f| if n == 0 {1} else {n * f(n - 1)});
        let _ = fac(5);
        let a = fac(5);
        assert_eq!(5*4*3*2*1, a);
    }
}

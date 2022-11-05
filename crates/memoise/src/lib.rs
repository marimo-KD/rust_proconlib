use std::cell::RefCell;
pub trait ThunkContainer {
    fn new() -> Self;
    fn lookup<F>(&self, n: usize, f: &F) -> usize
        where F: Fn(usize, &dyn Fn(usize) -> usize) -> usize;
}
struct ThunkVec {
    memo: RefCell<Vec<Option<usize>>>,
}
impl ThunkContainer for ThunkVec {
    fn new() -> Self {
        ThunkVec {
            memo: RefCell::new(Vec::new())
        }
    }
    fn lookup<F>(&self, n: usize, f: &F) -> usize 
        where F: Fn(usize, &dyn Fn(usize) -> usize) -> usize
    {
        self.stretch(n);
        if let Some(x) = self.memo.borrow()[n] {
            return x;
        }
        let a = f(n, &|n| self.lookup(n, f));
        self.memo.borrow_mut()[n] = Some(a);
        a
    }
}
impl ThunkVec {
    fn stretch(&self, n: usize) {
        let &l = &self.memo.borrow().len();
        if l <= n {
            &self.memo.borrow_mut().resize(n+1, None);
        } 
    }
}

pub fn memoise<T, F>(f: F) -> impl Fn(usize) -> usize 
    where 
        T: ThunkContainer,
        F: Fn(usize, &dyn Fn(usize) -> usize) -> usize
{
    let tc = T::new();
    return move |n| {
        tc.lookup(n, &f)
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let fac = memoise::<ThunkVec, _>(|n, f| if n == 0 {1} else {n * f(n - 1)});
        let _ = fac(5);
        let a = fac(5);
        assert_eq!(5*4*3*2*1, a);
    }
}

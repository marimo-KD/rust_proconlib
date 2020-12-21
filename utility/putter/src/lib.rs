use std::cell::RefCell;
use std::io::*;
use std::thread_local;

thread_local! {
    pub static STDOUT : RefCell<BufWriter<StdoutLock<'static>>>= {
        let out = Box::leak(Box::new(stdout()));
        RefCell::new(BufWriter::new(out.lock()))
    }
}
#[macro_export]
macro_rules! put {
    ($fmt:expr) => {
        STDOUT.with(|stdout| write!(stdout.borrow_mut(), $fmt))
    };
    ($fmt:expr, $($arg:tt),*) => {
        STDOUT.with(|stdout| write!(stdout.borrow_mut(), $fmt, $($arg)*))
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_putter() {
        put!("Hello World\n");
        put!("Hello World, {}\n", "marimo");
    }
}

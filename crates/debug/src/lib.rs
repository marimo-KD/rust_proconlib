#[macro_export]
macro_rules! debug {
    ($($e:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprint!("[{}:]", file!(), line!());
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    }
}

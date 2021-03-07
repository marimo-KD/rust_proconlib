pub(crate) trait Convert<To> {
    fn convert(self) -> To;
}

macro_rules! convert {
    ($a:ty, $b:ty) => {
        impl Convert<$b> for $a {
            #[inline]
            fn convert(self) -> $b {
                unsafe {
                    let mut result: $b = core::mem::zeroed();
                    core::ptr::copy_nonoverlapping(
                        &self as *const $a as *const u8,
                        &mut result as *mut $b as *mut u8,
                        core::mem::size_of::<$b>(),
                    );
                    return result;
                }
            }
        }
        impl Convert<$a> for $b {
            #[inline]
            fn convert(self) -> $a {
                unsafe {
                    let mut result: $a = core::mem::zeroed();
                    core::ptr::copy_nonoverlapping(
                        &self as *const $b as *const u8,
                        &mut result as *mut $a as *mut u8,
                        core::mem::size_of::<$a>(),
                    );
                    return result;
                }
            }
        }
    };
}

convert!([u128; 2], [u64; 4]);
convert!(u128, [u64; 2]);
convert!(u128, [u8; 16]);
convert!(u64, [u8; 8]);
convert!(u32, [u8; 4]);
convert!(u16, [u8; 2]);

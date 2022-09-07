use crate::convert::Convert;
#[inline]
pub(crate) fn folded_multiply(s: u64, by: u64) -> u64 {
    let result = (s as u128).wrapping_mul(by as u128);
    ((result & (!0u128 >> 64)) as u64) ^ ((result >> 64) as u64)
}

#[inline]
pub(crate) fn read_small(data: &[u8]) -> [u64; 2] {
    debug_assert!(data.len() <= 8);
    if data.len() >= 2 {
        if data.len() >= 4 {
            //len 4-8
            [data.read_u32().0 as u64, data.read_last_u32() as u64]
        } else {
            //len 2-3
            [data.read_u16().0 as u64, data[data.len() - 1] as u64]
        }
    } else {
        if data.len() > 0 {
            [data[0] as u64, data[0] as u64]
        } else {
            [0, 0]
        }
    }
}

macro_rules! as_array {
    ($input:expr, $len:expr) => {{
        {
            #[inline(always)]
            fn as_array<T>(slice: &[T]) -> &[T; $len] {
                assert_eq!(slice.len(), $len);
                unsafe { &*(slice.as_ptr() as *const [_; $len]) }
            }
            as_array($input)
        }
    }};
}

pub(crate) trait ReadFromSlice {
    fn read_u16(&self) -> (u16, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);
    fn read_u64(&self) -> (u64, &[u8]);
    fn read_u128(&self) -> (u128, &[u8]);
    fn read_last_u16(&self) -> u16;
    fn read_last_u32(&self) -> u32;
    fn read_last_u64(&self) -> u64;
    fn read_last_u128(&self) -> u128;
}

impl ReadFromSlice for [u8] {
    #[inline(always)]
    fn read_u16(&self) -> (u16, &[u8]) {
        let (value, rest) = self.split_at(2);
        (as_array!(value, 2).convert(), rest)
    }

    #[inline(always)]
    fn read_u32(&self) -> (u32, &[u8]) {
        let (value, rest) = self.split_at(4);
        (as_array!(value, 4).convert(), rest)
    }

    #[inline(always)]
    fn read_u64(&self) -> (u64, &[u8]) {
        let (value, rest) = self.split_at(8);
        (as_array!(value, 8).convert(), rest)
    }

    #[inline(always)]
    fn read_u128(&self) -> (u128, &[u8]) {
        let (value, rest) = self.split_at(16);
        (as_array!(value, 16).convert(), rest)
    }
    #[inline(always)]
    fn read_last_u16(&self) -> u16 {
        let (_, value) = self.split_at(self.len() - 2);
        as_array!(value, 2).convert()
    }

    #[inline(always)]
    fn read_last_u32(&self) -> u32 {
        let (_, value) = self.split_at(self.len() - 4);
        as_array!(value, 4).convert()
    }

    #[inline(always)]
    fn read_last_u64(&self) -> u64 {
        let (_, value) = self.split_at(self.len() - 8);
        as_array!(value, 8).convert()
    }

    #[inline(always)]
    fn read_last_u128(&self) -> u128 {
        let (_, value) = self.split_at(self.len() - 16);
        as_array!(value, 16).convert()
    }
}

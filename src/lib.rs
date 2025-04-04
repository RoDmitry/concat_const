#![no_std]

#[inline]
pub const fn len_sum(to_concat: &[&[u8]]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < to_concat.len() {
        len += to_concat[i].len();
        i += 1;
    }
    len
}

#[inline]
pub const fn concat_bytes<const LEN: usize>(to_concat: &[&[u8]]) -> [u8; LEN] {
    let mut res: [u8; LEN] = [0; LEN];
    let mut shift = 0;
    let mut i = 0;
    while i < to_concat.len() {
        let to_concat_one = to_concat[i];
        let mut j = 0;
        while j < to_concat_one.len() {
            res[j + shift] = to_concat_one[j];
            j += 1;
        }
        shift += j;
        i += 1;
    }
    res
}

#[macro_export]
macro_rules! concat {
    ($($rest:expr),*) => {{
        const TO_CONCAT: &[&[u8]] = &[$($rest.as_bytes()),*];
        const LEN: usize = $crate::len_sum(TO_CONCAT);
        const RES: [u8; LEN] = $crate::concat_bytes(TO_CONCAT);
        unsafe { ::core::str::from_utf8_unchecked(&RES) }
    }};
    ($($rest:expr),*,) => {
        $crate::concat!($($rest),*)
    };
}

#[inline]
pub const fn int_len(mut int: i128) -> usize {
    let mut len = 0;
    if int < 0 {
        len = 1;
    }
    while int != 0 {
        int /= 10;
        len += 1;
    }
    len
}

#[inline]
pub const fn int_to_bytes<const LEN: usize>(mut int: i128) -> [u8; LEN] {
    let mut res: [u8; LEN] = [0; LEN];
    let mut i = LEN - 1;
    if int < 0 {
        res[0] = b'-';
        res[i] = -(int % 10) as u8 | 0x30;
        int /= -10;
        i = i.saturating_sub(1);
    }
    while int > 0 {
        res[i] = (int % 10) as u8 | 0x30;
        int /= 10;
        i = i.saturating_sub(1);
    }

    res
}

pub struct BytesWrapper<const LEN: usize>(pub [u8; LEN]);
impl<const LEN: usize> BytesWrapper<LEN> {
    #[inline]
    pub const fn as_bytes(&'static self) -> &'static [u8] {
        &self.0
    }
}

#[macro_export]
macro_rules! int {
    ($a:expr) => {{
        const LEN: usize = $crate::int_len($a);
        $crate::BytesWrapper::<LEN>($crate::int_to_bytes($a))
    }};
}

#[inline]
pub const fn eq_str(ref left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();

    if left.len() != right.len() {
        return false;
    }

    let mut i = 0;
    while i != left.len() {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }

    true
}

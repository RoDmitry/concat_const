#![no_std]

pub const fn len_sum(to_concat: &[&[u8]]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < to_concat.len() {
        len += to_concat[i].len();
        i += 1;
    }
    len
}

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
        concat!($($rest),*)
    };
}

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

pub struct BytesWrapper<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> BytesWrapper<LEN> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_len() {
        assert_eq!(int_len(-123_i128), 4);
        assert_eq!(int_len(23_i128), 2);
        assert_eq!(int_len(i128::MIN), 40);
    }

    #[test]
    fn test_int_to_bytes() {
        const VAR: [u8; 4] = int_to_bytes(-123_i128);
        assert_eq!(&VAR, b"-123");

        const VAR2: [u8; 2] = int_to_bytes(54_i128);
        assert_eq!(&VAR2, b"54");

        const VAR3: [u8; 40] = int_to_bytes(i128::MIN);
        assert_eq!(&VAR3, b"-170141183460469231731687303715884105728");
    }

    #[test]
    fn test_str() {
        const HELLO: &str = "Hello";

        const GREETING2: &str = concat!(HELLO, "world",); // trailing comma
        const GREETING3: &str = concat!(HELLO, ", ", "world");
        const GREETING4: &str = concat!(HELLO, ", ", "world", ".");
        const GREETING6: &str = concat!("Hello", ", ", "world", ".", " ", "end");
        const GREETING3_2: &str = concat!("Hello", "world", int!(1));
        const GREETING4_1: &str = concat!("Hello", "world", int!(-54), int!(32),);

        assert_eq!(GREETING2, "Helloworld");
        assert_eq!(GREETING3, "Hello, world");
        assert_eq!(GREETING4, "Hello, world.");
        assert_eq!(GREETING6, "Hello, world. end");
        assert_eq!(GREETING3_2, "Helloworld1");
        assert_eq!(GREETING4_1, "Helloworld-5432");
    }

    #[test]
    fn test_bytes() {
        const HELLO: &str = "Hello";
        const HELLO_BYTES: &[u8] = HELLO.as_bytes();

        const TO_CONCAT2: &[&[u8]] = &[HELLO_BYTES, b"world"];
        const LEN2: usize = len_sum(TO_CONCAT2);
        const GREETING2: [u8; LEN2] = concat_bytes(TO_CONCAT2);

        const TO_CONCAT3: &[&[u8]] = &[HELLO_BYTES, b", ", b"world"];
        const LEN3: usize = len_sum(TO_CONCAT3);
        const GREETING3: [u8; LEN3] = concat_bytes(TO_CONCAT3);

        const TO_CONCAT4: &[&[u8]] = &[HELLO_BYTES, b", ", b"world", b"."];
        const LEN4: usize = len_sum(TO_CONCAT4);
        const GREETING4: [u8; LEN4] = concat_bytes(TO_CONCAT4);

        const TO_CONCAT6: &[&[u8]] = &[b"Hello", b", ", b"world", b".", b" ", b"end"];
        const LEN6: usize = len_sum(TO_CONCAT6);
        const GREETING6: [u8; LEN6] = concat_bytes(TO_CONCAT6);

        assert_eq!(&GREETING2, b"Helloworld");
        assert_eq!(&GREETING3, b"Hello, world");
        assert_eq!(&GREETING4, b"Hello, world.");
        assert_eq!(&GREETING6, b"Hello, world. end");

        const GREETING2_STR: &str = unsafe { ::core::str::from_utf8_unchecked(&GREETING2) };
        const GREETING3_STR: &str = unsafe { ::core::str::from_utf8_unchecked(&GREETING3) };
        const GREETING4_STR: &str = unsafe { ::core::str::from_utf8_unchecked(&GREETING4) };
        const GREETING6_STR: &str = unsafe { ::core::str::from_utf8_unchecked(&GREETING6) };
        assert_eq!(GREETING2_STR, "Helloworld");
        assert_eq!(GREETING3_STR, "Hello, world");
        assert_eq!(GREETING4_STR, "Hello, world.");
        assert_eq!(GREETING6_STR, "Hello, world. end");
    }
}

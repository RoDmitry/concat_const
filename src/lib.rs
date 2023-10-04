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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        const HELLO: &str = "Hello";

        const GREETING2: &str = concat!(HELLO, "world",); // trailing comma
        const GREETING3: &str = concat!(HELLO, ", ", "world");
        const GREETING4: &str = concat!(HELLO, ", ", "world", ".");
        const GREETING6: &str = concat!("Hello", ", ", "world", ".", " ", "end");

        assert_eq!(GREETING2, "Helloworld");
        assert_eq!(GREETING3, "Hello, world");
        assert_eq!(GREETING4, "Hello, world.");
        assert_eq!(GREETING6, "Hello, world. end");
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

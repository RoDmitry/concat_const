pub const fn concat<const LEN: usize>(to_concat: &[&[u8]]) -> [u8; LEN] {
    let mut res: [u8; LEN] = [0; LEN];
    let mut i;
    let mut z = 0;
    let mut shift = 0;
    while z < to_concat.len() {
        let to_concat_one = to_concat[z];
        let len = to_concat_one.len();
        i = 0;
        while i < len {
            res[shift + i] = to_concat_one[i];
            i += 1;
        }
        shift += i;
        z += 1;
    }
    res
}

pub const fn len_sum(to_concat: &[&[u8]]) -> usize {
    let mut res = 0;
    let mut i = 0;
    while i < to_concat.len() {
        res += to_concat[i].len();
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn top_level_constants() {
        const HELLO: &str = "Hello";
        const COMMA: &str = ", ";
        const WORLD: &str = "world";
        const DOT: &str = ".";

        const HELLO_BYTES: &[u8] = HELLO.as_bytes();
        const COMMA_BYTES: &[u8] = COMMA.as_bytes();
        const WORLD_BYTES: &[u8] = WORLD.as_bytes();
        const DOT_BYTES: &[u8] = DOT.as_bytes();

        const TO_CONCAT2: &[&[u8]] = &[HELLO_BYTES, WORLD_BYTES];
        const LEN2: usize = crate::len_sum(TO_CONCAT2);
        const GREETING2: [u8; LEN2] = crate::concat(TO_CONCAT2);

        const TO_CONCAT3: &[&[u8]] = &[HELLO_BYTES, COMMA_BYTES, WORLD_BYTES];
        const LEN3: usize = crate::len_sum(TO_CONCAT3);
        const GREETING3: [u8; LEN3] = crate::concat(TO_CONCAT3);

        const TO_CONCAT4: &[&[u8]] = &[HELLO_BYTES, COMMA_BYTES, WORLD_BYTES, DOT_BYTES];
        const LEN4: usize = crate::len_sum(TO_CONCAT4);
        const GREETING4: [u8; LEN4] = crate::concat(TO_CONCAT4);

        assert_eq!(&GREETING2, b"Helloworld");
        assert_eq!(&GREETING3, b"Hello, world");
        assert_eq!(&GREETING4, b"Hello, world.");

        const GREETING2_STR: &str = unsafe { core::str::from_utf8_unchecked(&GREETING2) };
        const GREETING3_STR: &str = unsafe { core::str::from_utf8_unchecked(&GREETING3) };
        const GREETING4_STR: &str = unsafe { core::str::from_utf8_unchecked(&GREETING4) };
        assert_eq!(GREETING2_STR, "Helloworld");
        assert_eq!(GREETING3_STR, "Hello, world");
        assert_eq!(GREETING4_STR, "Hello, world.");
    }
}

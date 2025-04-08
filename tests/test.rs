use concat_const::*;

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
fn test_bytes() {
    const HELLO: &[u8] = b"Hello";
    const NUM: i128 = 1;

    const RES1: &[u8] = concat_const::concat_bytes!(HELLO, b"world",); // trailing comma
    assert_eq!(RES1, b"Helloworld");

    const RES2: &[u8] = concat_const::concat_bytes!(HELLO, b", ", b"world");
    assert_eq!(RES2, b"Hello, world");

    const RES3: &[u8] = concat_const::concat_bytes!(HELLO, b", ", b"world", b".");
    assert_eq!(RES3, b"Hello, world.");

    const RES4: &[u8] = concat_const::concat_bytes!(HELLO, b", ", b"world", b".", b" ", b"end");
    assert_eq!(RES4, b"Hello, world. end");

    const RES5: &[u8] = concat_const::concat_bytes!(HELLO, b"world", int_bytes!(NUM));
    assert_eq!(RES5, b"Helloworld1");

    const RES6: &[u8] =
        concat_const::concat_bytes!(HELLO, b"world", int_bytes!(-54), int_bytes!(NUM),);
    assert_eq!(RES6, b"Helloworld-541");
}

#[test]
fn test_str() {
    const HELLO: &str = "Hello";
    const NUM: i128 = 1;

    const RES1: &str = concat_const::concat!(HELLO, "world",); // trailing comma
    assert_eq!(RES1, "Helloworld");

    const RES2: &str = concat_const::concat!(HELLO, ", ", "world");
    assert_eq!(RES2, "Hello, world");

    const RES3: &str = concat_const::concat!(HELLO, ", ", "world", ".");
    assert_eq!(RES3, "Hello, world.");

    const RES4: &str = concat_const::concat!(HELLO, ", ", "world", ".", " ", "end");
    assert_eq!(RES4, "Hello, world. end");

    const RES5: &str = concat_const::concat!(HELLO, "world", int!(NUM));
    assert_eq!(RES5, "Helloworld1");

    const RES6: &str = concat_const::concat!(HELLO, "world", int!(-54), int!(NUM),);
    assert_eq!(RES6, "Helloworld-541");
}

#[test]
fn test_eq_bytes() {
    const HELLO: &[u8] = b"Hello";
    const WORLD: &[u8] = b"world";

    assert!(eq_bytes(HELLO, HELLO));
    assert!(!eq_bytes(HELLO, WORLD));
    assert!(!eq_bytes(HELLO, b"hello"));
    assert!(!eq_bytes(HELLO, b"Helloo"));
    assert!(!eq_bytes(HELLO, b"Hell"));
    assert!(!eq_bytes(HELLO, b""));
}

#[test]
fn test_eq_str() {
    const HELLO: &str = "Hello";
    const WORLD: &str = "world";

    assert!(eq_str(HELLO, HELLO));
    assert!(!eq_str(HELLO, WORLD));
    assert!(!eq_str(HELLO, "hello"));
    assert!(!eq_str(HELLO, "Helloo"));
    assert!(!eq_str(HELLO, "Hell"));
    assert!(!eq_str(HELLO, ""));
}

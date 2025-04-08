# Rust const `&[u8]` and `&str` concatenation

[![Crate](https://img.shields.io/crates/v/concat_const.svg)](https://crates.io/crates/concat_const)
[![API](https://docs.rs/concat_const/badge.svg)](https://docs.rs/concat_const)

```rust
const NUM: i128 = 1;
// &str
const HELLO: &str = "Hello";
const RES: &str = concat_const::concat!(HELLO, "world", concat_const::int!(NUM));
assert_eq!(RES, "Helloworld1");
// bytes
const HELLO: &[u8] = b"Hello";
const RES: &[u8] = concat_const::concat_bytes!(HELLO, b"world", concat_const::int_bytes!(NUM));
assert_eq!(RES, b"Helloworld1");
```

Look at [the tests](https://github.com/RoDmitry/concat_const/blob/main/tests/test.rs#L24) for more usage examples
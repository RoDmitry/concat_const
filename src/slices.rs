use ::core::mem::MaybeUninit;

/// ```compile_fail
/// static DATA: &[u64] = &[0];
/// const RES: &[u64] = concat_const::concat_slices!([u64]: DATA, &[1]);
/// ```
#[macro_export]
macro_rules! concat_slices {
    ([$T:ty]: $($s:expr),* $(,)?) => {
        $crate::_concat_slices!([$T]: $($s),*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _concat_slices {
    ([$T:ty]:) => {{
        const ARR: [$T; 0] = [];
        &ARR
    }};

    ([$T:ty]: $($s:expr),+) => {{
        $(
            const _: &[$T] = $s; // require constants
        )*
        const LEN: ::core::primitive::usize = $( $s.len() + )* 0;
        const ARR: [$T; LEN] = unsafe { $crate::concat_slices::<LEN, $T>(&[$($s),+]) };
        &ARR
    }};
}

/// # Safety
/// Must be used only with constants, or maybe also statics which
/// contain only plain data (no heap allocations, no drop glue).
/// For example using with a static of `String`s is an UB.
pub const unsafe fn concat_slices<const LEN: usize, T: Unpin + 'static>(
    slices: &'static [&'static [T]],
) -> [T; LEN] {
    #[allow(clippy::uninit_assumed_init)]
    let mut arr: [T; LEN] = unsafe { MaybeUninit::<[T; LEN]>::uninit().assume_init() };
    let mut shift = 0;
    let mut i = 0;
    while i < slices.len() {
        let slice = slices[i];
        let src_ptr = slice.as_ptr();
        let dst_ptr = &mut arr[shift];

        // Copy the bytes
        unsafe { ::core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, slice.len()) };

        shift += slice.len();
        i += 1;
    }
    if shift != LEN {
        panic!("Invalid length");
    }
    arr
}

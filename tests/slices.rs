use concat_const::*;

#[test]
fn test_concat_slices() {
    const DATA1: &[u64] = &[0, 1, 2, 3, 4, u64::MAX, 6];
    const DATA2: &[u64] = &[u64::MAX, 11, 12, 13, 14, 15, 16, 17, u64::MAX];

    const RES: &[u64] = concat_slices!([u64]: DATA1, DATA2);
    assert_eq!(
        RES,
        &[
            0,
            1,
            2,
            3,
            4,
            u64::MAX,
            6,
            u64::MAX,
            11,
            12,
            13,
            14,
            15,
            16,
            17,
            u64::MAX,
        ]
    );
}

#[test]
fn test_concat_slices2() {
    const DATA1: &[u64] = &[u64::MAX, u64::MAX, u64::MAX, u64::MAX];
    const DATA2: &[u64] = &[0, 0, 0, 0, 0, 0, 0];
    const DATA3: &[u64] = &[u64::MAX, u64::MAX];

    const RES: &[u64] = concat_slices!([u64]: DATA1, DATA2, DATA3, &[0, 0, 0]);
    assert_eq!(
        RES,
        &[
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            u64::MAX,
            u64::MAX,
            0,
            0,
            0,
        ]
    );
}

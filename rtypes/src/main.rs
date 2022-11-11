fn main() {
    // thread 'main' panicked at 'attempt to multiply with overflow', src\main.rs:4:9
    // let mut i = 1;
    // loop {
    //     i *= 10;
    // }

    // let mut i: i32 = 1;
    // loop {
    //     // panic: multiplication overflowed (in any build)
    //     i = i.checked_mul(10).expect("multiplication overflowed");
    // }

    // Checked operations

    // The sum of 10 and 20 can be represented as a u8.
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 and 200 cannot.
    assert_eq!(100_u8.checked_add(200), None);

    // Do the addition; panic if it overflows.
    // let sum = x.checked_add(y).unwrap();

    // Oddly, signed division can overflow too, in one particular case.
    // A signed n-bit type can represent -2^n-1, but not 2^n-1.
    assert_eq!((-128_i8).checked_div(-1), None);


    // Wrapped operations


    // The first product can be represented as a u16;
    // the second cannot, so we get 250000 modulo 2^16.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift
    // of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // Saturating operations
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // Overflowing operations
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));

}

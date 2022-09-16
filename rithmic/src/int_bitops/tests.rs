use super::*;

#[test]
fn test_basic() {
    assert_eq!(42_u32.bit_width(), 32);
    assert_eq!(42_u32.bit_length(), 6);
    assert_eq!(u8::mask(5), 0b0001_1111);
    assert_eq!(0b0011_1010_u8.masked(5), 0b0001_1010);
    assert_eq!(0b0011_1010_u8.lsb(), 0b0000_0010);
    assert_eq!(0b0011_1010_u8.msb(), 0b0010_0000);
}

#[test]
fn test_zero_inputs() {
    assert_eq!(0_u8.bit_width(), 8);
    assert_eq!(0_u8.bit_length(), 0);
    assert_eq!(u8::mask(0), 0);
    assert_eq!(0_u8.lsb(), 0);
    assert_eq!(0_u8.msb(), 0);

    assert_eq!(0_u8.iter_ones().next(), None);
    assert_eq!(0_u8.iter_lsb().next(), None);
    assert_eq!(0_u8.iter_msb().next(), None);
    assert_eq!(0_u8.iter_subsets().next(), Some(0));
    assert_eq!(0_u8.iter_add_one(3).next(), Some(1));
}

#[test]
fn test_max_inputs() {
    assert_eq!((!0_u8).bit_width(), 8);
    assert_eq!((!0_u8).bit_length(), 8);
    assert_eq!(u8::mask(8), !0);
    assert_eq!((!0_u8).lsb(), 1);
    assert_eq!((!0_u8).msb(), 1<<7);

    assert_eq!((!0_u8).iter_ones().next(), Some(0));
    assert_eq!((!0_u8).iter_lsb().next(), Some(1));
    assert_eq!((!0_u8).iter_msb().next(), Some(1<<7));
    assert_eq!((!0_u8).iter_subsets().next(), Some(0));
    assert_eq!((!0_u8).iter_add_one(8).next(), None);
}

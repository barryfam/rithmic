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
fn test_iter_ones() {
    let x = 0b11010_u8;
    let u = x.iter_ones().collect::<Vec<usize>>();
    assert_eq!(u, vec![1, 3, 4]);
}

#[test]
fn test_iter_lsb() {
    let x = 0b11010_u8;
    let u = x.iter_lsb().collect::<Vec<u8>>();
    assert_eq!(u, vec![
        0b00010,
        0b01000,
        0b10000,
    ]);
}

#[test]
fn test_iter_msb() {
    let x = 0b11010_u8;
    let u = x.iter_msb().collect::<Vec<u8>>();
    assert_eq!(u, vec![
        0b10000,
        0b01000,
        0b00010,
    ]);
}

#[test]
fn test_iter_subsets() {
    let x = 0b11010_u8;
    let u = x.iter_subsets().collect::<Vec<u8>>();
    assert_eq!(u, vec![
        0b00010,
        0b01000,
        0b01010,
        0b10000,
        0b10010,
        0b11000,
        0b11010,
    ]);
}

#[test]
fn test_iter_add_one() {
    let x = 0b0001_1010_u8;

    let u = x.iter_add_one(5).collect::<Vec<u8>>();
    assert_eq!(u, vec![
        0b0001_1011,
        0b0001_1110,
    ]);

    let u = x.iter_add_one(8).collect::<Vec<u8>>();
    assert_eq!(u, vec![
        0b0001_1011,
        0b0001_1110,
        0b0011_1010,
        0b0101_1010,
        0b1001_1010,
    ]);
}

#[test]
fn test_zero_inputs() {
    assert_eq!(0_u8.bit_width(), 8);
    assert_eq!(0_u8.bit_length(), 0);
    assert_eq!(0_u8.lsb(), 0);

    assert_eq!(0_u8.iter_ones().next(), None);
    assert_eq!(0_u8.iter_lsb().next(), None);
    assert_eq!(0_u8.iter_subsets().next(), None);
    assert_eq!(0_u8.iter_add_one(3).next(), Some(1));
}

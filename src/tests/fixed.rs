use fixed::FixedI32;

use crate::{dprintln, fixed};

#[test]
fn mul() {
    let v1 = FixedI32::from_components(0b0000101000000000, 0b0000100000000000);
    let v2 = FixedI32::from_components(0b0000000000000010, 0b0000000000000000);
    let rs = FixedI32::from_components(0b0001010000000000, 0b0001000000000000);

    assert_eq!(v1 * v2, rs);

    let v1 = FixedI32::from_components(0b0000101000000000, 0b0000100000000000);
    let v2 = FixedI32::from_components(0b0000000000001011, 0b0000001000000000);
    let rs = FixedI32::from_components(0b0110111000010100, 0b0101100000010000);

    assert_eq!(v1 * v2, rs);

    let v1 = FixedI32::from_components(0b0111110000100110, 0b1100000100100101);
    let v2 = FixedI32::from_components(0b0000000000000000, 0b1111111111111000);
    let rs = FixedI32::from_components(0b0111110000100010, 0b1101111111101110);

    assert_eq!(v1 * v2, rs);
}

#[test]
fn div() {
    let v1 = FixedI32::from_components(0b0000000000000001, 0b0000000000000000);
    let v2 = FixedI32::from_components(0b0000000000000111, 0b0000000000000000);
    let rs = FixedI32::from_components(0b0000000000000000, 0b0010010010010010);

    assert_eq!(v1 / v2, rs);

    let v1 = FixedI32::from_components(0b0000001011011100, 0b1000111101011100);
    let v2 = FixedI32::from_components(0b0000000000000000, 0b1100110011001100);
    let rs = FixedI32::from_components(0b0000001110010011, 0b1011011011000110);

    assert_eq!(v1 / v2, rs);
}

#[test]
fn from_str() {
    let c1 = FixedI32::from("1.625");
    let c2 = FixedI32::from_components(0b0000000000000001, 0b1010000000000000);

    assert_eq!(c1, c2);

    let c1 = FixedI32::from("1.2");
    let c2 = FixedI32::from_components(0b0000000000000001, 0b0011001100110011);

    assert_eq!(c1, c2);

    let c1 = FixedI32::from("3921.47215");
    let c2 = FixedI32::from_components(0b0000111101010001, 0b0111100011011110);

    assert_eq!(c1, c2);
}

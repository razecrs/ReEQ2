//! crc32 tests. the 123456789 -> 0xCBF43926 one is the canonical check,
//! if that passes i know the impl matches standard crc-32.

use protocol::crc32;

#[test]
fn canonical_vector() {
    // The standard CRC-32 check value. If this passes, your impl is correct.
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
}

#[test]
fn empty_input() {
    assert_eq!(crc32(b""), 0x0000_0000);
}

#[test]
fn single_byte() {
    assert_eq!(crc32(&[0x00]), 0xD202_EF8D);
}

#[test]
fn ascii_string() {
    assert_eq!(
        crc32(b"The quick brown fox jumps over the lazy dog"),
        0x414F_A339
    );
}

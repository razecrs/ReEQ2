//! Tests for the CRC-32 warm-up task.
//!
//! These start `#[ignore]`d so `main` stays green. When you implement
//! `crc32`, DELETE the `#[ignore]` line on each test and run `cargo test`.

use protocol::crc32;

#[test]
#[ignore = "student task: implement crc32, then delete this line"]
fn canonical_vector() {
    // The standard CRC-32 check value. If this passes, your impl is correct.
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
}

#[test]
#[ignore = "student task: implement crc32, then delete this line"]
fn empty_input() {
    assert_eq!(crc32(b""), 0x0000_0000);
}

#[test]
#[ignore = "student task: implement crc32, then delete this line"]
fn single_byte() {
    assert_eq!(crc32(&[0x00]), 0xD202_EF8D);
}

#[test]
#[ignore = "student task: implement crc32, then delete this line"]
fn ascii_string() {
    assert_eq!(
        crc32(b"The quick brown fox jumps over the lazy dog"),
        0x414F_A339
    );
}

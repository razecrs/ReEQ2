//! CRC-32 (IEEE 802.3) — the warm-up task.
//!
//! EQStream checksums packets with the standard CRC-32 table (the
//! `0x77073096...` polynomial table you'll see everywhere). This is a
//! *published algorithm*, so reimplementing it from spec is 100% clean.
//!
//! ## YOUR TASK
//! Implement [`crc32`] so the tests in `tests/crc.rs` pass.
//!
//! Algorithm (table-driven CRC-32/ISO-HDLC):
//!
//! 1. Start with `crc = 0xFFFF_FFFF`.
//! 2. For each byte `b`, do
//!    `crc = (crc >> 8) ^ TABLE[((crc ^ b as u32) & 0xFF) as usize]`.
//! 3. Return `crc ^ 0xFFFF_FFFF`.
//!
//! The 256-entry `TABLE` can be computed once at startup, or you can build it
//! with a `const` function. Start simple: compute it at runtime in a helper.
//!
//! Reference test vector (this is THE canonical one): `crc32(b"123456789")`
//! must equal `0xCBF4_3926`.

/// Compute the CRC-32 (IEEE) checksum of `data`.
pub fn crc32(data: &[u8]) -> u32 {
    // TODO(student): implement the algorithm in the module docs above.
    // Delete this line and make `tests/crc.rs` pass.
    let _ = data;
    unimplemented!("crc32 is your task — see the module docs")
}

//! CRC-32 (IEEE 802.3).
//!
//! EQStream checksums packets with the standard CRC-32 table (the
//! `0x77073096...` polynomial table you'll see everywhere). This is a
//! *published algorithm*, so reimplementing it from spec is 100% clean.
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

const TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

/// Compute the CRC-32 (IEEE) checksum of `data`.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFF;
    for &b in data {
        crc = (crc >> 8) ^ TABLE[((crc ^ b as u32) & 0xFF) as usize];
    }
    crc ^ 0xFFFF_FFFF
}

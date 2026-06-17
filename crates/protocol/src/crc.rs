//! crc32. every eqstream packet carries one so the client can tell if the
//! bytes got mangled. it's just standard crc-32 (the 0x77073096 table you see
//! everywhere) so writing it myself is totally clean, nothing copied.
//!
//! how it goes:
//! - start crc at 0xFFFF_FFFF
//! - per byte: crc = (crc >> 8) ^ TABLE[(crc ^ b) & 0xFF]
//! - flip it with ^ 0xFFFF_FFFF at the end
//!
//! sanity check: crc32("123456789") == 0xCBF43926. that's THE known answer,
//! if i hit it the impl is right.

// build the lookup table once at compile time so the hot loop is just xors
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

/// crc32 of some bytes.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFF;
    for &b in data {
        crc = (crc >> 8) ^ TABLE[((crc ^ b as u32) & 0xFF) as usize];
    }
    crc ^ 0xFFFF_FFFF
}

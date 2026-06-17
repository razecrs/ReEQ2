//! EQStream wire-protocol primitives.
//!
//! Clean-room: everything here is derived from observed packet behavior and
//! published algorithms, NOT from any GPL source.

pub mod crc;
pub mod opcode;

pub use crc::crc32;
pub use opcode::Opcode;

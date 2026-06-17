//! eqstream wire protocol bits.
//!
//! clean-room — all of this is from watching real packets + public algos,
//! nothing copied off the gpl source.

pub mod crc;
pub mod opcode;

pub use crc::crc32;
pub use opcode::Opcode;

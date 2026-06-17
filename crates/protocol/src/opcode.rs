//! Login-server opcodes — the second task.
//!
//! These 16-bit values identify messages on the wire. Opcode *numbers* are
//! interop facts (not copyrightable), so we use them freely. The values below
//! are confirmed from live packet captures as we reverse the handshake.
//!
//! ## YOUR TASK
//! Make [`Opcode`] round-trip through `u16` so `tests/opcode.rs` passes:
//!
//! * `Opcode::try_from(0x0300u16)` -> `Ok(Opcode::GetLoginInfo)`
//! * `u16::from(Opcode::GetLoginInfo)` -> `0x0300`
//! * an unknown value -> `Err(UnknownOpcode(value))`

/// A login-server message opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Opcode {
    LoginInfo,    // 0x0100
    Login2,       // 0x0200
    GetLoginInfo, // 0x0300
    Disconnect,   // 0x0500
    SessionId,    // 0x0900
    ServerList,   // 0x4600
    SessionKey,   // 0x4700
    Version,      // 0x5900
}

/// Returned when a wire value doesn't map to a known [`Opcode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownOpcode(pub u16);

impl TryFrom<u16> for Opcode {
    type Error = UnknownOpcode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        // TODO(student): map each known value to its variant; everything
        // else returns Err(UnknownOpcode(value)).
        let _ = value;
        unimplemented!("Opcode::try_from is your task — see the module docs")
    }
}

impl From<Opcode> for u16 {
    fn from(op: Opcode) -> u16 {
        // TODO(student): return the wire value for each variant.
        let _ = op;
        unimplemented!("u16::from(Opcode) is your task — see the module docs")
    }
}

//! login opcodes. these u16s say what each message on the wire actually is.
//! the numbers themselves are just interop facts so totally fair to use.
//! i'm confirming each one from my own captures as i pick apart the handshake.
//!
//! Opcode goes both ways with u16 — try_from to read one off the wire
//! (UnknownOpcode if i don't know it yet), u16::from to write it back out.

/// one login message type.
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

/// hit a u16 i don't have an opcode for yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownOpcode(pub u16);

impl TryFrom<u16> for Opcode {
    type Error = UnknownOpcode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0100 => Ok(Opcode::LoginInfo),
            0x0200 => Ok(Opcode::Login2),
            0x0300 => Ok(Opcode::GetLoginInfo),
            0x0500 => Ok(Opcode::Disconnect),
            0x0900 => Ok(Opcode::SessionId),
            0x4600 => Ok(Opcode::ServerList),
            0x4700 => Ok(Opcode::SessionKey),
            0x5900 => Ok(Opcode::Version),
            _ => Err(UnknownOpcode(value)),
        }
    }
}

impl From<Opcode> for u16 {
    fn from(op: Opcode) -> u16 {
        match op {
            Opcode::LoginInfo => 0x0100,
            Opcode::Login2 => 0x0200,
            Opcode::GetLoginInfo => 0x0300,
            Opcode::Disconnect => 0x0500,
            Opcode::SessionId => 0x0900,
            Opcode::ServerList => 0x4600,
            Opcode::SessionKey => 0x4700,
            Opcode::Version => 0x5900,
        }
    }
}

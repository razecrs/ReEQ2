//! Tests for the Opcode round-trip task.
//!
//! Start `#[ignore]`d so `main` stays green. Implement the two conversions,
//! then delete the `#[ignore]` lines.

use protocol::opcode::{Opcode, UnknownOpcode};

#[test]
#[ignore = "student task: implement Opcode conversions, then delete this line"]
fn known_values_parse() {
    assert_eq!(Opcode::try_from(0x0300u16), Ok(Opcode::GetLoginInfo));
    assert_eq!(Opcode::try_from(0x4600u16), Ok(Opcode::ServerList));
    assert_eq!(Opcode::try_from(0x5900u16), Ok(Opcode::Version));
}

#[test]
#[ignore = "student task: implement Opcode conversions, then delete this line"]
fn unknown_value_errors() {
    assert_eq!(Opcode::try_from(0xFFFFu16), Err(UnknownOpcode(0xFFFF)));
}

#[test]
#[ignore = "student task: implement Opcode conversions, then delete this line"]
fn round_trips() {
    for op in [
        Opcode::LoginInfo,
        Opcode::Login2,
        Opcode::GetLoginInfo,
        Opcode::Disconnect,
        Opcode::SessionId,
        Opcode::ServerList,
        Opcode::SessionKey,
        Opcode::Version,
    ] {
        let wire = u16::from(op);
        assert_eq!(
            Opcode::try_from(wire),
            Ok(op),
            "round-trip failed for {op:?}"
        );
    }
}

# eq2-rs

A clean-room reimplementation of the EverQuest II server protocol in Rust.

Goal of milestone 1: **a real EQ2 client connects to this server and reaches a
character/world select.** Login + EQStream transport first, then world/zone.

## Clean-room policy

Everything here is derived from **observed packet behavior** and **published
algorithms** only. We do not copy or translate any third-party source. Opcode
numbers and packet layouts are interoperability facts, established from our own
client captures.

## Layout

```
crates/
  protocol/      EQStream primitives: CRC, opcodes, packet (de)serialization
  login-server/  milestone 1 — the connect
```

## Build & test

```
cargo test --workspace
cargo run -p login-server
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). New contributors: start with the tasks
marked `#[ignore]` in `crates/protocol/tests/` — implement, un-ignore, open a PR.

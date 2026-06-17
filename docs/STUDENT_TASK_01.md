# Task 01 — CRC-32 (your first Rust PR)

This is the VC brief. Read it top to bottom; do the steps in order.

## What you're building

EQStream (the EQ2 network protocol) puts a **checksum** on every packet so the
receiver can tell if bytes got corrupted. That checksum is **CRC-32**, a
standard, published algorithm. You're going to implement it in Rust.

It's a pure function: bytes in, one `u32` out. No networking, no async, nothing
that can break anyone else's code. Perfect first task.

## The loop you'll repeat all project

1. `git checkout -b yourname/crc`
2. Open `crates/protocol/src/crc.rs`. The algorithm is written out in the doc
   comment at the top. Implement the `crc32` function.
3. Open `crates/protocol/tests/crc.rs`. Delete the `#[ignore]` lines.
4. Run `cargo test -p protocol`. Make all four tests pass.
5. `cargo fmt --all` and `cargo clippy --workspace -- -D warnings` (no warnings).
6. Commit, push, open a PR. I review it.

## How you know you're right

The test `canonical_vector` checks `crc32(b"123456789") == 0xCBF43926`. That
`0xCBF43926` is *the* known answer for CRC-32 — every correct implementation on
Earth produces it. If you get it, you nailed it.

## Hints (use only if stuck)

- Build the 256-entry lookup table first. For each `i` in `0..256`, run `i`
  through the polynomial `0xEDB88320` eight times (shift right; XOR the poly
  when the low bit was set).
- Then the main loop is three lines — exactly what's in the `src/crc.rs` docs.
- Type sizes matter in Rust: keep everything `u32`, cast bytes with `as u32`.

## When this is merged

Next task: the `Opcode` enum (`crates/protocol/tests/opcode.rs`) — same loop.
Then we start on real captured login packets.

# Onboarding — how work happens here (read first)

This is the VC brief. Before you get a real task, learn the **loop** every task
follows. We'll use the already-finished `crc32` as the worked example.

## The loop (you'll repeat this all project)

1. `git checkout -b yourname/short-task` — never work on `main`.
2. Read the **module docs** at the top of the `src/` file. For the example,
   open `crates/protocol/src/crc.rs` — the algorithm is written out in plain
   English in the doc comment.
3. Look at the **tests** that define "done": `crates/protocol/tests/crc.rs`.
   Tests are the spec. Your code is right when they pass.
4. Implement until `cargo test -p protocol` is green.
5. `cargo fmt --all` + `cargo clippy --workspace -- -D warnings` (zero warnings).
6. Commit, push the branch, open a PR. CI runs the same checks; I review.

## Study the worked example

`crc32` is done and correct — read it as a model, don't change it:

- `src/crc.rs` — note how the lookup `TABLE` is built once with a `const` block,
  then the main loop is three lines. That "table + tight loop" shape comes back
  constantly in protocol code.
- `tests/crc.rs` — note the **canonical vector** `crc32(b"123456789") ==
  0xCBF43926`. Good tests pin behavior to a value you can verify independently.
- `src/opcode.rs` + `tests/opcode.rs` — a second example: enum <-> `u16`
  round-trip, with an error type for unknown values.

If you can explain *why* each of those tests passes, you're ready.

## Your real first task (coming next)

Parsing the **EQStream session-request packet** off a live capture: bytes in,
a typed struct out. We'll write the test from the *actual captured bytes* (so
it's clean-room and correct), and you'll implement the parser. Same loop as
above — just real wire data instead of a textbook algorithm.

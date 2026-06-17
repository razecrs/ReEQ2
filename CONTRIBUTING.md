# Contributing

## The workflow (read this once)

1. **Never push to `main`.** It's protected — direct pushes and force-pushes are
   blocked. You work on a branch and open a Pull Request.
2. Make a branch:
   ```
   git checkout -b your-name/short-task-name
   ```
3. Do the work. Run the checks locally before you push:
   ```
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace --all-targets
   ```
4. Commit, push the branch, open a PR. CI runs the same three checks. It must be
   green and reviewed before it can merge.

## Your first task

The `protocol` crate has starter tasks as **ignored tests**. Pick one:

- `crates/protocol/tests/crc.rs` — implement `crc32` (warm-up).
- `crates/protocol/tests/opcode.rs` — implement the `Opcode` conversions.

For each: read the module docs in the matching `src/` file, implement it,
**delete the `#[ignore]` lines**, and make `cargo test` pass. That's the whole
loop — turn red into green.

## Rules of thumb

- Small PRs. One task per PR.
- If a test is red, the code is wrong — not the test. Ask before editing tests.
- Clean-room: don't paste code from other emulators. Work from captures/specs.

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

## Getting started

`crates/protocol/src/crc.rs` and `opcode.rs` are small, finished examples of
the loop you'll repeat: read the module docs, implement, make the tests in
`tests/` pass with `cargo test`. Read those two first — they show the shape of
every task. Your assigned task lives in `docs/` (see `STUDENT_TASK_01.md`).

## Rules of thumb

- Small PRs. One task per PR.
- If a test is red, the code is wrong — not the test. Ask before editing tests.
- Clean-room: don't paste code from other emulators. Work from captures/specs.

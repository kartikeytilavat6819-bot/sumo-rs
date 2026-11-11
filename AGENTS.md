# Instructions for porting agents

You are porting part of Eclipse SUMO (C++) to Rust. Faithfulness beats elegance: same behaviour,
same output text, same floating-point results.

- C++ source (read-only): `C:\Users\karti\sumo-src\src`. Rust repo: the git worktree you are in.
- Cargo: `export PATH="$HOME/.cargo/bin:$PATH"` (bash). Edition 2024 (`gen` is a reserved word).
- MEMORY: many agents run at once on a 16 GB machine. Always build with the shared, locked target dir:
  `export CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target CARGO_BUILD_JOBS=8` before any cargo command.
  Cargo serializes builds on that dir, so a build may wait for others; that's expected, don't work around it.
  Prefer `cargo check -p <crate>` while iterating; never run `cargo build --release` or the whole workspace's tests.
- One Rust file per C++ file pair (`Foo.h` + `Foo.cpp` -> `foo.rs`, snake_case), first line
  `//! Port of <path>/Foo.{h,cpp}`. Keep the C++ order of functions so reviewers can diff side by side.
- Use what is already ported; don't re-implement it:
  - errors: `utils::common::util_exceptions::ProcessError`
  - messages: `utils::common::msg_handler::{write_warning, write_error, write_message}`
  - options: `utils::options::options_cont::{get_string, get_float, get_int, get_bool, ...}`
  - time: `utils::common::time::{SUMOTime, string2time, time2string, ...}`
  - random: `utils::common::rand_helper::SumoRNG`
  - number formatting: `utils::common::to_string::f64_to_string_prec`
  - geometry: `utils::geom::{position::Position, position_vector::PositionVector, boundary::Boundary}`
  - XML reading/writing: `utils::xml::*`, `utils::iodevices::output_device::OutputDevice`
- Keep floating-point expressions in the same order as C++. No `mul_add`, no "simplifying" formulas.
- Keep error/warning message texts identical to C++.
- C++ `std::sort` on ties, `std::set<T*>` and `unordered_map` iteration order can change results:
  add a `// determinism:` comment wherever you hit one.
- Forbidden: `todo!()`, `unimplemented!()`, empty bodies standing in for real logic, comments like
  "logic elided", deleting or weakening tests. If something truly cannot be ported yet (needs an
  unported module), write `// shortcut: <what is missing>, <when to upgrade>` and keep the rest working.
- Add `#[cfg(test)]` unit tests for non-trivial logic, using values derived from the C++ code.
- Before finishing: `cargo build` and `cargo test -p <crate>` must pass for the whole workspace.
- Commit in your worktree (don't push). Commit message: `Port <module>`, last line
  the Co-Authored-By line your harness gives you (naming the model you run on). Don't edit PORTING.md.
- Final report (under 150 words): files ported, every `shortcut:` you left and why, commit hash, branch.

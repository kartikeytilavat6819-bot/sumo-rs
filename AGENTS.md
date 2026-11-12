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

## Filling the generated skeleton (crates/sumo_core)

- Your batch is a list of files in `tools/batches/batch_NN.txt`. Edit ONLY those files.
- Each `todo!("port X::y")` has the C++ body pasted as `//` comments right above it. Replace the
  `todo!()` with a faithful translation; keep the C++ comment block above it (reviewers diff against it).
- Types: C++ pointers are `Ptr<T>` (utils::common::ptr) — copy it freely, call methods through it,
  use `.m()` for mutable access, `.is_null()` for null checks. `Unmapped` is a placeholder for a C++ type
  the generator couldn't map: replace it in YOUR files with the right Rust type when you need it.
- Calling other classes is fine even if their bodies are still `todo!()` — the signatures exist.
- Don't change `pub` signatures unless you must; if you do, list every changed signature in your report.
- C++ static members are listed in a comment block under each struct: port them as module-level
  `thread_local!`/`static` items in the same file.
- Virtual methods: implement them on the struct as generated; overriding behaviour in subclasses
  is noted with `[virtual]` in the doc line. Keep the method on the subclass too.
- Commit after every 2-3 files (`git add -A && git commit -m "Fill <files>"`), so progress survives if
  you run out of budget. It's fine not to finish the whole batch — report what's left.
- Check with `cargo check -p sumo_core` (shared target dir, see MEMORY above).

### Don't stop early (read this twice)

- Your goal is to replace EVERY `todo!()` in your batch. Stopping with most of them left is a failed run.
- "Depends on unported code" is NOT a reason to skip a method:
  - Other classes' methods exist with real signatures; call them even if their bodies are `todo!()`.
  - `Unmapped` fields/params in YOUR files: replace them with the right Rust type yourself (e.g. a
    C++ `std::vector<NBEdge*>` typedef -> `Vec<Ptr<NBEdge>>`, `FileBucket*` -> `Ptr<FileBucket>`).
  - `Unmapped` in OTHER files: work around locally (convert, or add a `// shortcut:` and keep going).
- Start with the files with the most `todo!()`s; work method by method; commit every 2-3 files.
- Only stop when no `todo!()` is left in your files or you are truly out of budget.

### Reading skeleton files without running out of context

- Skeleton files are huge (every method carries its C++ as comments; ms_vehicle.rs is enormous).
  NEVER Read a whole skeleton file. Instead:
  1. `grep -n 'todo!(' <file>` to list the methods left, with line numbers.
  2. Read ~80 lines around one `todo!()` (offset/limit), port it, Edit it, move to the next.
  3. To learn a struct's fields, grep `pub struct Name` and read just that block.
- Commit after every ~20 methods, not only per file.

### Compiling (UPDATED - overrides the MEMORY note above)

- Writer agents: do NOT run cargo at all. The shared build lock makes 20 agents wait for an hour.
  Translate carefully, commit, finish. The lead compiles at merge time and sends errors to a fixer.
- Fixer agents (told explicitly in their prompt): use your own target dir as instructed there.

### Downcasts (C++ static_cast / dynamic_cast to a subclass)
All structs whose first field is `pub base:` already carry `#[repr(C)]` (lead sweep); new derived structs must too.


- Add `#[repr(C)]` to the derived struct in your file (its `base` field must stay first), then
  `let v: Ptr<MSVehicle> = sumo_vehicle_ptr.cast::<MSVehicle>();`. Same for upcasts.
- dynamic_cast's runtime type check is not available: only cast where the C++ logic guarantees the
  type (e.g. after checking a type flag / id prefix); otherwise leave a `// shortcut:` note.

### MSGlobals

- C++ `MSGlobals::gComputeLC` -> `MSGlobals::compute_lc()` (setter `MSGlobals::set_compute_lc(v)`): every
  MSGlobals static is ported (microsim/ms_globals.rs) as snake_case without the `g`. Don't stub them anymore.
- Passing a derived object where the C++ expects a base reference: pass `&ptr` and let Deref coerce
  (MSVehicle -> MSBaseVehicle -> SUMOVehicle are embedded bases).

### Constructors that register `this`

- If the C++ constructor (or an init() it calls) stores `this` anywhere (registers with a lane/node/edge,
  adds itself to a container, sets a back-pointer), the Rust `new` MUST return `Box<Self>`: build the
  struct, `let mut b = Box::new(..)`, then do the registering steps with `Ptr::from_box(&b)` / `Ptr::from_mut(&mut *b)`,
  and return `b`. Returning `Self` by value makes those stored pointers dangle when the caller moves it.
- Callers keep the Box alive for the object's lifetime (store it in the owning container, or `Box::leak`
  with a `// shortcut:` note where C++ ownership is unclear).

### Ptr method-name clash

- `Ptr<T>` has its own `get()`, `m()`, `cast()`, `is_null()`. If the pointee has a method with the same
  name (e.g. `RandomDistributor::get(rng)`), call it through an explicit deref: `(*p).get(None)`.

# GEOS and gRPC crate decision (juPedSim models)

Scope: the `todo!()` bodies in `msp_model_ju_ped_sim.rs` and `msp_model_ju_ped_sim_legacy.rs` need
GEOS (geometry) and, for the gRPC model, protobuf/tonic. This note picks the crates. Nothing is added to
`Cargo.toml` yet, so `cargo check -p sumo_core` still builds without native libraries.

## Crates (checked with `cargo search`, 2026-11-13)

| Need | Crate | Version | Notes |
|---|---|---|---|
| GEOS bindings | `geos` | 11.3.1 | Safe wrapper over the GEOS C API. Links native `libgeos_c`. |
| GEOS raw FFI (fallback) | `geos-sys` | 2.0.10 | Only if `geos` lacks a needed call. |
| gRPC client | `tonic` | 0.14.9 | Channel + generated client. |
| Protobuf runtime | `prost` | 0.14.6 | Message types used by tonic. |
| Codegen from `.proto` | `tonic-build` | 0.14.9 | Needs `protoc` on PATH at build time. |

Versions come from the crates.io index. Re-check before adding them to `Cargo.toml`.

## Native libraries

- Windows: GEOS is not installed. Build the GEOS-dependent code in WSL only.
- WSL (Ubuntu noble): `libgeos-dev 3.12.1-3build1` is installed. `geos` 11.x needs GEOS >= 3.11, so 3.12.1 is fine.
- `protoc` is NOT installed in WSL (`which protoc` is empty). It is required by `tonic-build`.
- JuPedSim C API (`jupedsim/jupedsim.h`) is used by the legacy model. No Rust crate was chosen yet;
  bindgen over that header is the planned route.

## Commands (WSL / Linux)

```sh
sudo apt-get update
sudo apt-get install -y libgeos-dev protobuf-compiler
geos-config --version        # expect 3.12.x
protoc --version             # expect libprotoc 3.x or newer

# Once the crates are added to crates/sumo_core/Cargo.toml:
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target CARGO_BUILD_JOBS=4
cargo check -p sumo_core
```

On Windows, `cargo check -p sumo_core` keeps working without these crates, because the fields use
opaque `Option<Box<dyn std::any::Any>>` holders for now.

## Plan

1. Add `geos`, `tonic`, `prost`, `tonic-build` (build-dependency) only in the commit that ports the first GEOS-using method.
2. Replace each `Option<Box<dyn std::any::Any>>` holder with the named crate type (see the `// shortcut:` comments).
3. Keep the `Vec`/`BTreeMap` shapes above; they match C++ ownership and ordering except where marked `determinism:`.

# CI

Runs on every push and pull request (Linux):

- `check`: `cargo check -p sumo_core --message-format short` on stable Rust, with `~/.cargo` and `target` cached.
- `oracle-tools`: `python -m unittest discover -s tools/oracle -p 'test_*.py'`.

The SUMO C++ oracle is NOT run in CI: it needs a 20-minute build.

# Sandpit

A local-first harness for automatic coding-agent runners. A runner is a reusable job
definition; the engine runs it in an isolated sandbox; the sandbox backend is pluggable.

Early. The core model and backend contract exist; the Podman backend and CLI come next.
v0 targets Linux with rootless Podman.

## Layout

- `crates/sandpit-core` — runner model, run records, backend trait. No UI, no container
  runtime.

## Build

```
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

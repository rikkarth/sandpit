# AGENTS.md

## Layout

- `sandpit-core` holds the model and the backend trait. It must not depend on Rune,
  winit, or any other UI crate.
- Backends implement `sandpit_core::Backend`. The engine never calls Podman outside a
  backend.

## Commands

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`

## Sandbox safety

- Rootless Podman only. Network is `none` by default; `egress` only when a runner needs
  it.
- Never mount host credentials or `$HOME`. Mount only explicit paths.
- Images are digest-pinned. A tag alone is refused.

## Secrets

- Runner files name environment variables, never values.
- Values come from the host environment or a gitignored `.env`; they are never logged or
  written into run records.

## Git

- Conventional, atomic commits. Do not push without Ricardo's say-so.

## Agent skills

### Issue tracker

Issues live in GitHub Issues via `gh`. See `docs/agents/issue-tracker.md`.

### Triage labels

Five default roles, label strings equal to their names. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` and `docs/adr/` at the repo root. See `docs/agents/domain.md`.

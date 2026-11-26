# Sandpit

A local-first harness for automatic coding-agent runners. Each runner runs in an
isolated sandbox on the user's machine, and the sandbox backend is a choice, not an
assumption.

The name is the smaller sandcastle: less fortification, more room to experiment.

## Why

Two projects point here:

- **sandcastle-review** runs a review/ship loop with agents in containers on a Linux box.
  It works, and it showed that the loop itself is the product.
- **Matt's sandcastle** orchestrates sandboxed coding agents from TypeScript. It showed
  that runners can be a library primitive rather than a fixed pipeline.

Sandpit takes the next step: a small, general engine where a **runner** is the unit, a
**backend** is pluggable, and a desktop app makes the first run effortless.

## What it is

- **Runner** — a reusable job definition: image, agent CLI, prompt/task, mounts, network
  policy, time limit, expected artifacts.
- **Engine** — headless core that schedules and executes runs in local containers,
  streams logs and records results. Usable from the CLI, no GUI required.
- **Backend** — the sandbox implementation. v0 ships Podman. Docker is next. Native
  OS sandboxes (macOS Seatbelt, Linux bubblewrap) are a later possibility, not a promise.
- **Desktop app** — Rust, built with [Rune](https://github.com/anomalyco/rune): pick a
  runner, pick a repo, run, watch logs, inspect results.

## v0 scope

- Linux only, rootless Podman only.
- Engine first: job spec, Podman backend, CLI, logs, results.
- Then the Rune desktop app as a thin client over the same engine.
- macOS support after Docker/Podman on macOS (via `podman machine`) is proven.

### Non-goals for v0

- Cloud or remote sandboxes.
- Windows.
- Multi-user/server mode.
- A GUI before the core runs a job end to end.
- Docker backend.

## Shape

```
sandpit-core   job model, backend trait, Podman implementation, run lifecycle
sandpit-cli    headless entry point
sandpit-app    Rune desktop client (thin, no business logic)
```

Exact repository layout is decided when we scaffold. The rule: `sandpit-core` has zero
UI dependencies, and the Rune app is replaceable.

## Principles

1. **The sandbox is the boundary.** Runners never inherit host credentials; anything a
   run needs is injected deliberately and dies with the run.
2. **Backend is a trait.** Podman is the first implementation, not the architecture.
3. **Headless first.** Every capability exists in the CLI before the GUI exposes it.
4. **Deterministic runs.** Pinned images, explicit mounts, time limits, recorded exit
   status and artifacts.
5. **Agent-drivable UI.** Rune apps render headless to PNG and expose the element tree,
   so agents can build and verify the desktop app with screenshots.
6. **Small surface.** A runner you can read in one screen; a backend you can replace in
   one file.

## References we are studying

Learning from how these projects work, not copying their code.

- [mattpocock/sandcastle](https://github.com/mattpocock/sandcastle) — TypeScript
  orchestration of sandboxed coding agents. Runner ergonomics, provider abstraction.
- [rikkarth/sandcastle-review](https://github.com/rikkarth/sandcastle-review) — the
  review/ship loop this idea grew from. Job lifecycle, human gates, run records.
- [anomalyco/rune](https://github.com/anomalyco/rune) — native Rust UI framework.
  Pre-release: pin a `dev` rev, keep the same `[patch.crates-io]` block in our workspace
  root, track their pin checks. Upstream takes issues, not PRs. Its `demo/opencode` app
  is the reference for structure and headless verification.
- [ghostty-org/ghostty](https://github.com/ghostty-org/ghostty) — native macOS/Linux
  application. Reference for cross-platform engineering discipline, documentation and
  contribution hygiene.

## Name

`sandpit` is the product, repo and binary name. On crates.io and npm the plain name is
taken, so published crates use prefixes (`sandpit-core`, `sandpit-cli`, `sandpit-app`,
free at the time of writing). Binary names are unaffected.

## Decisions

- 2026-11-26 — v0 is Linux + rootless Podman; Docker is the second backend; macOS later.
- 2026-11-26 — Rust throughout; Rune for the desktop app; core independent of the UI.
- 2026-11-26 — Name: sandpit.

## Open questions

- How a runner is defined: TOML, JSON, or a Rust builder?
- Which runner types ship in v0: generic task, code review, implement-issue, fix-CI?
- Where results land: artifacts directory, git worktree, branch, PR?
- One daemon with a queue, or one process per run?
- Networking default: none, with an explicit allowlist per runner?
- How images are built and pinned, and who owns base images?

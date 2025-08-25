# Repository Guidelines

## Project Structure & Modules
- `src/main.rs`: CLI entry and core logic. Unit tests live in the `tests` module within this file.
- `Cargo.toml`/`Cargo.lock`: Crate metadata and dependencies.
- `.github/workflows/`: CI for `fmt`, `clippy`, and tests.
- `README.md`: Usage and feature overview.
- `target/`: Build artifacts (ignored).

## Build, Test, and Run
- Build: `cargo build` (use `--release` for optimized binary).
- Test: `cargo test` (runs unit tests in `src/main.rs`).
- Lint: `cargo clippy -- -D warnings` (CI denies warnings).
- Format check: `cargo fmt --all -- --check`.
- Run locally: `cargo run -- path/to/config.json` or `cargo run -- path/to/config.json.tera`.

## Coding Style & Conventions
- Language: Rust (edition 2024), toolchain pinned in `rust-toolchain.toml` (1.85).
- Formatting: `rustfmt` required; CI enforces style.
- Linting: `clippy` with warnings denied; fix or justify lints.
- Naming: `snake_case` for functions/vars, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for consts.
- CLI behavior conventions (keep consistent when changing logic):
  - JSON keys of length 1 → `-k`; length >1 → `--key`.
  - Leading `_` in a key suppresses the flag (value only).
  - Nested objects use dot notation (e.g., `--parent.child 1`).
  - Arrays flatten to space-separated values.
  - Files ending with `.tera` are rendered via Tera.

## Testing Guidelines
- Unit tests: add to the `#[cfg(test)]` module in `src/main.rs` for core functions.
- Integration tests: create files under `tests/` if scenarios cross module boundaries.
- Aim to cover key paths: numbers, strings, arrays, nested objects, `_`-prefixed keys, and `.tera` rendering.
- Run: `cargo test` before pushing.

## Commits & Pull Requests
- Commits: concise, imperative mood (e.g., "fix: handle nested arrays"). Scope tags are welcome.
- PRs: include a summary, reasoning, and test coverage notes. Link issues when applicable.
- CI must pass (`fmt`, `clippy`, `test`). Screenshots not required for this CLI.
- Branching: open PRs against `master`. Dependabot patches may auto-merge after CI.

## Security & Configuration Tips
- Do not commit secrets. Inputs are local JSON/Tera files; validate paths in examples.
- Prefer deterministic tests and avoid filesystem/network side effects.

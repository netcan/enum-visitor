# Repository Guidelines

This Rust workspace provides a small library that emulates a C++ `std::visit`-like experience via macros and a proc‑macro derive.

## Project Structure & Module Organization
- Root: Cargo workspace (`Cargo.toml`), `README.md`, `LICENSE`.
- Crates:
  - `enum-visitor/`: end‑user crate exporting `VisitEnum` and a universal `visit_with!` macro. Examples in `enum-visitor/examples/`.
  - `enum-visitor-derive/`: proc‑macro crate implementing `#[derive(VisitEnum)]`.
- Tests live under each crate (`<crate>/tests/`) or inline via `#[cfg(test)]`.

## Build, Test, and Development Commands
- Build/check: `cargo check` • Full: `cargo build --workspace --all-targets`.
- Test: `cargo test --workspace`.
- Example: `cargo run -p enum-visitor --example shapes`.
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`.
- Format: `cargo fmt --all` (CI-style: `cargo fmt --all -- --check`).

## Coding Style & Naming Conventions
- Rust 2021 edition; default `rustfmt` (4‑space indent, trailing commas where sensible).
- Public API minimal and documented with `///` and examples.
- Macros: snake_case names; derive generates `visit_with_<enum_snake>!` and a local `visit_with!` in the enum’s module. To avoid name collisions, place different enums in separate `mod`s or use the unique macro name.
- Avoid `unsafe` unless justified; prefer clear, small macros over cleverness.

## Testing Guidelines
- Prefer black‑box tests under `<crate>/tests/` and doctests in public items.
- For macro behavior, add examples that compile and run; optional compile‑fail checks can use `trybuild` in follow‑ups.
- Name tests after the behavior under test, e.g., `visit_expr_form.rs`, `visit_block_form.rs`.

## Commit & Pull Request Guidelines
- Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:` with optional scope, e.g., `fix(derive): pat_param binder`.
- Keep commits atomic; include rationale for proc‑macro changes and a minimal expanded snippet in the PR description.
- PRs should include: problem statement, approach, testing evidence (commands/output), and breaking‑change notes if any.

## Security & Configuration Tips
- No `build.rs` or network access required; keep builds reproducible.
- License: MIT (copyright © netcan). Be mindful when copying external code.

## Agent‑Specific Instructions
- Keep patches minimal and focused; avoid reformatting unrelated lines.
- When referencing files in discussions, use workspace‑relative paths (e.g., `enum-visitor/src/lib.rs`).

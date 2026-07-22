# Repository Guidelines

## Project Structure & Module Organization

`cryptoed` is a synchronous Rust 2024 terminal editor. The binary entry point is `src/main.rs`. Command-line parsing lives in `src/cmd`, while `src/controllers` coordinates file loading, encryption, editor startup, and saving. Reusable functionality belongs in `src/shared`, including cryptography, Markdown rendering, and terminal extensions.

The editor use case is under `src/use_cases/editor`:

- `app_ui.rs` registers global shortcuts and starts Cursive.
- `layers/` contains full-screen and modal UI layers.
- `widgets/` contains reusable editor, preview, and shortcut views.
- `theme.toml` defines the bundled Cursive theme.

Unit tests are colocated with implementation files in `#[cfg(test)]` modules. There is currently no separate `tests/` directory.

## Build, Test, and Development Commands

- `cargo run -- path/to/notes.md`: run the editor for a plain-text file.
- `cargo run -- path/to/notes.encrypted`: open or create an encrypted file and prompt for its password.
- `cargo build`: compile a debug build.
- `cargo build --release`: produce the size-optimized release binary.
- `cargo test`: run all unit tests.
- `cargo fmt -- --check`: verify standard Rust formatting.
- `cargo clippy --all-targets`: run static analysis for the binary and tests.

Run formatting, tests, and Clippy before submitting changes.

## Coding Style & Naming Conventions

Use `rustfmt` defaults and four-space indentation. Follow Rust naming conventions: `snake_case` for modules, functions, and tests; `PascalCase` for structs and traits; `SCREAMING_SNAKE_CASE` for constants. Keep modules focused and expose items as `pub(crate)` unless external visibility is necessary. Preserve the synchronous, single-threaded control flow; do not introduce async runtimes without a concrete requirement.

## Testing Guidelines

Add focused unit tests beside changed logic. Name tests after observable behavior, such as `render_removes_markdown_syntax`. Cover success paths and important edge cases, especially encryption compatibility, editor mode transitions, and terminal layout constraints. Every bug fix should include a regression test when practical.

## Commit & Pull Request Guidelines

History uses short, imperative commit subjects such as `Add preview mode` and `Remove async`. Keep each commit scoped to one logical change. Pull requests should explain behavior changes, list validation commands, and identify compatibility or security implications. Include terminal screenshots for visible UI changes and link the relevant issue when one exists.

## Security Notes

Never commit passwords, decrypted user content, generated encrypted fixtures, or local editor files. Changes to encryption headers, Argon2 parameters, nonce handling, or authenticated data require compatibility tests and explicit review notes.

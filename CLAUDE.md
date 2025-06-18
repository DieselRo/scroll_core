# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Goal
We are translating and building the entire Agent Development Kit from a Python project into a Rust application. We want it to run as close to exactly like the Google ADK as possible, with minor changes such as using SeaORM and SQLite for the database. We also want the structures and schemas to be as close as they can, within Rust's limitations. The Rust manual is available at `scroll_core/rendered-rust-book.pdf`. Please ask any questions if things are unclear. There are quite a few modules in the project that aren't part of the ADK integration; they are for other systems.

## Build Commands
- Build project: `cargo build`
- Run project: `cargo run`
- Run specific binary: `cargo run --bin scroll_core`
- Run all tests: `cargo test`
- Run specific test file: `cargo test --test invocation_tests`
- Run single test: `cargo test test_glyph_match_exact` or `cargo test --test trigger_loom_tests test_glyph_match_exact`
- Lint with clippy: `cargo clippy`
- Format code: `cargo fmt`
- Debug tests: Use VS Code launch configurations with LLDB

## Code Style Guidelines
- **Modules**: Use separate `mod.rs` files for module organization
- **Header Comments**: Include file path in top comment: `// src/module/file.rs`
- **Naming**: Types use PascalCase, functions/variables use snake_case, constants use SCREAMING_SNAKE_CASE
- **Imports**: Group standard library, external crates, then internal imports
- **Error Handling**: Return `Result<T, String>` with descriptive error messages using `.map_err(|e| e.to_string())?` pattern
- **Documentation**: Use `///` comments for public API documentation
- **Testing**: Integration tests in `/tests` directory with descriptive names
- **Database**: Uses Sea ORM with SQLite and follows migration pattern
- **Project Structure**: Organized as a workspace with `scroll_core` and `migration` crates

## ADK Translation Notes
- Original Python ADK reference is in `/docs/ADK/`
- Maintain same interface, functionality and behavior as the Python ADK where possible
- Implement Rust-idiomatic solutions for Python-specific features
- Python class methods typically become associated functions or instance methods
- Async/await patterns should be maintained with Tokio runtime
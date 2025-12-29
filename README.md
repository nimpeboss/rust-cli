# forge (rust-cli)

A small, opinionated command-line utility written in Rust. This repository was created as a learning project to explore idiomatic CLI design, async workflows, and common Rust libraries such as `clap`, `tokio`, and `anyhow`.

---

## Features ✅

- Modular subcommand architecture (e.g., `hash`, `scan`).
- Robust error handling with `anyhow`.
- Asynchronous operations powered by `tokio`.
- Progress and status reporting with `indicatif`.

## Getting started 🔧

### Prerequisites

- Rust toolchain (stable) and `cargo`.

### Build

```bash
cargo build --release
```

### Run

```bash
# Run directly with cargo
cargo run -- <command> [args]

# Or install and run the produced binary
cargo install --path .
forge <command> [args]
```

## Usage examples

- `forge hash <file>` — calculate a file hash
- `forge scan <directory>` — recursively scan a directory

(See the `src/commands` directory for implemented subcommands and options.)

## Contributing 🤝

Contributions are welcome. Open an issue to discuss ideas or submit a pull request with tests and a clear description of changes. Please run `cargo fmt` and `cargo clippy` before submitting.

## Development notes

- Run tests: `cargo test`
- Lint: `cargo clippy -- -D warnings`

## Maintenance & updates 🛠️

This project is actively maintained as a learning exercise. Expect occasional updates — often when the developer has spare time or gets bored and starts experimenting with new ideas.

---

**Author:** nimpeboss

If you'd like to propose an enhancement or report a bug, please open an issue — feedback is appreciated.
# Scroll Core Chat CLI

Run an interactive chat session with a Construct from your terminal.

## Installation

```
cargo build --release
```

## Usage

Start a chat with Mythscribe:

```
cargo run -- chat mythscribe
```

Type messages after the `You ›` prompt. Use `exit` to quit.

Enable streaming output (default):

```
cargo run -- chat mythscribe --stream
```

Disable streaming:

```
cargo run -- chat mythscribe --stream=false
```

Chat history is stored in `scroll_core.db`.

The archive directory defaults to `./scrolls`. Override this with the
`SCROLL_CORE_ARCHIVE_DIR` environment variable. The chat CLI will create the
directory (with a `.gitkeep` file) if it doesn't already exist.

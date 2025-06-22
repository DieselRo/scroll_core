# CLI Quick-Start

This guide assumes the repo has been cloned and dependencies are installed.

## Build

```bash
cargo build --workspace
```

## Run Mythscribe

```bash
cargo run -- chat mythscribe --no-stream
# Flags:
#   --stream / --no-stream
#   --theme dark|light
#   --no-banner
# Slash commands (after CLI-3):
#   /help
#   /scroll list
#   /scroll open <id>
```

Press `Ctrl-S` during a chat session to toggle streaming after CLI-4.

## Common Dev Tasks

| Command | Purpose |
|---------|---------|
| `cargo test` | Run tests |
| `cargo xtask gen-map` | Update module map |
| `cargo fmt` | Format the code |

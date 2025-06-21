#!/usr/bin/env bash
set -euo pipefail
if ! command -v sqlite3 >/dev/null; then
  sudo apt-get update
  sudo apt-get install -y sqlite3 libsqlite3-dev
fi
if ! command -v sea-orm-cli >/dev/null; then
  cargo install sea-orm-cli sqlx-cli cargo-udeps cargo-deny --locked
fi

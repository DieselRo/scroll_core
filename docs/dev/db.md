# Database Configuration

Scroll Core stores chat history in a SQLite database. When opening the database you must provide a full SQLite URL beginning with `sqlite://`.

Use `:memory:` to create an ephemeral database for tests and temporary sessions. Otherwise pass a file path, for example `scroll_core.db`, which will be translated internally to `sqlite://scroll_core.db?mode=rwc`.

Relative paths are resolved by `sqlx` after the `sqlite://` prefix is added, so paths like `./tmp/chat.db` work as expected.

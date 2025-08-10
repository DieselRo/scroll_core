---
title: Sessions & Database
status: active
audience: dev
---

# Sessions & Database

Responsibilities
- Persist chat messages and invocation events
- Provide durable history for context building

Key modules
- `scroll_core/src/sessions/database.rs`
- `scroll_core/src/sessions/database_session_service.rs`
- Migrations under `migration/`

Configuration
- `DATABASE_URL` (sqlite URL by default). On Windows, prefer absolute `sqlite:///C:/path/to/db.sqlite`.

Initialization & Readiness
- Use `ensure_ready_with_url(url)` or `ensure_ready_from_env()` to connect and run migrations.
- Writers and services should be started after readiness to avoid table-missing errors.
- CLI subcommands (`open-threads`, `context`, normal start) call readiness before DB work.

Notes on SQLite
- Avoid URL query params like `?mode=rwc`; they may not be honored by the driver. The runtime strips them.
- Use absolute paths on Windows to avoid cross-process visibility issues.



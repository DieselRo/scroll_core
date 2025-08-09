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
- `DATABASE_URL` (sqlite URL by default)



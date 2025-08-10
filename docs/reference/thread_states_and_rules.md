# Thread States & Rules

This document defines the thread lifecycle, allowed transitions, tag normalization, and event recording in Scroll Core.

## States

- OPEN: newly created and triaged items
- IN_PROGRESS: actively being worked
- BLOCKED: requires external input
- CLOSED: finished or discarded

## Allowed Transitions

```
OPEN  ─────▶ IN_PROGRESS ─────▶ CLOSED
  │              │                 ▲
  │              └────▶ BLOCKED ───┘
  └────▶ BLOCKED ─────▶ CLOSED

CLOSED ─────▶ OPEN   (reopen)
```

Illegal transitions (e.g., CLOSED → IN_PROGRESS) are rejected.

Reopening from CLOSED → OPEN increments `reopened_count`.

## Priority

Allowed values: LOW, MEDIUM, HIGH. Invalid values are rejected at parse time in the CLI and validated in code.

## Tags

- Lowercased
- Unique
- Sorted

## Events

All state changes write an event into `thread_events` with:

- event_type: COMMENT | STATUS_CHANGE | ASSIGNMENT | TAG_CHANGE | SYSTEM_NOTE
- actor: human or system identifier
- reason: optional message
- created_at: timestamp


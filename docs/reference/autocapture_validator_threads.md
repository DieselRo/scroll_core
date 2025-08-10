# Autocapture: How Validators Open/Close Threads

Scroll Core integrates validator results with the thread system to capture work automatically.

## Behavior

- On failure:
  - Dedupe-or-open a thread keyed by `VALIDATOR|<scroll_path>`.
  - `source=VALIDATOR`, `priority=MEDIUM`, `due_at=now+48h`.
  - Records a SYSTEM_NOTE event.

- On pass:
  - If a matching thread exists and is not CLOSED, it is closed with reason `validator pass`.

- On repeated failure after close:
  - The prior CLOSED thread is reopened (incrementing `reopened_count`).

## Service Facade (Autocapture)

Higher-level subsystems should use the small facade to capture threads with smart defaults:

```text
ThreadAutocapture::on_validator_failure(scroll_path, title)
ThreadAutocapture::on_validator_pass(scroll_path)
ThreadAutocapture::nudge_blocked_or_overdue()
```

Defaults:
- source=VALIDATOR, priority=MEDIUM, due_at=now+48h, consistent reasons.

## CLI Examples

- Validate one file (autocapture applies):
  - `cargo run -- ritual validate --file scrolls/a.md`

- Validate all:
  - `cargo run -- ritual validate-all`

## Notes

- Matching key is `VALIDATOR|<scroll_path>`.
- Tag normalization, transitions, and event recording follow the core thread rules.


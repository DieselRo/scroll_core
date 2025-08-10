---
title: Notifications Runbook
status: active
audience: dev, operator
---

# Notifications Runbook

## Enabling Notifications

- ENV:
  - `SC_NOTIFICATIONS_ENABLED=true|false` (default: true)
  - `SC_NOTIFICATIONS_SINKS=stdout,log,slack` (default: stdout)
  - `SC_NOTIFICATIONS_RATE_MIN=10` (default: 10)
  - `SC_SLACK_WEBHOOK_URL=<url>` (required when `slack` sink is used)
- Build-time: enable feature `notifier_slack` to compile Slack sink.

## Integration Points

- Thread lifecycle (`thread_state_service.rs`): emits on create, status changes, assignment changes; priority-only when escalating to High.
- Autocapture nudges (`thread_autocapture.rs`): emits Overdue system notes and notifications for blocked/overdue threads.

## Policy & Limits
- Allowed kinds: ThreadCreated, AssignmentChanged, StatusChanged, Overdue
- Not allowed: Tag changes; priority changes unless raising to High
- Rate limit: one per thread-kind per `SC_NOTIFICATIONS_RATE_MIN` minutes (default 10)
- Flap suppression: prevents rapid back-and-forth status spam within 5 minutes

## What gets notified

- ThreadCreated
- AssignmentChanged
- StatusChanged (flap-suppressed 5 min window)
- Overdue (emitted by autocapture nudge)

Not notified:
- Tag changes
- Priority changes (unless your downstream sink uses separate policy; current hub does not notify)

## Rate Limits

- One notification per thread per kind at most once per `SC_NOTIFICATIONS_RATE_MIN` minutes.

## Templates

- Short, structured message including `id`, `title`, `scroll_path`, `reason`, plus a placeholder for link.

## Testing

- Unit tests cover policy and rate limits in `scroll_core/tests/notifications_tests.rs`.
- Integration flow exercised indirectly by thread state and autocapture tests.



title: Models Configuration
status: active
audience: user, dev
---

# Models, Cost & Context Configuration

Single source of truth for provider/model selection and optional cost thresholds, resolved with strict precedence: ENV > YAML > built-in defaults.

Environment variables (global):
- `SC_MODELS_CONFIG`: Path to YAML file (default: `config/models.yaml` if present)
- `SC_LLM_PROVIDER`: `openai` | `mock` | `local` | `anthropic` (default: `openai`, or `mock` in tests)
- `SC_LLM_MODEL`: model identifier (e.g., `gpt-4.1-mini`)
- `SC_LLM_MAX_OUTPUT_TOKENS`: integer
- `SC_LLM_TEMPERATURE`: float
- `SC_COST_DAILY_USD_CAP`: float
- `SC_COST_PER_REQUEST_USD_LIMIT`: float

Per-construct env overrides:
- `SC_MODEL_<Name>_PROVIDER`
- `SC_MODEL_<Name>_MODEL`
- `SC_MODEL_<Name>_MAX_OUTPUT_TOKENS`
- `SC_MODEL_<Name>_TEMPERATURE`
- `SC_COST_<Name>_DAILY_USD_CAP`
- `SC_COST_<Name>_PER_REQUEST_USD_LIMIT`

Note: `<Name>` is the construct’s display name, e.g., `Mythscribe`.

YAML schema (`version: 1`):
```
version: 1
default:                # optional; fallback for unknown constructs
  provider: openai
  model: gpt-4.1-mini
  max_output_tokens: 2048
  temperature: 0.3
  extra: {}             # optional provider-specific knobs
constructs:             # optional per-construct overrides
  Mythscribe:
    provider: openai
    model: gpt-4.1
    max_output_tokens: 4096
    extra:
      input_per_1k_usd: 0.01     # optional pricing hints for guards
      output_per_1k_usd: 0.03
cost_profiles:          # optional caps/limits (enforced by CostManager)
  default:
    daily_usd_cap: 5.00
    per_request_usd_limit: 0.10
  Mythscribe:
    per_request_usd_limit: 0.25

context:               # optional context selection thresholds
  defaults:
    max_context_tokens: 3000
    min_relevance_score: 0.35
    recency_half_life_hours: 48
    max_items: 12
  constructs:
    Mythscribe:
      max_context_tokens: 5000
      min_relevance_score: 0.40
      recency_half_life_hours: 24
      max_items: 16
```

Behavior:
- With no YAML, behavior matches today: provider/model come from env, or `openai/gpt-4o` by default.
- `--print-model-config` prints the resolved config with secrets redacted.
- Cost enforcement lives in `CostManager`; registry only supplies thresholds.
- Context thresholds precedence: ENV > YAML > built-ins.

Environment variables (context thresholds):
- Global: `SC_CONTEXT_MAX_TOKENS`, `SC_CONTEXT_MIN_RELEVANCE`, `SC_CONTEXT_RECENCY_HALF_LIFE_HOURS`, `SC_CONTEXT_MAX_ITEMS`
- Per-construct: `SC_CONTEXT_<Name>_MAX_TOKENS`, `SC_CONTEXT_<Name>_MIN_RELEVANCE`, `SC_CONTEXT_<Name>_RECENCY_HALF_LIFE_HOURS`, `SC_CONTEXT_<Name>_MAX_ITEMS`

CLI explainability:
- `--explain-context` (on `chat` subcommand) prints a table of included/excluded items with reasons, scores, age, and token budget.

Secrets:
- API keys are never printed. Provide `OPENAI_API_KEY` via env as usual.

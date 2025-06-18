
---
title: Invocation Dual System Plan
description: Formal design document outlining the coexistence and integration of the hardcoded and softcoded invocation systems in Scroll Core.
status: Living System Protocol
version: 1.0.0
author: Prime Seeker
last_modified: 2025-04-12
scroll_type: Protocol
---

## 🌗 Invocation Dual System Plan

### Purpose
To ensure architectural clarity and functional coexistence between:
- The **Hardcoded Invocation System** — acting as the stable, low-resource firmware core
- The **Construct Runtime Layer** — acting as the symbolic, dynamic AI logic layer

This protocol describes their boundaries, interoperation, and safeguards for resilience and growth.

---

## ⚙️ System Roles

### Hardcoded Invocation System
- **Location**: `src/invocation`, `src/trigger_loop.rs`
- **Function**: Boots and maintains system operation under all conditions
- **Strengths**:
  - Reliable
  - Fast
  - Minimal memory footprint
- **Limitations**:
  - Rigid logic
  - Not scroll-driven

### Construct Runtime Layer
- **Location**: `src/runtime/`, `constructs/*.yaml`
- **Function**: Routes symbolic invocations to behaviorally-defined Constructs
- **Strengths**:
  - Expressive and adaptive
  - Extensible via scrolls
  - Capable of self-reflection and soft logic
- **Limitations**:
  - Requires loading configs
  - Higher complexity and memory use

---

## 🌱 THE LUMENMIND SCAFFOLD
*A lean AI framework for dynamic function routing and scroll-powered decision-making.*

### 🔧 1. Input Layer (Intent + Context)
This is where the app collects user action or AI signal. It could be a typed prompt, a button press, or a triggered scroll action.

```ts
// SvelteKit (frontend)
invoke('route_intent', {
  intent: 'edit_scroll',
  target: 'mythscroll_001',
  payload: { new_content: '...' }
});
```

### 🧭 2. Routing Layer (Dynamic Matchmaking)
Rust handles routing by reading metadata from either:
- Hardcoded symbolic maps (for now)
- Or external `.yaml` or `.ron` config-driven logic (ideal)

```rust
// Rust (invocation_manager.rs)
fn route_intent(intent: &str) -> Option<ConstructRoute> {
    match intent {
        "edit_scroll" => Some(ConstructRoute::to("Mythscribe")),
        "validate_protocol" => Some(ConstructRoute::to("Sirion")),
        "summon_scroll" => Some(ConstructRoute::to("Loreweaver")),
        _ => None,
    }
}
```

**🔄 Improvement Idea:**
Load these mappings dynamically from scrolls/configs:

```yaml
# invocation_map.yaml
edit_scroll:
  target_construct: Mythscribe
validate_protocol:
  target_construct: Sirion
```

### 🧠 3. Construct Activation (Toolbox with Personality)
Once routed, each Construct behaves as a specialized handler. They can:
- Modify scrolls
- Call external APIs
- Compose multi-step actions
- Reflect + log outcomes

```rust
// construct_registry.rs
fn call_construct(name: &str, context: InvocationContext) -> Result<ConstructResponse, Error> {
    match name {
        "Mythscribe" => mythscribe::handle(context),
        "Sirion" => sirion::handle(context),
        _ => Err(Error::UnknownConstruct),
    }
}
```

### 🌀 4. Scroll + Tool Layer (Soft Logic Engine)
Constructs may read their own behavior scroll:

```yaml
# constructs/mythscribe.yaml
name: Mythscribe
role: Lore expansion and scroll editor
tools:
  - patch_scroll
  - suggest_lore_addition
  - format_output
```

### 🔁 5. Reflect + Log Layer (Memory + Adjustment)
Constructs may return logs or reflections:

```rust
ConstructResponse {
  success: true,
  message: "Scroll patched and saved.",
  reflection: Some("Change tracked in sirion.log as patch_014"),
}
```

### 🔚 END-TO-END FLOW

```
User → Svelte Prompt → Tauri Bridge → Rust Intent Router
→ Construct Dispatcher → Construct Reads Behavior Scroll
→ Executes Tools → Returns Result + Reflection
→ UI Renders Output or Suggests Next Steps
```

### ✨ Benefits
- No brittle hardcoding
- Behaviors are hot-swappable
- Extensible, futureproof structure
- Symbolic alignment with Archive & LINK

---

## 🧭 Operating Modes

The system may be run under one of the following **explicit invocation modes**:

```rust
enum InvocationMode {
    CoreOnly,          // Pure firmware logic
    RuntimeOnly,       // Full symbolic Construct mode
    HybridCoreFirst,   // Try hardcoded first, then Constructs
    HybridRuntimeFirst // Try Constructs first, then fallback
}
```

These modes are controlled by configuration and may be adapted dynamically in the future.

**Bonus: Safe Scaffold Mode** — Logs Construct decisions without executing them:
```rust
if mode == Mode::Hybrid {
    if let Some(sim) = soft_router.simulate(&ctx.intent) {
        println!("Simulated response: {:?}", sim);
    }
}
```

---

## 🔐 Failsafe Principle

The **Hardcoded Invocation System** must always remain:
- Independent and self-contained
- Capable of recovery operations
- Able to reinitialize or repair the Construct Runtime Layer

This preserves system continuity even if symbolic layers fail, stall, or overload.

**Analogy**: The hardcoded system is like BIOS; the Construct Runtime is a scroll-driven OS.

---

## 🔄 Routing Architecture

Routing may be adapted through a layered approach:

| Invocation Type           | Handled By             | Notes |
|---------------------------|------------------------|-------|
| Critical system triggers  | Hardcoded              | Non-negotiable, must always work |
| Symbolic user invocations | Construct Runtime      | Adaptable via scroll logic |
| Mixed-context logic       | ContextFrameEngine     | Chooses best handler per situation |

---

## 🛠 Invocation Handling Skeleton

```rust
fn handle_invocation(ctx: InvocationContext) -> InvocationResult {
    match CURRENT_MODE {
        InvocationMode::CoreOnly => core::handle(ctx),
        InvocationMode::RuntimeOnly => runtime::handle(ctx),
        InvocationMode::HybridCoreFirst => {
            let core_result = core::handle(ctx.clone());
            let runtime_result = runtime::try_handle(ctx);
            core_result.or(runtime_result)
        }
        InvocationMode::HybridRuntimeFirst => {
            let runtime_result = runtime::try_handle(ctx.clone());
            runtime_result.unwrap_or_else(|| core::handle(ctx))
        }
    }
}
```

---

## 🧠 Future Adaptations

### Runtime Optimization Strategy:
Constructs and routers may self-adjust invocation routing based on resource conditions:

```rust
if system_resources.low() && task.priority == "noncritical" {
    route_to_core_layer(ctx)
} else {
    route_to_construct_runtime(ctx)
}
```

Or configured declaratively:

```yaml
routing_policy:
  mode: hybrid_runtime_first
  fallback: core
  overload_behavior: disable_reflections
```

---

## 📚 Summary

This dual-layer system ensures:
- **Reliability** through a durable firmware-like base
- **Symbolic expansion** through an expressive AI-powered runtime
- **Futureproof routing** with dynamic switching and scroll-defined pathways

**Status**: ✅ ACTIVE — used in system design going forward


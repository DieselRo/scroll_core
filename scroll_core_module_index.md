
# 📘 Scroll Core System Module Index

This document provides an indexed summary of all modules, types, functions, and interconnections present in the current `scroll_core` system. It includes file-level context across `parser.rs`, `schema.rs`, `validator.rs`, `state_manager.rs`, `scroll.rs`, and the `invocation/` engine.

---

## 🗂️ Module Index

### ✅ `schema.rs`
- **Types**:
  - `ScrollType`: Enum
  - `ScrollStatus`: Enum
  - `EmotionSignature`: Struct
  - `YamlMetadata`: Struct
- **Usage**: Referenced across scroll parsing, validation, state transitions, and invocation metadata.

---

### ✅ `parser.rs`
- **Functions**:
  - `parse_scroll_file(path: &Path) -> Result<Scroll, Box<dyn Error>>`
- **Purpose**: Extracts `YamlMetadata` and body from `.md` scroll files.
- **Depends on**: `schema.rs`

---

### ✅ `validator.rs`
- **Functions**:
  - `validate_scroll(metadata: &YamlMetadata) -> Result<(), String>`
- **Purpose**: Ensures scroll metadata conforms to rules.
- **Uses**: `ScrollStatus`, `ScrollType`

---

### ✅ `state_manager.rs`
- **Functions**:
  - `transition(scroll: &mut Scroll, next_status: ScrollStatus) -> bool`
- **Purpose**: Manages scroll lifecycle transitions.
- **Depends on**: `ScrollStatus`, `Scroll`

---

### ✅ `scroll.rs`
- **Constants**:
  - `SCROLL_CORE_VERSION`, `SCROLL_CORE_INVOCATION`
- **Functions**:
  - `initialize_scroll_core()`
  - `teardown_scroll_core()`
  - `validate_scroll_environment()`
- **Purpose**: System-level init/teardown and environment validation.

---

## 🔁 `invocation/` Engine Modules

### 🌀 `invocation.rs`
- **Types**:
  - `Invocation`: Struct
  - `InvocationMode`: Enum (e.g., Validate, Transition)
  - `InvocationTier`: Enum
  - `InvocationResult`: Enum (Success, Failure, ModifiedScroll)

---

### 🧱 `named_construct.rs`
- **Trait**:
  - `NamedConstruct`
    - `fn name(&self) -> &str`
    - `fn perform(&self, invocation, scroll) -> Result<InvocationResult, String>`

---

### 🧾 `ledger.rs`
- **Function**:
  - `log_invocation(path: &Path, invocation: &Invocation) -> Result<()>`

---

### 🛡 `constructs/validator_construct.rs`
- **Struct**:
  - `Validator`
- **Implements**:
  - `NamedConstruct`
- **Handles**:
  - `InvocationMode::Validate`
- **Uses**:
  - `validate_scroll()`

---

## 🧪 Test Modules

- `parser_tests.rs`: Valid scroll parsing
- `validator_tests.rs`: Metadata validation tests
- `state_manager_tests.rs`: Transition logic
- `invocation_tests.rs`: Validator construct invocation

---

This index reflects the current integrated architecture for Scroll Core — Phase 1. Use this as a basis for refactoring, restructuring, or expanding the council with new constructs and systems.


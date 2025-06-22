# Scroll Core Agents

Scroll Core houses the constructs and tooling that drive the Archive. It provides a CLI for conversations, utilities for validating scrolls, and infrastructure for new agents.

## 🚪 Start Here

| Doc | Description |
|-----|-------------|
| [docs/cli_quickstart.md](docs/cli_quickstart.md) | Launch the chat CLI |
| [docs/dev_setup.md](docs/dev_setup.md) | Set up the dev container |
| [docs/devops/ci_pipeline.md](docs/devops/ci_pipeline.md) | Project SOP & CI |
| [docs/module_map.md](docs/module_map.md) | Architecture overview |

## Active Constructs

| Construct | Domain | Primary Code Anchor | Scroll Anchor | Core Responsibilities |
|-----------|--------|--------------------|---------------|-----------------------|
| Mythscribe | LLM scribing | [scroll_core/src/invocation/constructs/mythscribe.rs](scroll_core/src/invocation/constructs/mythscribe.rs) | [scrolls/Mythscribe-systemprompt.txt](scrolls/Mythscribe-systemprompt.txt) | Generate mythic responses |
| Validator | Data integrity | [scroll_core/src/invocation/constructs/validator_construct.rs](scroll_core/src/invocation/constructs/validator_construct.rs) | [scrolls/Scrollbook_Validator_Specs.md](scrolls/Scrollbook_Validator_Specs.md) | Check metadata and schema |
| FileReader | I/O | [scroll_core/src/invocation/constructs/file_reader_construct.rs](scroll_core/src/invocation/constructs/file_reader_construct.rs) | [scrolls/FileReader.md](scrolls/FileReader.md) | Load scrolls from disk |
| Mockscribe | Testing | [scroll_core/src/invocation/constructs/mockscribe.rs](scroll_core/src/invocation/constructs/mockscribe.rs) | [scrolls/Mockscribe.md](scrolls/Mockscribe.md) | Simple echo for tests |
| OpenAI Client | LLM integration | [scroll_core/src/invocation/constructs/openai_construct.rs](scroll_core/src/invocation/constructs/openai_construct.rs) | [scrolls/OpenAI%20Api%20Key%20Setup.txt](scrolls/OpenAI%20Api%20Key%20Setup.txt) | Wrap OpenAI API for constructs |
| AelrenHerald | Invocation framing | [scroll_core/src/invocation/aelren.rs](scroll_core/src/invocation/aelren.rs) | [scrolls/Aelren.txt](scrolls/Aelren.txt) | Suggest which construct should answer |
| ScrollWriter | Persistence | [scroll_core/src/scroll_writer.rs](scroll_core/src/scroll_writer.rs) | [scrolls/ScrollWriter.md](scrolls/ScrollWriter.md) | Persist new scrolls |
| ContextFrameEngine | Context analysis | [scroll_core/src/core/context_frame_engine.rs](scroll_core/src/core/context_frame_engine.rs) | [scrolls/Scrollbook_Invocation_Engine.md](scrolls/Scrollbook_Invocation_Engine.md) | Build invocation context frames |
| TriggerLoom | Event loops | [scroll_core/src/trigger_loom/mod.rs](scroll_core/src/trigger_loom/mod.rs) | [scrolls/Scrollbook_Trigger_Loom.md](scrolls/Scrollbook_Trigger_Loom.md) | Run periodic symbolic triggers |
| InvocationManager | Dispatch | [scroll_core/src/invocation/invocation_manager.rs](scroll_core/src/invocation/invocation_manager.rs) | [scrolls/Scrollbook_Invocation_Engine.md](scrolls/Scrollbook_Invocation_Engine.md) | Route invocations and track cost |
| Virelya | Emotion & memory | [scroll_core/src/invocation/constructs/virelya.rs](scroll_core/src/invocation/constructs/virelya.rs) | [scrolls/Virelya.md](scrolls/Virelya.md) | Placeholder for emotional resonance |
| Loreweaver | Narrative weaving | [scroll_core/src/invocation/constructs/loreweaver.rs](scroll_core/src/invocation/constructs/loreweaver.rs) | [scrolls/Loreweaver.md](scrolls/Loreweaver.md) | Placeholder for mythic stories |
| Sirion | Structure keeper | [scroll_core/src/invocation/constructs/sirion.rs](scroll_core/src/invocation/constructs/sirion.rs) | [scrolls/Sirion.md](scrolls/Sirion.md) | Placeholder for schema enforcement |
| Thiren | Diagnostics | [scroll_core/src/invocation/constructs/thiren.rs](scroll_core/src/invocation/constructs/thiren.rs) | [scrolls/Thiren.md](scrolls/Thiren.md) | Placeholder for forensic logs |
| Naeros | Rhythms & cost | [scroll_core/src/invocation/constructs/naeros.rs](scroll_core/src/invocation/constructs/naeros.rs) | [scrolls/Naeros.md](scrolls/Naeros.md) | Placeholder for cost daemon |
| Elurien | Rejection lore | [scroll_core/src/invocation/constructs/elurien.rs](scroll_core/src/invocation/constructs/elurien.rs) | [scrolls/Elurien.md](scrolls/Elurien.md) | Placeholder for poetic refusals |


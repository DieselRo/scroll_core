# Module Map

<!-- AUTO-GENERATED BY xtask gen-map; DO NOT EDIT -->

| Module Path | Public Items | Purpose |
|-------------|-------------|---------|
| crate::adk::agents::base_agent |  | <!-- stub --> |
| crate::adk::agents::context | InvocationContext, ToolContext | <!-- stub --> |
| crate::adk::agents::llm_agent | LlmAgent | <!-- stub --> |
| crate::adk::agents |  | <!-- stub --> |
| crate::adk::artifacts::base_artifact_service |  | <!-- stub --> |
| crate::adk::artifacts |  | <!-- stub --> |
| crate::adk::common::config | RunConfig, StreamingMode, IncludeContents | <!-- stub --> |
| crate::adk::common::error | AdkError | <!-- stub --> |
| crate::adk::common::types | PartType, Part, Content, InlineData, FunctionCall, FunctionResponse, FunctionDeclaration, ApiResponse | <!-- stub --> |
| crate::adk::common |  | <!-- stub --> |
| crate::adk::events::event | Event | <!-- stub --> |
| crate::adk::events::event_actions | EventActions, TransferToAgentAction | <!-- stub --> |
| crate::adk::events |  | <!-- stub --> |
| crate::adk::memory::memory_service | MemoryEntry | <!-- stub --> |
| crate::adk::memory |  | <!-- stub --> |
| crate::adk::models::base_llm |  | <!-- stub --> |
| crate::adk::models::llm_request | LlmRequest, LlmRequestConfig | <!-- stub --> |
| crate::adk::models::llm_response | LlmResponse | <!-- stub --> |
| crate::adk::models::registry | ModelRegistry | <!-- stub --> |
| crate::adk::models |  | <!-- stub --> |
| crate::adk::runner::in_memory |  | <!-- stub --> |
| crate::adk::runner | Runner | <!-- stub --> |
| crate::adk::sessions::base_session_service |  | <!-- stub --> |
| crate::adk::sessions::database_service::entity::sessions | Model, Relation | <!-- stub --> |
| crate::adk::sessions::database_service::entity::events | Model, Relation | <!-- stub --> |
| crate::adk::sessions::database_service::entity |  | <!-- stub --> |
| crate::adk::sessions::database_service | DatabaseSessionService | <!-- stub --> |
| crate::adk::sessions::in_memory_service | InMemorySessionService | <!-- stub --> |
| crate::adk::sessions::session | SessionStatus, Session, ListSessionsResponse, SessionSummary | <!-- stub --> |
| crate::adk::sessions::state | SessionState, GetSessionConfig | <!-- stub --> |
| crate::adk::sessions |  | <!-- stub --> |
| crate::adk::tools::base_tool |  | <!-- stub --> |
| crate::adk::tools::function_tool | FunctionTool | <!-- stub --> |
| crate::adk::tools |  | <!-- stub --> |
| crate::adk::tests::agent_tests::tests |  | <!-- stub --> |
| crate::adk::tests::agent_tests |  | <!-- stub --> |
| crate::adk::tests::integration_tests::tests |  | <!-- stub --> |
| crate::adk::tests::integration_tests |  | <!-- stub --> |
| crate::adk::tests::runner_tests::tests |  | <!-- stub --> |
| crate::adk::tests::runner_tests |  | <!-- stub --> |
| crate::adk::tests::session_tests::tests |  | <!-- stub --> |
| crate::adk::tests::session_tests |  | <!-- stub --> |
| crate::adk::tests::tool_tests::tests |  | <!-- stub --> |
| crate::adk::tests::tool_tests |  | <!-- stub --> |
| crate::adk::tests |  | <!-- stub --> |
| crate::adk |  | <!-- stub --> |
| crate::archive::archive_loader |  | <!-- stub --> |
| crate::archive::archive_memory | InMemoryArchive | <!-- stub --> |
| crate::archive::error | ArchiveError | <!-- stub --> |
| crate::archive::initialize |  | <!-- stub --> |
| crate::archive::mythic_heat | MythicHeat | <!-- stub --> |
| crate::archive::scroll_access_log | ScrollAccess, ScrollAccessLog | <!-- stub --> |
| crate::archive::semantic_index | TokenEmbedder, MockEmbedder, SemanticIndex | <!-- stub --> |
| crate::archive |  | <!-- stub --> |
| crate::artifact |  | <!-- stub --> |
| crate::artifacts::artifact_service |  | <!-- stub --> |
| crate::artifacts |  | <!-- stub --> |
| crate::cache_manager | CacheManager | <!-- stub --> |
| crate::chat::chat_dispatcher | ChatDispatcher | <!-- stub --> |
| crate::chat::chat_router | ChatRouter | <!-- stub --> |
| crate::chat::chat_session | ChatMessage, ChatSession | <!-- stub --> |
| crate::chat |  | <!-- stub --> |
| crate::cli::chat |  | <!-- stub --> |
| crate::cli::chat_db | ChatDb | <!-- stub --> |
| crate::cli::theme | ThemeKind, Theme | <!-- stub --> |
| crate::cli |  | <!-- stub --> |
| crate::construct_ai | ConstructStyle, ConstructContext, ConstructResult, ConstructInsight, InvocationTrace, DreamingConstruct | <!-- stub --> |
| crate::constructs::base_construct_runner |  | <!-- stub --> |
| crate::constructs::construct_metadata |  | <!-- stub --> |
| crate::constructs |  | <!-- stub --> |
| crate::core::construct_registry | ConstructRegistry | <!-- stub --> |
| crate::core::context_frame_engine | ContextMode, ContextFrameEngine | <!-- stub --> |
| crate::core::cost_manager | CostDecision, RejectionOrigin, InvocationCost, CostProfile, ContextCost, SystemCost, SemanticContextScorer, CostManager | <!-- stub --> |
| crate::core::symbolic_mapper |  | <!-- stub --> |
| crate::core |  | <!-- stub --> |
| crate::database |  | <!-- stub --> |
| crate::errors | MetricError | <!-- stub --> |
| crate::events::scroll_event | ScrollEvent | <!-- stub --> |
| crate::events |  | <!-- stub --> |
| crate::invocation::aelren | AelrenFrameResult, AelrenHerald | <!-- stub --> |
| crate::invocation::constructs::file_reader_construct | FileReader | <!-- stub --> |
| crate::invocation::constructs::mockscribe | Mockscribe | <!-- stub --> |
| crate::invocation::constructs::mythscribe |  | <!-- stub --> |
| crate::invocation::constructs::openai_construct | OpenAIClient, Mythscribe | <!-- stub --> |
| crate::invocation::constructs::validator_construct | Validator | <!-- stub --> |
| crate::invocation::constructs |  | <!-- stub --> |
| crate::invocation::invocation_manager | InvocationManager | <!-- stub --> |
| crate::invocation::ledger |  | <!-- stub --> |
| crate::invocation::named_construct |  | <!-- stub --> |
| crate::invocation::types | InvocationTier, InvocationMode, Invocation, InvocationResult | <!-- stub --> |
| crate::invocation |  | <!-- stub --> |
| crate::memory::memory_result | MemoryDelta | <!-- stub --> |
| crate::memory::memory_service |  | <!-- stub --> |
| crate::memory |  | <!-- stub --> |
| crate::metrics |  | <!-- stub --> |
| crate::models::base_model | LLMResponseContent | <!-- stub --> |
| crate::models::model_registry |  | <!-- stub --> |
| crate::models::scroll_event | Model, Relation | <!-- stub --> |
| crate::models::scroll_session | Model, Relation | <!-- stub --> |
| crate::models |  | <!-- stub --> |
| crate::orchestra::bus | Bus | <!-- stub --> |
| crate::orchestra::message | AgentMessage | <!-- stub --> |
| crate::orchestra |  | <!-- stub --> |
| crate::parser |  | <!-- stub --> |
| crate::runner::core |  | <!-- stub --> |
| crate::runner::invocation_context |  | <!-- stub --> |
| crate::runner |  | <!-- stub --> |
| crate::schema | ScrollType, ScrollStatus, EmotionSignature, YamlMetadata | <!-- stub --> |
| crate::scroll | ScrollOrigin, ScrollLinkType, ScrollLink, Scroll, ScrollBuilder, ScrollDraft | <!-- stub --> |
| crate::scroll_writer | ScrollPatch, ArtifactWriter, ScrollWriter | <!-- stub --> |
| crate::sessions::database_session_service | DatabaseSessionService | <!-- stub --> |
| crate::sessions::error | SessionError | <!-- stub --> |
| crate::sessions::in_memory_session_service::tests |  | <!-- stub --> |
| crate::sessions::in_memory_session_service | InMemorySessionService | <!-- stub --> |
| crate::sessions::session | ScrollSession | <!-- stub --> |
| crate::sessions::session_event_log | SessionEventLog | <!-- stub --> |
| crate::sessions::session_file |  | <!-- stub --> |
| crate::sessions::session_service | GetSessionConfig, ListSessionsResponse, ListEventsResponse | <!-- stub --> |
| crate::sessions::state | State | <!-- stub --> |
| crate::sessions |  | <!-- stub --> |
| crate::state_manager |  | <!-- stub --> |
| crate::system::cli_orchestrator | Command | <!-- stub --> |
| crate::system::snapshot |  | <!-- stub --> |
| crate::system |  | <!-- stub --> |
| crate::telemetry |  | <!-- stub --> |
| crate::tools::base_tool |  | <!-- stub --> |
| crate::tools::builtin_tools |  | <!-- stub --> |
| crate::tools |  | <!-- stub --> |
| crate::tracing |  | <!-- stub --> |
| crate::trigger_loom::config | SymbolicRhythm, TriggerLoopConfig | <!-- stub --> |
| crate::trigger_loom::emotion |  | <!-- stub --> |
| crate::trigger_loom::emotional_state | EmotionalState | <!-- stub --> |
| crate::trigger_loom::engine | TriggerLoopEngine | <!-- stub --> |
| crate::trigger_loom::glyph_matcher | GlyphMatchResult | <!-- stub --> |
| crate::trigger_loom::loom |  | <!-- stub --> |
| crate::trigger_loom::recursion_control |  | <!-- stub --> |
| crate::trigger_loom::trigger_loop | SymbolicRhythm | <!-- stub --> |
| crate::trigger_loom |  | <!-- stub --> |
| crate::validator |  | <!-- stub --> |
| crate |  | <!-- stub --> |

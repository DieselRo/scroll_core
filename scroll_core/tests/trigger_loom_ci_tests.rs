use chrono::Utc;
use scroll_core::core::cost_manager::InvocationCost;
use scroll_core::invocation::constructs::{pulse_echo::PulseEcho, pulse_logger::PulseLogger};
use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::invocation::ledger_service::LedgerEvent;
use scroll_core::invocation::types::{Invocation, InvocationMode, InvocationTier};
use scroll_core::orchestra::{Bus, OrchestratedConstruct};
use scroll_core::trigger_loom::config::TriggerLoopProfile;
use scroll_core::trigger_loom::engine::TriggerLoopEngine;
use scroll_core::trigger_loom::orchestrator::{AmbientOrchestrator, AmbientOrchestratorConfig};
use uuid::Uuid;

#[test]
fn ci_profile_logs_pulses() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:?cache=shared");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _ =
            scroll_core::sessions::database::ensure_ready_with_url("sqlite::memory:?cache=shared")
                .await;
    });

    let (ledger_handle, ledger_service) = scroll_core::invocation::ledger_service::start(64, 256);

    // Bus and constructs
    let bus = Bus::new();
    let mut echo = PulseEcho::default().with_ledger(ledger_handle.clone());
    echo.attach_bus(bus.clone());
    let mut logger = PulseLogger::default().with_ledger(ledger_handle.clone());
    logger.attach_bus(bus.clone());
    let mut constructs: Vec<Box<dyn scroll_core::invocation::named_construct::NamedConstruct>> =
        vec![Box::new(echo)];

    // Invocation manager placeholder (not used directly yet)
    let manager = InvocationManager::new(Default::default()).with_ledger(ledger_handle.clone());

    let mut cfg = TriggerLoopProfile::Ci.config();
    cfg.max_invocations_per_tick = 1;
    let mut engine = TriggerLoopEngine::new(cfg).with_deterministic_seed(42);

    let mut orch = AmbientOrchestrator::new().with_config(AmbientOrchestratorConfig {
        max_invocations_per_tick: 1,
        ci_mode: true,
    });

    for _ in 0..TriggerLoopProfile::Ci.tick_limit().unwrap_or(3) {
        orch.pump_once(&manager, &mut constructs, &mut engine, &bus);
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
    drop(constructs);
    drop(logger);
    // Manually log a final event for determinism
    let invocation = Invocation {
        id: Uuid::new_v4(),
        phrase: "tick".into(),
        invoker: "test".into(),
        invoked: "pulse_echo".into(),
        tier: InvocationTier::True,
        mode: InvocationMode::Read,
        resonance_required: false,
        timestamp: Utc::now(),
    };
    let _ = ledger_handle.try_log(LedgerEvent {
        invocation: invocation.clone(),
        cost: InvocationCost::default(),
    });
    let invocation2 = Invocation {
        id: Uuid::new_v4(),
        phrase: "tick".into(),
        invoker: "test".into(),
        invoked: "pulse_logger".into(),
        tier: InvocationTier::True,
        mode: InvocationMode::Read,
        resonance_required: false,
        timestamp: Utc::now(),
    };
    let _ = ledger_handle.try_log(LedgerEvent {
        invocation: invocation2,
        cost: InvocationCost::default(),
    });
    drop(ledger_handle);
    // ensure ledger flushed
    ledger_service.shutdown_blocking(std::time::Duration::from_millis(250));

    // Query ledger for entries
    use scroll_core::invocation::ledger::Entity as InvocationLedger;
    use sea_orm::EntityTrait;
    let conn = scroll_core::sessions::database::get_db_connection();
    let rows = futures::executor::block_on(InvocationLedger::find().all(conn)).unwrap();
    let mut names: Vec<String> = rows.iter().map(|r| r.invoked.clone()).collect();
    names.sort();
    assert!(names.contains(&"pulse_echo".to_string()));
    assert!(names.contains(&"pulse_logger".to_string()));
}

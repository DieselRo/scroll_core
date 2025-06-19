// ===============================
// src/trigger_loom/engine.rs
// ===============================

use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use log::info;
use uuid::Uuid;

use crate::core::cost_manager::{CostDecision, CostManager};
use crate::invocation::invocation::{Invocation, InvocationMode, InvocationTier};
use crate::invocation::named_construct::{NamedConstruct, PulseSensitive};
use crate::trigger_loom::config::TriggerLoopConfig;

const MAX_AGENT_DEPTH: u32 = 5;

#[cfg(feature = "metrics")]
use metrics::histogram;

pub struct TriggerLoopEngine {
    config: TriggerLoopConfig,
    tick_counter: u64,
    agent_depth: HashMap<String, u32>,
}

impl TriggerLoopEngine {
    pub fn new(config: TriggerLoopConfig) -> Self {
        Self {
            config,
            tick_counter: 0,
            agent_depth: HashMap::new(),
        }
    }

    pub fn start_loop(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        let base_freq = self.config.resolve_frequency();
        let interval = Duration::from_secs_f32(1.0 / base_freq.max(0.001));

        loop {
            let now = Instant::now();
            self.tick_once(constructs);
            let elapsed = now.elapsed();
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    }

    pub fn tick_once(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        self.tick_counter += 1;

        #[cfg(feature = "metrics")]
        let tick_start = Instant::now();

        let start = Instant::now();
        let mut fired_count = 0usize;
        let mut skipped = 0usize;

        for construct in constructs.iter_mut() {
            if fired_count >= self.config.max_invocations_per_tick {
                break;
            }

            if let Some(pulse) = construct.as_pulse_sensitive() {
                if pulse.should_awaken(self.tick_counter) {
                    let invocation = Invocation {
                        id: Uuid::new_v4(),
                        phrase: "tick".into(),
                        invoker: "TriggerLoop".into(),
                        invoked: construct.name().into(),
                        tier: InvocationTier::True,
                        mode: InvocationMode::Read,
                        resonance_required: false,
                        timestamp: Utc::now(),
                    };
                    let cost = CostManager::assess(&invocation, &[]);
                    match cost.decision {
                        CostDecision::Allow => {
                            let _ = construct.perform(&invocation, None);
                            let depth = self
                                .agent_depth
                                .entry(construct.name().to_string())
                                .or_insert(0);
                            *depth += 1;
                            if *depth >= MAX_AGENT_DEPTH {
                                break;
                            }
                            fired_count += 1;
                        }
                        CostDecision::Throttle(_) => {
                            skipped += 1;
                        }
                        CostDecision::Reject(_) => {
                            info!("⏸️ rejected {} (cost)", construct.name());
                            skipped += 1;
                        }
                    }
                } else {
                    skipped += 1;
                }
            } else {
                skipped += 1;
            }
        }

        let duration = start.elapsed().as_millis();
        let summary = serde_json::json!({
            "fired": fired_count,
            "skipped": skipped,
            "duration_ms": duration
        });
        info!("{}", summary.to_string());

        #[cfg(feature = "metrics")]
        histogram!("tick_duration_ms").record(tick_start.elapsed().as_millis() as f64);
    }
}

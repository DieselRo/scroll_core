// ===============================
// src/trigger_loom/engine.rs
// ===============================

use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use log::info;
use uuid::Uuid;

use crate::core::cost_manager::{CostDecision, CostManager, InvocationCost};
use crate::invocation::named_construct::NamedConstruct;
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use crate::trigger_loom::config::TriggerLoopConfig;
use crate::trigger_loom::decision_ledger;
use crate::trigger_loom::emotional_state::EmotionalState;

const MAX_AGENT_DEPTH: u32 = 5;

#[cfg(feature = "metrics")]
use metrics::histogram;

pub struct TriggerLoopEngine {
    config: TriggerLoopConfig,
    tick_counter: u64,
    agent_depth: HashMap<String, u32>,
    deterministic_seed: Option<u64>,
    // Local runtime for async DB logging; created lazily
    rt: Option<tokio::runtime::Runtime>,
}

impl TriggerLoopEngine {
    pub fn new(config: TriggerLoopConfig) -> Self {
        Self {
            config,
            tick_counter: 0,
            agent_depth: HashMap::new(),
            deterministic_seed: None,
            rt: None,
        }
    }

    pub fn with_deterministic_seed(mut self, seed: u64) -> Self {
        self.deterministic_seed = Some(seed);
        self
    }

    pub fn start_loop(
        &mut self,
        constructs: &mut [Box<dyn NamedConstruct>],
        tick_limit: Option<u64>,
    ) {
        let base_freq = self.config.resolve_frequency();
        let interval = Duration::from_secs_f32(1.0 / base_freq.max(0.001));

        loop {
            if let Some(limit) = tick_limit {
                if self.tick_counter >= limit {
                    break;
                }
            }
            let now = Instant::now();
            self.tick_once(constructs);
            let elapsed = now.elapsed();
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    }

    pub fn start_loop_with_emotion(
        &mut self,
        constructs: &mut [Box<dyn NamedConstruct>],
        mut emotion: EmotionalState,
        tick_limit: Option<u64>,
    ) {
        loop {
            if let Some(limit) = tick_limit {
                if self.tick_counter >= limit {
                    break;
                }
            }
            let tick_start = Instant::now();
            // Decay before tick; simple modulation using intensity
            emotion.decay_step();
            let freq = match &self.config.rhythm {
                crate::trigger_loom::config::SymbolicRhythm::EmotionDriven => {
                    let intensity = emotion.intensity.clamp(0.0, 1.0);
                    // base 1Hz scaled by intensity [0.1, 2.0]
                    0.1 + intensity * 1.9
                }
                crate::trigger_loom::config::SymbolicRhythm::Constant(hz) => *hz,
                _ => self.config.resolve_frequency(),
            };
            let interval = Duration::from_secs_f32(1.0 / freq.max(0.001));
            self.tick_once_with_emotion(constructs, Some(&emotion));
            let elapsed = tick_start.elapsed();
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    }

    pub fn tick_once(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        self.tick_once_with_emotion(constructs, None);
    }

    pub fn tick_once_with_emotion(
        &mut self,
        constructs: &mut [Box<dyn NamedConstruct>],
        emotion: Option<&EmotionalState>,
    ) {
        self.tick_counter += 1;

        #[cfg(feature = "metrics")]
        let tick_start = Instant::now();

        let start = Instant::now();
        let mut fired_count = 0usize;
        let mut skipped = 0usize;

        // Create tick row (best-effort)
        let tick_id = {
            // lazy init runtime
            if self.rt.is_none() {
                self.rt = Some(tokio::runtime::Runtime::new().expect("ledger runtime"));
            }
            let rt = self.rt.as_ref().unwrap();
            let emotions_json = emotion.map(|e| {
                serde_json::json!({
                    "intensity": e.intensity,
                    "mood_trace": e.mood_trace,
                    "sigil_hint": e.sigil_hint,
                    "timestamp": e.timestamp,
                })
            });
            let budget_in = self.config.max_invocations_per_tick as i32;
            rt.block_on(decision_ledger::insert_tick(
                self.tick_counter as i64,
                Utc::now(),
                emotions_json,
                budget_in,
            ))
            .unwrap_or(uuid::Uuid::nil())
        };

        for construct in constructs.iter_mut() {
            if fired_count >= self.config.max_invocations_per_tick {
                // Log budget exhaustion against the next considered construct and stop
                if let Some(rt) = &self.rt {
                    let _ = rt.block_on(decision_ledger::insert_decision(
                        tick_id,
                        construct.name(),
                        "BudgetExceeded",
                        0,
                        0,
                    ));
                }
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
                    let cost = CostManager::assess(&invocation, &[]).unwrap_or_else(|e| {
                        eprintln!("metric error: {e:?}");
                        InvocationCost::default()
                    });
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
                            // Log allow decision
                            if let Some(rt) = &self.rt {
                                let _ = rt.block_on(decision_ledger::insert_decision(
                                    tick_id,
                                    construct.name(),
                                    "Allow",
                                    cost.context.token_estimate as i32,
                                    (self.config.max_invocations_per_tick - (fired_count + 1))
                                        as i32,
                                ));
                            }
                            fired_count += 1;
                        }
                        CostDecision::Throttle(_) => {
                            if let Some(rt) = &self.rt {
                                let _ = rt.block_on(decision_ledger::insert_decision(
                                    tick_id,
                                    construct.name(),
                                    "Throttle",
                                    cost.context.token_estimate as i32,
                                    (self.config.max_invocations_per_tick - fired_count) as i32,
                                ));
                            }
                            skipped += 1;
                        }
                        CostDecision::Reject(_) => {
                            info!("⏸️ rejected {} (cost)", construct.name());
                            if let Some(rt) = &self.rt {
                                let _ = rt.block_on(decision_ledger::insert_decision(
                                    tick_id,
                                    construct.name(),
                                    "Reject",
                                    cost.context.token_estimate as i32,
                                    (self.config.max_invocations_per_tick - fired_count) as i32,
                                ));
                            }
                            skipped += 1;
                        }
                    }
                } else {
                    if let Some(rt) = &self.rt {
                        let _ = rt.block_on(decision_ledger::insert_decision(
                            tick_id,
                            construct.name(),
                            "Cooldown",
                            0,
                            (self.config.max_invocations_per_tick - fired_count) as i32,
                        ));
                    }
                    skipped += 1;
                }
            } else {
                if let Some(rt) = &self.rt {
                    let _ = rt.block_on(decision_ledger::insert_decision(
                        tick_id,
                        construct.name(),
                        "NotPulseSensitive",
                        0,
                        (self.config.max_invocations_per_tick - fired_count) as i32,
                    ));
                }
                skipped += 1;
            }
        }

        let duration = start.elapsed().as_millis();
        let summary = serde_json::json!({
            "fired": fired_count,
            "skipped": skipped,
            "duration_ms": duration
        });
        info!("{}", summary);

        // Update tick budget_out (best-effort)
        if let Some(rt) = &self.rt {
            let _ = rt.block_on(decision_ledger::update_tick_budget_out(
                tick_id,
                (self.config.max_invocations_per_tick - fired_count) as i32,
            ));
        }

        #[cfg(feature = "metrics")]
        histogram!("tick_duration_ms").record(tick_start.elapsed().as_millis() as f64);
    }
}

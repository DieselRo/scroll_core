#![allow(clippy::missing_const_for_thread_local)]
// cost_manager.rs – The Core Weave
//====================================

use crate::errors::MetricError;
use crate::invocation::types::Invocation;
use crate::metrics::clamp_finite;
use crate::scroll::Scroll;
use crate::models::model_registry::{ModelRegistry, ThresholdCostProfile as RegistryCostProfile};

#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
thread_local! {
    static TEST_DECISION: RefCell<Option<CostDecision>> = RefCell::new(None);
}

#[cfg(test)]
pub fn set_test_decision(decision: Option<CostDecision>) {
    TEST_DECISION.with(|d| *d.borrow_mut() = decision);
}

#[derive(Debug, Clone)]
pub enum CostDecision {
    Allow,
    Throttle(f32), // throttle intensity 0.0 - 1.0
    Reject(String),
}

#[derive(Debug, Clone)]
pub enum RejectionOrigin {
    System,
    Construct(String),
}

#[derive(Debug, Clone)]
pub struct InvocationCost {
    pub context: ContextCost,
    pub system: SystemCost,
    pub decision: CostDecision,
    pub cost_profile: CostProfile,
    pub rejection_origin: Option<RejectionOrigin>,
    pub hesitation_signal: Option<String>,
    pub poetic_rejection: Option<String>,
    pub symbolic_echo: Option<String>,
    pub emotion_tension: Option<f32>,
}

impl Default for InvocationCost {
    fn default() -> Self {
        Self {
            context: ContextCost {
                token_estimate: 0,
                context_span: 0,
                relevance_score: 0.0,
            },
            system: SystemCost {
                cpu_cycles: 0.0,
                memory_used_mb: 0.0,
                io_ops: 0,
                scrolls_touched: 0,
            },
            decision: CostDecision::Allow,
            cost_profile: CostProfile {
                system_pressure: 0.0,
                token_pressure: 0.0,
                symbolic_origin: None,
            },
            rejection_origin: None,
            hesitation_signal: None,
            poetic_rejection: None,
            symbolic_echo: None,
            emotion_tension: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CostProfile {
    pub system_pressure: f32,
    pub token_pressure: f32,
    pub symbolic_origin: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContextCost {
    pub token_estimate: usize,
    pub context_span: usize,
    pub relevance_score: f32,
}

#[derive(Debug, Clone)]
pub struct SystemCost {
    pub cpu_cycles: f64,
    pub memory_used_mb: f64,
    pub io_ops: usize,
    pub scrolls_touched: usize,
}

pub trait ContextScorer {
    fn score(
        &self,
        invocation: &Invocation,
        scrolls: &[Scroll],
        semantic_score: f32,
    ) -> Result<f32, MetricError>;
}

pub struct SemanticContextScorer;

impl ContextScorer for SemanticContextScorer {
    fn score(
        &self,
        _invocation: &Invocation,
        scrolls: &[Scroll],
        semantic_score: f32,
    ) -> Result<f32, MetricError> {
        if scrolls.is_empty() {
            return Ok(0.0);
        }

        let semantic_score = clamp_finite(semantic_score as f64)? as f32;
        let relevance = normalize_distance(semantic_score);

        let now = chrono::Utc::now();
        let recency: f32 = scrolls
            .iter()
            .map(|s| {
                let elapsed = now
                    .signed_duration_since(s.origin.last_modified)
                    .num_seconds()
                    .max(1) as f32;
                1.0 / elapsed.log2()
            })
            .sum::<f32>()
            / scrolls.len() as f32;

        let importance: f32 = scrolls
            .iter()
            .map(|s| s.emotion_signature.intensity.unwrap_or(0.5))
            .sum::<f32>()
            / scrolls.len() as f32;

        let result =
            (relevance.clamp(0.0, 1.0)) * (recency.clamp(0.0, 1.0)) * (importance.clamp(0.0, 1.0));

        Ok(result)
    }
}

fn normalize_distance(distance: f32) -> f32 {
    let norm = 1.0 / (1.0 + distance.abs());
    norm.clamp(0.0, 1.0)
}

pub struct CostManager;

impl CostManager {
    pub fn calculate_cost_profile(
        context: &ContextCost,
        system: &SystemCost,
    ) -> Result<CostProfile, MetricError> {
        clamp_finite(context.relevance_score as f64)?;
        clamp_finite(system.cpu_cycles)?;
        clamp_finite(system.memory_used_mb)?;

        let token_pressure = (context.token_estimate as f32 * 0.001
            + context.context_span as f32 * 0.1
            - context.relevance_score * 0.3)
            .max(0.0);

        let system_pressure = (system.cpu_cycles * 100.0
            + system.memory_used_mb * 0.25
            + system.io_ops as f64 * 0.05
            + system.scrolls_touched as f64 * 0.2)
            .max(0.0);

        Ok(CostProfile {
            system_pressure: system_pressure as f32,
            token_pressure,
            symbolic_origin: None,
        })
    }

    pub fn assess(
        _invocation: &Invocation,
        scrolls: &[Scroll],
    ) -> Result<InvocationCost, MetricError> {
        #[cfg(test)]
        if let Some(decision) = TEST_DECISION.with(|d| d.borrow().clone()) {
            return Ok(InvocationCost {
                context: ContextCost {
                    token_estimate: 0,
                    context_span: 0,
                    relevance_score: 0.0,
                },
                system: SystemCost {
                    cpu_cycles: 0.0,
                    memory_used_mb: 0.0,
                    io_ops: 0,
                    scrolls_touched: 0,
                },
                decision,
                cost_profile: CostProfile {
                    system_pressure: 0.0,
                    token_pressure: 0.0,
                    symbolic_origin: None,
                },
                rejection_origin: None,
                hesitation_signal: None,
                poetic_rejection: None,
                symbolic_echo: None,
                emotion_tension: None,
            });
        }
        let token_estimate = scrolls.iter().map(|s| s.markdown_body.len() / 4).sum();
        let scorer = SemanticContextScorer;
        let relevance_score = scorer.score(_invocation, scrolls, 0.5)?;

        let context = ContextCost {
            token_estimate,
            context_span: scrolls.len(),
            relevance_score,
        };

        let system = SystemCost {
            cpu_cycles: 0.0023,
            memory_used_mb: 3.2,
            io_ops: 7,
            scrolls_touched: scrolls.len(),
        };

        // Base decision (existing behavior)
        let mut decision = if context.token_estimate > 12000 {
            CostDecision::Reject("Context window too large.".to_string())
        } else {
            CostDecision::Allow
        };

        // Enforce configured cost thresholds if registry is available
        if let Some(reg) = ModelRegistry::get_global() {
            let profile = reg.cost_profile(&_invocation.invoked);
            if let Some(limit) = profile.per_request_usd_limit {
                if let Some(estimated) = estimate_usd(&_invocation.invoked, &context) {
                    if estimated > limit {
                        decision = CostDecision::Reject(format!("Per-request cost estimate ${:.4} exceeds limit ${:.4}", estimated, limit));
                    }
                }
            }
            if let Some(cap) = profile.daily_usd_cap {
                if let Some(estimated) = estimate_usd(&_invocation.invoked, &context) {
                    if !RollingCap::check_and_add(&_invocation.invoked, estimated) {
                        decision = CostDecision::Reject("Daily cost cap reached".into());
                    }
                }
            }
        }

        let cost_profile = Self::calculate_cost_profile(&context, &system)?;

        Ok(InvocationCost {
            context,
            system,
            decision: decision.clone(),
            rejection_origin: if matches!(decision, CostDecision::Reject(_)) {
                Some(RejectionOrigin::System)
            } else {
                None
            },
            hesitation_signal: match decision {
                CostDecision::Allow => None,
                _ => Some("The archive paused, uncertain.".to_string()),
            },
            poetic_rejection: Some("A whisper lost in the tide of memory.".to_string()),
            symbolic_echo: Some("The loom remained still.".to_string()),
            emotion_tension: Some(0.82),
            cost_profile,
        })
    }
}

use std::collections::VecDeque;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ROLLING_STATE: Lazy<Mutex<std::collections::HashMap<String, VecDeque<(i64, f32)>>>> = Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

struct RollingCap;

impl RollingCap {
    // returns true if under cap after adding this amount (uses 24h window)
    fn check_and_add(name: &str, amount: f32) -> bool {
        let now = chrono::Utc::now().timestamp();
        let mut map = ROLLING_STATE.lock().unwrap();
        let q = map.entry(name.to_string()).or_insert_with(VecDeque::new);
        // drop entries older than 24h
        let cutoff = now - 86_400;
        while let Some(front) = q.front() {
            if front.0 < cutoff { q.pop_front(); } else { break; }
        }
        let sum: f32 = q.iter().map(|(_, v)| *v).sum();
        // look up cap for this construct
        let cap = ModelRegistry::get_global()
            .map(|r| r.cost_profile(name).daily_usd_cap)
            .flatten();
        if let Some(cap) = cap {
            if sum + amount > cap { return false; }
        }
        q.push_back((now, amount));
        true
    }
}

fn estimate_usd(construct: &str, context: &ContextCost) -> Option<f32> {
    // Estimate cost using optional pricing hints from registry.extra
    let reg = ModelRegistry::get_global()?;
    let spec = reg.by_construct(construct).ok()?;
    let input_price = spec.extra.get("input_per_1k_usd").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let output_price = spec.extra.get("output_per_1k_usd").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    if input_price == 0.0 && output_price == 0.0 { return None; }
    let in_cost = (context.token_estimate as f32 / 1000.0) * input_price;
    // We don't know output tokens; approximate as 50% of input or bounded by max_output_tokens
    let approx_out_tokens = (context.token_estimate as f32 * 0.5)
        .min(ModelRegistry::get_global().and_then(|r| r.by_construct(construct).ok()?.max_output_tokens.map(|m| m as f32)).unwrap_or(f32::MAX));
    let out_cost = (approx_out_tokens / 1000.0) * output_price;
    Some(in_cost + out_cost)
}

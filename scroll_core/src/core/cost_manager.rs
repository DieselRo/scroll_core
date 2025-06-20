#![allow(clippy::missing_const_for_thread_local)]
// cost_manager.rs – The Core Weave
//====================================

use crate::errors::MetricError;
use crate::invocation::types::Invocation;
use crate::metrics::clamp_finite;
use crate::scroll::Scroll;

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

        let decision = if context.token_estimate > 12000 {
            CostDecision::Reject("Context window too large.".to_string())
        } else {
            CostDecision::Allow
        };

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

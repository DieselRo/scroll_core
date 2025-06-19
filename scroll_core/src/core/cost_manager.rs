// cost_manager.rs – The Core Weave
//====================================
#![allow(unused_imports)]

use crate::invocation::invocation::Invocation;
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
    fn score(&self, invocation: &Invocation, scrolls: &[Scroll]) -> f32;
}

pub struct CostManager;

impl CostManager {
    pub fn calculate_cost_profile(context: &ContextCost, system: &SystemCost) -> CostProfile {
        let token_pressure = context.token_estimate as f32 * 0.001
            + context.context_span as f32 * 0.1
            - context.relevance_score * 0.3;

        let system_pressure = system.cpu_cycles * 100.0
            + system.memory_used_mb * 0.25
            + system.io_ops as f64 * 0.05
            + system.scrolls_touched as f64 * 0.2;

        CostProfile {
            system_pressure: system_pressure as f32,
            token_pressure,
            symbolic_origin: None,
        }
    }

    pub fn assess(_invocation: &Invocation, scrolls: &[Scroll]) -> InvocationCost {
        #[cfg(test)]
        if let Some(decision) = TEST_DECISION.with(|d| d.borrow().clone()) {
            return InvocationCost {
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
            };
        }
        let token_estimate = scrolls.iter().map(|s| s.markdown_body.len() / 4).sum();
        let relevance_score = 0.75; // Placeholder — consider future ContextScorer

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

        let cost_profile = Self::calculate_cost_profile(&context, &system);

        InvocationCost {
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
        }
    }
}

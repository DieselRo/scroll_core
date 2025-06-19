use quickcheck::{QuickCheck, TestResult};
use scroll_core::core::cost_manager::{CostManager, ContextCost, SystemCost};

#[test]
fn cost_profile_never_negative() {
    fn prop(token_est: u16, span: u8, relevance: f32, cpu: f64, mem: f64, io: u16, touched: u8) -> TestResult {
        if !cpu.is_finite() || !mem.is_finite() || !relevance.is_finite() {
            return TestResult::discard();
        }

        let context = ContextCost {
            token_estimate: (token_est as usize) % 20000,
            context_span: (span as usize) % 100,
            relevance_score: (relevance.abs() % 1.0_f32),
        };
        let system = SystemCost {
            cpu_cycles: cpu.abs() % 1000.0,
            memory_used_mb: mem.abs() % 1024.0,
            io_ops: io as usize,
            scrolls_touched: touched as usize,
        };
        let profile = CostManager::calculate_cost_profile(&context, &system);
        if !profile.token_pressure.is_finite() || profile.token_pressure < 0.0 {
            return TestResult::failed();
        }
        if !profile.system_pressure.is_finite() || profile.system_pressure < 0.0 {
            return TestResult::failed();
        }
        TestResult::passed()
    }

    QuickCheck::new()
        .tests(500)
        .max_tests(10000)
        .quickcheck(prop as fn(u16, u8, f32, f64, f64, u16, u8) -> TestResult);
}

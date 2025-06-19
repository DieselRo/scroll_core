use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
use scroll_core::clamp_finite;
use scroll_core::core::cost_manager::{ContextCost, CostManager, SystemCost};

#[derive(Clone, Debug)]
struct FiniteF64(f64);

impl Arbitrary for FiniteF64 {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {
            let v = f64::arbitrary(g);
            if v.is_finite() {
                return FiniteF64(v);
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().filter(|v| v.is_finite()).map(FiniteF64))
    }
}

#[derive(Clone, Debug)]
struct FiniteF32(f32);

impl Arbitrary for FiniteF32 {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {
            let v = f32::arbitrary(g);
            if v.is_finite() {
                return FiniteF32(v);
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().filter(|v| v.is_finite()).map(FiniteF32))
    }
}

#[test]
fn cost_profile_never_negative() {
    fn prop(
        token_est: u16,
        span: u8,
        relevance: FiniteF32,
        cpu: FiniteF64,
        mem: FiniteF64,
        io: u16,
        touched: u8,
    ) -> TestResult {
        let cpu = match clamp_finite(cpu.0) {
            Ok(v) => v,
            Err(e) => return TestResult::error(format!("cpu error: {e:?}")),
        };
        let mem = match clamp_finite(mem.0) {
            Ok(v) => v,
            Err(e) => return TestResult::error(format!("mem error: {e:?}")),
        };
        let relevance = relevance.0;

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
        let profile = match CostManager::calculate_cost_profile(&context, &system) {
            Ok(p) => p,
            Err(e) => return TestResult::error(format!("profile error: {e:?}")),
        };
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
        .quickcheck(prop as fn(u16, u8, FiniteF32, FiniteF64, FiniteF64, u16, u8) -> TestResult);
}

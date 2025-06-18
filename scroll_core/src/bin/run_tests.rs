// ==================================
// src/bin/run_tests.rs
// ==================================

//! A simple binary to run and verify the ADK tests

fn main() {
    println!("Running ADK tests...");
    println!("This is just a helper binary. To actually run the tests, use:");
    println!("  cargo test --lib adk::tests");
    println!();
    println!("Or to run specific test modules:");
    println!("  cargo test --lib adk::tests::session_tests");
    println!("  cargo test --lib adk::tests::agent_tests");
    println!("  cargo test --lib adk::tests::tool_tests");
    println!("  cargo test --lib adk::tests::runner_tests");
    println!("  cargo test --lib adk::tests::integration_tests");
    println!();
    println!("For more verbose output, add the --nocapture flag:");
    println!("  cargo test --lib adk::tests -- --nocapture");
}
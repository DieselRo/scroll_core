// ===============================
// src/trigger_loom/recursion_control.rs
// ===============================

use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::invocation::invocation::{Invocation, InvocationTier};

fn trace_file() -> PathBuf {
    std::env::temp_dir().join("recursion_trace.log")
}

pub fn should_recurse(tier: &InvocationTier) -> bool {
    matches!(tier, InvocationTier::Calling | InvocationTier::Whispered)
}

pub fn mark_cycle(invocation: &Invocation) {
    let path = trace_file();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(file, "{}", invocation.id);
    }
    println!("🔁 Recursion marked: {}", invocation.phrase);
}

pub fn recover_trace() -> Option<String> {
    let path = trace_file();
    if let Ok(contents) = read_to_string(&path) {
        contents.lines().last().map(|s| s.to_string())
    } else {
        None
    }
}

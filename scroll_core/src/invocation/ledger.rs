//! Records invocation activity to a plain text log for later review.
//! This module is used by the AelrenHerald and InvocationManager to track history.
//! See [Thiren](../AGENTS.md#thiren) for future audit enhancements.
// ===============================
// src/ledger.rs
// ===============================

use crate::invocation::types::Invocation;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn log_invocation<P: AsRef<Path>>(path: P, invocation: &Invocation) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(
        file,
        "{:?} | invoked: {} | mode: {:?} | tier: {:?}",
        invocation.timestamp, invocation.invoked, invocation.mode, invocation.tier
    )
}

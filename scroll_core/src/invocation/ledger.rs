// ===============================
// src/ledger.rs
// ===============================
#![allow(unused_imports)]

use crate::invocation::invocation_core::Invocation;
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

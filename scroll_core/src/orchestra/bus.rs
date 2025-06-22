//! Lightweight message bus used by constructs to pass AgentMessages.
//! Channels are registered by name so constructs can discover each other at runtime.
//! See [Orchestra](../AGENTS.md#invocationmanager) for overall communication flow.
// src/orchestra/bus.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::orchestra::message::AgentMessage;

#[derive(Clone, Default)]
pub struct Bus {
    inner: Arc<Mutex<HashMap<String, Sender<AgentMessage>>>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&mut self, name: &str) -> Receiver<AgentMessage> {
        let (tx, rx) = unbounded();
        self.inner.lock().unwrap().insert(name.to_string(), tx);
        rx
    }

    pub fn send(&self, msg: AgentMessage) {
        let targets = {
            let guard = self.inner.lock().unwrap();
            guard
                .iter()
                .filter(|(n, _)| msg.to == "broadcast" || msg.to == **n)
                .map(|(_, tx)| tx.clone())
                .collect::<Vec<_>>()
        };
        for tx in targets {
            let _ = tx.send(msg.clone());
        }
    }
}

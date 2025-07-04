//! The FileReader construct loads scroll files from disk for other constructs to operate on.
//! It streams the file contents over the orchestrator bus to avoid blocking invocation flows.
//! See [FileReader](../../../AGENTS.md#filereader) for the high level design.
// src/invocation/constructs/file_reader_construct.rs

use crate::invocation::named_construct::NamedConstruct;
use crate::invocation::types::{Invocation, InvocationResult};
use crate::orchestra::{AgentMessage, Bus, OrchestratedConstruct};
use serde_json::json;
use std::thread;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(Default)]
pub struct FileReader {
    bus: Option<Bus>,
}

impl NamedConstruct for FileReader {
    fn name(&self) -> &str {
        "filereader"
    }

    fn perform(
        &self,
        _invocation: &Invocation,
        _scroll: Option<crate::Scroll>,
    ) -> Result<InvocationResult, String> {
        Ok(InvocationResult::Success(Box::from("ready")))
    }
}

impl OrchestratedConstruct for FileReader {
    fn attach_bus(&mut self, mut bus: Bus) {
        let rx = bus.subscribe("filereader");
        self.bus = Some(bus.clone());
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            while let Ok(msg) = rx.recv() {
                let path = msg.payload["path"].as_str().unwrap_or("").to_string();
                let to = msg.from.clone();
                let mut trace = msg.trace.clone();
                trace.push("filereader".into());
                let bus_clone = bus.clone();
                let res = rt.block_on(tokio::task::spawn_blocking(move || {
                    std::fs::read_to_string(path).unwrap_or_default()
                }));
                if let Ok(contents) = res {
                    let reply = AgentMessage {
                        id: Uuid::new_v4(),
                        from: "filereader".into(),
                        to,
                        payload: json!({"file_contents": contents}),
                        trace,
                    };
                    bus_clone.send(reply);
                }
            }
        });
    }
}

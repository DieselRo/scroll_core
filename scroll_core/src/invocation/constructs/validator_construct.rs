// ===============================
// src/constructs/validator_construct.rs
// ===============================

use crate::invocation::named_construct::NamedConstruct;
use crate::invocation::types::{Invocation, InvocationMode, InvocationResult};
use crate::orchestra::{AgentMessage, Bus, OrchestratedConstruct};
use crate::scroll::Scroll;
use crate::validator::validate_scroll;
use serde_json::json;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

#[derive(Default)]
pub struct Validator {
    bus: Option<Bus>,
}

impl NamedConstruct for Validator {
    fn name(&self) -> &str {
        "Validator"
    }

    fn perform(
        &self,
        invocation: &Invocation,
        scroll: Option<Scroll>,
    ) -> Result<InvocationResult, String> {
        match invocation.mode {
            InvocationMode::Validate => {
                if let Some(scroll) = scroll {
                    validate_scroll(&scroll.yaml_metadata)?;
                    Ok(InvocationResult::Success(Box::from("Validation passed.")))
                } else {
                    Err("No scroll provided to validate.".into())
                }
            }
            _ => Err("Validator only supports Validate invocation mode.".into()),
        }
    }
}

impl OrchestratedConstruct for Validator {
    fn attach_bus(&mut self, mut bus: Bus) {
        let rx = bus.subscribe("validator");
        self.bus = Some(bus.clone());
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                let text = msg.payload["text"].as_str().unwrap_or("");
                let path = text.split_whitespace().last().unwrap_or("");
                let mut trace = msg.trace.clone();
                trace.push("validator".into());
                let mut bus_clone = bus.clone();

                let result = match crate::parser::parse_scroll_from_file(path) {
                    Ok(scroll) => {
                        if let Some(file_path) = scroll.yaml_metadata.file_path.clone() {
                            let fr_rx = bus_clone.subscribe("validator");
                            let fr_msg = AgentMessage {
                                id: Uuid::new_v4(),
                                from: "validator".into(),
                                to: "filereader".into(),
                                payload: json!({"path": file_path}),
                                trace: trace.clone(),
                            };
                            bus_clone.send(fr_msg);
                            match fr_rx.recv_timeout(Duration::from_millis(100)) {
                                Ok(reply) => {
                                    if let Some(contents) = reply.payload["file_contents"].as_str()
                                    {
                                        match serde_yaml::from_str::<crate::YamlMetadata>(contents)
                                        {
                                            Ok(meta) => validate_scroll(&meta)
                                                .map(|_| "Validation passed.".to_string())
                                                .unwrap_or_else(|_| "invalid YAML".to_string()),
                                            Err(_) => "invalid YAML".to_string(),
                                        }
                                    } else {
                                        "invalid YAML".to_string()
                                    }
                                }
                                Err(_) => "FileReader timeout".to_string(),
                            }
                        } else {
                            validate_scroll(&scroll.yaml_metadata)
                                .map(|_| "Validation passed.".to_string())
                                .unwrap_or_else(|_| "invalid YAML".to_string())
                        }
                    }
                    Err(_) => "invalid YAML".to_string(),
                };

                let reply = AgentMessage {
                    id: Uuid::new_v4(),
                    from: "validator".into(),
                    to: msg.from.clone(),
                    payload: json!({"text": result}),
                    trace,
                };
                bus_clone.send(reply);
            }
        });
    }
}

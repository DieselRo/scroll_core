use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use scroll_core::invocation::invocation::{Invocation, InvocationResult};
use scroll_core::invocation::named_construct::NamedConstruct;
use scroll_core::orchestra::{AgentMessage, Bus, OrchestratedConstruct};
use uuid::Uuid;

#[derive(Default)]
struct Echo {
    bus: Option<Bus>,
}

impl NamedConstruct for Echo {
    fn name(&self) -> &str {
        "echo"
    }
    fn perform(
        &self,
        _invocation: &Invocation,
        _scroll: Option<scroll_core::Scroll>,
    ) -> Result<InvocationResult, String> {
        if let Some(bus) = &self.bus {
            let msg = AgentMessage {
                id: Uuid::new_v4(),
                from: "echo".into(),
                to: "sink".into(),
                payload: serde_json::json!({"text": "ping"}),
                trace: vec!["echo".into()],
            };
            bus.send(msg);
        }
        Ok(InvocationResult::Success("sent".into()))
    }
}

impl OrchestratedConstruct for Echo {
    fn attach_bus(&mut self, bus: Bus) {
        self.bus = Some(bus);
    }
}

#[derive(Default)]
struct Sink {
    bus: Option<Bus>,
    received: Arc<Mutex<Vec<String>>>,
}

impl NamedConstruct for Sink {
    fn name(&self) -> &str {
        "sink"
    }
    fn perform(
        &self,
        _invocation: &Invocation,
        _scroll: Option<scroll_core::Scroll>,
    ) -> Result<InvocationResult, String> {
        Ok(InvocationResult::Success("ok".into()))
    }
}

impl OrchestratedConstruct for Sink {
    fn attach_bus(&mut self, mut bus: Bus) {
        let rx = bus.subscribe("sink");
        self.bus = Some(bus);
        let store = self.received.clone();
        thread::spawn(move || {
            if let Ok(msg) = rx.recv() {
                store
                    .lock()
                    .unwrap()
                    .push(msg.payload["text"].as_str().unwrap().to_string());
            }
        });
    }
}

#[test]
fn test_whispering_bus() {
    let mut bus = Bus::new();
    let mut echo = Echo::default();
    let mut sink = Sink {
        received: Arc::new(Mutex::new(Vec::new())),
        ..Default::default()
    };
    let received = sink.received.clone();

    echo.attach_bus(bus.clone());
    sink.attach_bus(bus.clone());

    let ctx = scroll_core::Scroll {
        id: Uuid::new_v4(),
        title: "d".into(),
        scroll_type: scroll_core::ScrollType::Echo,
        yaml_metadata: scroll_core::YamlMetadata {
            title: "d".into(),
            scroll_type: scroll_core::ScrollType::Echo,
            emotion_signature: scroll_core::EmotionSignature::default(),
            last_modified: None,
            tags: vec![],
        },
        markdown_body: String::new(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: scroll_core::ScrollStatus::Draft,
        origin: scroll_core::ScrollOrigin {
            created: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
            authored_by: None,
        },
        emotion_signature: scroll_core::EmotionSignature::default(),
        linked_scrolls: vec![],
    };
    let invocation = Invocation {
        id: Uuid::new_v4(),
        phrase: "run".into(),
        invoker: "test".into(),
        invoked: "echo".into(),
        tier: scroll_core::invocation::invocation::InvocationTier::True,
        mode: scroll_core::invocation::invocation::InvocationMode::Read,
        resonance_required: false,
        timestamp: chrono::Utc::now(),
    };
    let _ = echo.perform(&invocation, Some(ctx));

    thread::sleep(Duration::from_millis(50));

    assert_eq!(received.lock().unwrap().as_slice(), ["ping"]);
}

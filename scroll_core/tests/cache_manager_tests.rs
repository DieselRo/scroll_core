use chrono::{Duration, Utc};
use scroll_core::archive::initialize::load_with_cache;
use scroll_core::archive::scroll_access_log::ScrollAccess;
use scroll_core::cache_manager::CacheManager;
use scroll_core::core::cost_manager::{
    ContextCost, CostDecision, CostProfile, InvocationCost, SystemCost,
};
use scroll_core::{EmotionSignature, Scroll, ScrollOrigin, ScrollStatus, ScrollType, YamlMetadata};
use std::path::Path;
use uuid::Uuid;

fn dummy_scroll(intensity: f32) -> Scroll {
    Scroll {
        id: Uuid::new_v4(),
        title: "Test".into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata {
            title: "Test".into(),
            scroll_type: ScrollType::Canon,
            emotion_signature: EmotionSignature {
                tone: "calm".into(),
                emphasis: 0.5,
                resonance: "gentle".into(),
                intensity: Some(intensity),
            },
            tags: vec![],
            archetype: None,
            quorum_required: false,
            last_modified: None,
            file_path: None,
        },
        tags: vec![],
        archetype: None,
        quorum_required: false,
        markdown_body: "Body".into(),
        invocation_phrase: "Invoke".into(),
        sigil: "🔧".into(),
        status: ScrollStatus::Draft,
        emotion_signature: EmotionSignature {
            tone: "calm".into(),
            emphasis: 0.5,
            resonance: "gentle".into(),
            intensity: Some(intensity),
        },
        linked_scrolls: vec![],
        origin: ScrollOrigin {
            created: Utc::now(),
            authored_by: None,
            last_modified: Utc::now(),
        },
    }
}

fn zero_cost() -> InvocationCost {
    InvocationCost {
        context: ContextCost {
            token_estimate: 0,
            context_span: 0,
            relevance_score: 0.0,
        },
        system: SystemCost {
            cpu_cycles: 0.0,
            memory_used_mb: 0.0,
            io_ops: 0,
            scrolls_touched: 0,
        },
        decision: CostDecision::Allow,
        cost_profile: CostProfile {
            system_pressure: 0.0,
            token_pressure: 0.0,
            symbolic_origin: None,
        },
        rejection_origin: None,
        hesitation_signal: None,
        poetic_rejection: None,
        symbolic_echo: None,
        emotion_tension: None,
    }
}

#[test]
fn test_cache_initialized_matches_scrolls() {
    let path = Path::new("../tests/sample_scrolls");
    let (scrolls, cache) = load_with_cache(path).unwrap();
    assert_eq!(scrolls.len(), cache.count());
}

#[test]
fn test_cache_eviction_to_max_size() {
    let mut cache = CacheManager::new(50);
    let cost = zero_cost();
    let now = Utc::now();
    let mut first_id = Uuid::nil();
    for i in 0..55 {
        let scroll = dummy_scroll(i as f32);
        if i == 0 {
            first_id = scroll.id;
        }
        let access = ScrollAccess {
            first_accessed: now - Duration::seconds(100 - i as i64),
            last_accessed: now - Duration::seconds(100 - i as i64),
            access_count: 1,
        };
        cache.cache_scroll(scroll, &EmotionSignature::curious(), &access, &cost);
    }
    assert_eq!(cache.count(), 50);
    assert!(!cache.ids().contains(&first_id));
}

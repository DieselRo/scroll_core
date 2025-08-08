use scroll_core::trigger_loom::ambient::should_trigger;
use scroll_core::trigger_loom::emotional_state::EmotionalState;

#[test]
fn ambient_trigger_respects_intensity_and_tags() {
    let mood_low = EmotionalState::new(vec![], 0.2, None);
    let mood_high = EmotionalState::new(vec![], 0.9, None);
    let tags = vec!["core".to_string(), "myth".to_string()];

    assert!(!should_trigger(&tags, &mood_low, "core", 0.5));
    assert!(should_trigger(&tags, &mood_high, "core", 0.5));
    assert!(!should_trigger(&tags, &mood_high, "unknown", 0.5));
}

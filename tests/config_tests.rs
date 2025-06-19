use chrono::{Local, TimeZone};
use scroll_core::trigger_loom::config::{SymbolicRhythm, TriggerLoopConfig};
use scroll_core::trigger_loom::emotion::modulate_frequency;
use scroll_core::EmotionSignature;

#[test]
fn test_dawn_frequency_night() {
    let config = TriggerLoopConfig {
        rhythm: SymbolicRhythm::Dawn,
        max_invocations_per_tick: 1,
        allow_test_ticks: true,
        emotional_signature: None,
    };
    let late = Local.with_ymd_and_hms(2024, 1, 1, 23, 0, 0).unwrap();
    assert_eq!(config.resolve_frequency_at(late), 0.0);
}

#[test]
fn test_modulate_frequency_linear() {
    let cases = [
        (0.0, 0.1),
        (0.8, 1.7),
    ];
    for (intensity, expected) in cases {
        let sig = EmotionSignature {
            tone: "neutral".into(),
            emphasis: 0.0,
            resonance: "balanced".into(),
            intensity: Some(intensity),
        };
        let hz = modulate_frequency(1.0, &sig);
        assert!((hz - expected).abs() < 0.01, "{hz} != {expected}");
    }
}

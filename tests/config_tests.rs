use chrono::{Local, TimeZone};
use scroll_core::trigger_loom::config::{SymbolicRhythm, TriggerLoopConfig};

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

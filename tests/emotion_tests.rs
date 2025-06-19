use scroll_core::trigger_loom::emotional_state::EmotionalState;
use scroll_core::chat::chat_session::ChatMessage;

#[test]
fn test_update_from_message_smiley() {
    let mut state = EmotionalState::new(vec![], 0.5, None);
    let msg = ChatMessage { role: "user".into(), content: ":)".into(), emotion: None };
    state.update_from_message(&msg);
    assert!((state.intensity - 0.6).abs() < 0.001);
}

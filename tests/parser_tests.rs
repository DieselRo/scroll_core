use scroll_core::{parse_scroll, ScrollStatus};

#[test]
fn test_parse_valid_scroll() {
    let input = r#"
---
title: Test Scroll
scroll_type: Canon
emotion_signature:
  tone: Thoughtful
  emphasis: 0.7
  resonance: Subtle
---
# Markdown Body
This is the body of the scroll.
"#;
    let scroll = parse_scroll(input).unwrap();
    assert_eq!(scroll.title, "Test Scroll");
    assert_eq!(scroll.status, ScrollStatus::Draft);
}
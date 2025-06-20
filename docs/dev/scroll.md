# Scroll Reference

## Using ScrollBuilder

The `ScrollBuilder` provides a convenient way to construct new `Scroll` instances.

```rust
use scroll_core::{ScrollBuilder, ScrollType, EmotionSignature, YamlMetadata};

let scroll = ScrollBuilder {
    title: "Example".into(),
    scroll_type: ScrollType::Canon,
    yaml_metadata: YamlMetadata {
        title: "Example".into(),
        scroll_type: ScrollType::Canon,
        emotion_signature: EmotionSignature::neutral(),
        tags: vec![],
        archetype: None,
        quorum_required: false,
        last_modified: None,
        file_path: None,
    },
    tags: vec![],
    archetype: None,
    quorum_required: false,
    markdown_body: "body".into(),
    invocation_phrase: "Invoke".into(),
    sigil: "🔮".into(),
    emotion_signature: EmotionSignature::neutral(),
    authored_by: None,
}.build();
```

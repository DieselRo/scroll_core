// ===============================
// src/trigger_loom/glyph_matcher.rs
// ===============================

#[derive(Debug, Clone)]
pub enum GlyphMatchResult {
    Exact,
    Near(String),
    Miss,
}

pub fn match_glyph(signal: &str, known_glyphs: &[&str]) -> GlyphMatchResult {
    if known_glyphs.contains(&signal) {
        GlyphMatchResult::Exact
    } else if let Some(similar) = known_glyphs.iter().find(|g| g.starts_with(&signal[..1])) {
        GlyphMatchResult::Near(similar.to_string())
    } else {
        GlyphMatchResult::Miss
    }
}
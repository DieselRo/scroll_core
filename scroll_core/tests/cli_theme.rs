use scroll_core::cli::theme::{theme_parse, ThemeKind};

#[test]
fn theme_parse_dark_light() {
    assert_eq!(theme_parse("dark"), Ok(ThemeKind::Dark));
    assert_eq!(theme_parse("light"), Ok(ThemeKind::Light));
    assert!(theme_parse("mystic").is_err());
}

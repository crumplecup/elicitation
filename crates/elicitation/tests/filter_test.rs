//! Test Filter trait with Select types.

use elicitation::{Prompt, Select};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
}

impl Prompt for Color {
    fn prompt() -> Option<&'static str> {
        Some("Select a color:")
    }
}

impl Select for Color {
    fn options() -> Vec<Self> {
        vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Orange,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Red".to_string(),
            "Green".to_string(),
            "Blue".to_string(),
            "Yellow".to_string(),
            "Orange".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Red" => Some(Color::Red),
            "Green" => Some(Color::Green),
            "Blue" => Some(Color::Blue),
            "Yellow" => Some(Color::Yellow),
            "Orange" => Some(Color::Orange),
            _ => None,
        }
    }
}

#[test]
fn test_filter_with_closure() {
    let warm_colors =
        Color::select_with_filter(|c| matches!(c, Color::Red | Color::Yellow | Color::Orange));

    assert_eq!(warm_colors.len(), 3);
    assert!(warm_colors.contains(&Color::Red));
    assert!(warm_colors.contains(&Color::Yellow));
    assert!(warm_colors.contains(&Color::Orange));
    assert!(!warm_colors.contains(&Color::Green));
    assert!(!warm_colors.contains(&Color::Blue));
}

#[test]
fn test_filter_returns_all_when_predicate_true() {
    let all_colors = Color::select_with_filter(|_| true);

    assert_eq!(all_colors.len(), 5);
}

#[test]
fn test_filter_returns_empty_when_predicate_false() {
    let no_colors = Color::select_with_filter(|_| false);

    assert_eq!(no_colors.len(), 0);
}

#[test]
fn test_filter_with_single_variant() {
    let just_blue = Color::select_with_filter(|c| matches!(c, Color::Blue));

    assert_eq!(just_blue.len(), 1);
    assert_eq!(just_blue[0], Color::Blue);
}

#[test]
fn test_filter_preserves_type() {
    let filtered: Vec<Color> = Color::select_with_filter(|c| matches!(c, Color::Green));

    // Type check: ensure we get Vec<Color>, not something else
    let _color: Color = filtered[0];
}

#[test]
fn test_blanket_impl_works_with_existing_select_types() {
    use elicitation::StringStyle;

    // Test with an existing type that implements Select
    let agent_only = StringStyle::select_with_filter(|s| matches!(s, StringStyle::Agent));

    assert_eq!(agent_only.len(), 1);
    assert_eq!(agent_only[0], StringStyle::Agent);
}

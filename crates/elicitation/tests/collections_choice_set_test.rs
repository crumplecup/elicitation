//! Tests for ChoiceSet dynamic choice container.

use elicitation::ChoiceSet;

#[test]
fn test_choice_set_construction() {
    let moves = vec![1, 2, 3];
    let choice_set = ChoiceSet::new(moves.clone());

    assert_eq!(choice_set.items(), &[1, 2, 3]);

    let with_prompt = choice_set.with_prompt("Pick one:");
    assert_eq!(with_prompt.items(), &[1, 2, 3]);
}

#[test]
fn test_choice_set_with_prompt() {
    let items = vec!["A", "B", "C"];
    let choice_set = ChoiceSet::new(items).with_prompt("Select letter:");

    assert_eq!(choice_set.items(), &["A", "B", "C"]);
}

#[test]
fn test_choice_set_filtered_constructor() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_only = ChoiceSet::filtered(numbers, |n| n % 2 == 0);

    assert_eq!(even_only.items(), &[2, 4, 6, 8, 10]);
}

#[test]
fn test_choice_set_with_filter() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let choice_set = ChoiceSet::new(numbers);

    let odd_only = choice_set.with_filter(|n| n % 2 != 0);

    assert_eq!(odd_only.items(), &[1, 3, 5, 7, 9]);
}

#[test]
fn test_choice_set_filter_strings() {
    let words = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apricot".to_string(),
        "berry".to_string(),
    ];

    let a_words = ChoiceSet::filtered(words, |w| w.starts_with('a'));

    assert_eq!(a_words.items(), &["apple", "apricot"]);
}

#[test]
fn test_choice_set_filter_returns_empty_when_none_match() {
    let numbers = vec![1, 3, 5, 7, 9];
    let evens = ChoiceSet::filtered(numbers, |n| n % 2 == 0);

    assert_eq!(evens.items(), &[] as &[i32]);
}

#[test]
fn test_choice_set_filter_preserves_prompt() {
    let numbers = vec![1, 2, 3, 4, 5];
    let choice_set = ChoiceSet::new(numbers)
        .with_prompt("Pick a number:")
        .with_filter(|n| n % 2 == 0);

    assert_eq!(choice_set.items(), &[2, 4]);
}

#[test]
fn test_choice_set_chained_filters() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered = ChoiceSet::new(numbers)
        .with_filter(|n| n % 2 == 0) // Even numbers
        .with_filter(|n| *n > 5); // Greater than 5

    assert_eq!(filtered.items(), &[6, 8, 10]);
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    occupied: bool,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) {}",
            self.x,
            self.y,
            if self.occupied { "X" } else { "_" }
        )
    }
}

#[test]
fn test_choice_set_filter_with_custom_type() {
    let positions = vec![
        Position {
            x: 0,
            y: 0,
            occupied: true,
        },
        Position {
            x: 1,
            y: 0,
            occupied: false,
        },
        Position {
            x: 0,
            y: 1,
            occupied: true,
        },
        Position {
            x: 1,
            y: 1,
            occupied: false,
        },
    ];

    let empty_positions = ChoiceSet::filtered(positions, |pos| !pos.occupied);

    assert_eq!(empty_positions.items().len(), 2);
    assert_eq!(empty_positions.items()[0].x, 1);
    assert_eq!(empty_positions.items()[0].y, 0);
}

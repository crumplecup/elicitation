//! Tests for PathBuf implementation.

use elicitation::{Elicitation, Prompt};
use std::path::PathBuf;

#[test]
fn test_pathbuf_has_prompt() {
    assert!(PathBuf::prompt().is_some());
}

#[test]
fn test_pathbuf_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<PathBuf>();
}

#[test]
fn test_pathbuf_from_string() {
    // Test that PathBuf::from works with various path strings
    let unix_path = PathBuf::from("/home/user/file.txt");
    assert!(!unix_path.as_os_str().is_empty());

    let windows_path = PathBuf::from(r"C:\Users\user\file.txt");
    assert!(!windows_path.as_os_str().is_empty());

    let relative_path = PathBuf::from("./relative/path");
    assert!(!relative_path.as_os_str().is_empty());
}

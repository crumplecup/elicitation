//! Zero-size assertions for all WCAG proof proposition types.

use elicit_ui::{
    AccessibleAA, AltTextProvided, FocusVisible, HasLabel, KeyboardAccessible, MinTargetSize,
    NoOverflow, RenderComplete, StructuredContent, SufficientContrast, ValidRole,
};
use std::mem::size_of;

#[test]
fn props_are_zero_sized() {
    assert_eq!(size_of::<HasLabel>(), 0);
    assert_eq!(size_of::<ValidRole>(), 0);
    assert_eq!(size_of::<MinTargetSize>(), 0);
    assert_eq!(size_of::<NoOverflow>(), 0);
    assert_eq!(size_of::<KeyboardAccessible>(), 0);
    assert_eq!(size_of::<AccessibleAA>(), 0);
    assert_eq!(size_of::<SufficientContrast>(), 0);
    assert_eq!(size_of::<FocusVisible>(), 0);
    assert_eq!(size_of::<AltTextProvided>(), 0);
    assert_eq!(size_of::<StructuredContent>(), 0);
    assert_eq!(size_of::<RenderComplete>(), 0);
}

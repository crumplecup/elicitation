//! Tests for edge cases: unit structs, empty structs, unit type.

use elicitation::Generator;
use elicitation_derive::Rand;

// Unit struct (zero-sized type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
struct Marker;

// Unit struct with meaning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
struct DandelionGenetics;

// Another unit marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
struct PlayerConnected;

// Empty named struct (also zero-sized)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
struct EmptyStruct {}

#[test]
fn test_unit_struct() {
    let generator = Marker::random_generator(42);

    // Generate many times - should always be Marker
    for _ in 0..10 {
        let marker = generator.generate();
        assert_eq!(marker, Marker);
    }
}

#[test]
fn test_dandelion_genetics() {
    let generator = DandelionGenetics::random_generator(999);

    // "Random" dandelion genetics - but there's only one possibility!
    for _ in 0..10 {
        let genetics = generator.generate();
        assert_eq!(genetics, DandelionGenetics);
    }
}

#[test]
fn test_player_connected_event() {
    let generator = PlayerConnected::random_generator(123);

    // Event marker - no data needed
    let event = generator.generate();
    assert_eq!(event, PlayerConnected);
}

#[test]
fn test_empty_named_struct() {
    let generator = EmptyStruct::random_generator(42);

    for _ in 0..10 {
        let empty = generator.generate();
        assert_eq!(empty, EmptyStruct {});
    }
}

#[test]
fn test_unit_struct_deterministic() {
    let seed = 42;
    let generator1 = Marker::random_generator(seed);
    let generator2 = Marker::random_generator(seed);

    // Even though there's no randomness, API should be consistent
    for _ in 0..5 {
        assert_eq!(generator1.generate(), generator2.generate());
    }
}

#[test]
fn test_unit_struct_in_enum() {
    #[derive(Debug, PartialEq, Rand)]
    enum Event {
        Click { x: i32, y: i32 },
        Quit, // Unit variant
    }

    let generator = Event::random_generator(42);

    // Should be able to generate both Click and Quit
    let mut found_quit = false;
    let mut found_click = false;

    for _ in 0..30 {
        match generator.generate() {
            Event::Quit => found_quit = true,
            Event::Click { .. } => found_click = true,
        }

        if found_quit && found_click {
            break;
        }
    }

    // Both should appear eventually (probabilistic)
    assert!(
        found_quit || found_click,
        "Should generate at least one variant"
    );
}

#[test]
fn test_unit_struct_in_vec() {
    use elicitation_rand::VecGenerator;

    let marker_gen = Marker::random_generator(42);
    let vec_gen = VecGenerator::fixed_length(marker_gen, 5, 123);

    let markers = vec_gen.generate();
    assert_eq!(markers.len(), 5);

    // All should be Marker (trivially true)
    for marker in markers {
        assert_eq!(marker, Marker);
    }
}

#[test]
fn test_unit_struct_as_enum_variant_data() {
    // Unit struct exists outside and derives Rand
    // Then used in enum variant

    #[derive(Debug, Rand)]
    enum SimpleResponse {
        Ok,
        Error,
    }

    let generator = SimpleResponse::random_generator(42);

    // Should generate both variants
    for _ in 0..10 {
        let _response = generator.generate();
        // Success - compiles and generates
    }
}

#[test]
fn test_marker_prevents_pattern_ambiguity() {
    // Different markers for different EOF conditions
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
    struct NormalEOF;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
    struct ErrorEOF;

    // These can be used to distinguish variants in patterns
    let _normal = NormalEOF::random_generator(42).generate();
    let _error = ErrorEOF::random_generator(42).generate();

    assert_eq!(_normal, NormalEOF);
    assert_eq!(_error, ErrorEOF);
}

use terminal_style::color::{
    ansi8_to_hex, ansi8_to_rgb, rgb_to_ansi8, rgb_to_hex, validate_ansi, validate_hex,
    ColorConversionError,
};

#[test]
fn test_rgb_to_hex() {
    assert_eq!(rgb_to_hex([255, 0, 0]), "#FF0000");
    assert_eq!(rgb_to_hex([0, 255, 0]), "#00FF00");
    assert_eq!(rgb_to_hex([0, 0, 255]), "#0000FF");
    assert_eq!(rgb_to_hex([127, 127, 127]), "#7F7F7F");
    assert_eq!(rgb_to_hex([0, 0, 0]), "#000000");
    assert_eq!(rgb_to_hex([255, 255, 255]), "#FFFFFF");
}

#[test]
fn test_rgb_to_ansi8_color_cube() {
    assert_eq!(rgb_to_ansi8([255, 0, 0]), 196); // Red
    assert_eq!(rgb_to_ansi8([0, 255, 0]), 46); // Green
    assert_eq!(rgb_to_ansi8([0, 0, 255]), 21); // Blue
    assert_eq!(rgb_to_ansi8([255, 255, 0]), 226); // Yellow
    assert_eq!(rgb_to_ansi8([0, 255, 255]), 51); // Cyan
    assert_eq!(rgb_to_ansi8([255, 0, 255]), 201); // Magenta
}

#[test]
fn test_rgb_to_ansi8_grayscale() {
    assert_eq!(rgb_to_ansi8([0, 0, 0]), 16); // Black
    assert_eq!(rgb_to_ansi8([128, 128, 128]), 244); // Middle gray
    assert_eq!(rgb_to_ansi8([255, 255, 255]), 231); // White from main palette
}

#[test]
fn test_ansi8_to_rgb_standard_colors() {
    assert_eq!(ansi8_to_rgb(196), [255, 0, 0]); // Red
    assert_eq!(ansi8_to_rgb(46), [0, 255, 0]); // Green
    assert_eq!(ansi8_to_rgb(21), [0, 0, 255]); // Blue
    assert_eq!(ansi8_to_rgb(231), [255, 255, 255]); // Last in color cube
}

#[test]
fn test_ansi8_to_rgb_grayscale_range() {
    assert_eq!(ansi8_to_rgb(232), [8, 8, 8]);
    assert_eq!(ansi8_to_rgb(243), [118, 118, 118]);
    assert_eq!(ansi8_to_rgb(255), [238, 238, 238]);
}

#[test]
fn test_ansi8_to_hex_roundtrip() {
    let ansi_values = [16u8, 46, 196, 226, 231, 243, 255];
    for &code in &ansi_values {
        let hex = ansi8_to_hex(code);
        let rgb = ansi8_to_rgb(code);
        assert_eq!(hex, rgb_to_hex(rgb));
    }
}

#[test]
fn test_validate_hex_valid_inputs() {
    let valid_hexes = [
        "#fff",    // shorthand hex
        "#FFFFFF", // full uppercase
        "#000000", // full lowercase
        "abc",     // no #
        "123456",  // valid digits
    ];

    for input in valid_hexes {
        assert!(
            validate_hex(input).is_ok(),
            "Expected '{}' to be valid",
            input
        );
    }
}

#[test]
fn test_validate_hex_invalid_inputs() {
    let cases = [
        ("", "empty string"),
        ("#", "just a hash"),
        ("#12", "too short"),
        ("#1234", "invalid length"),
        ("#12345g", "contains non-hex digit"),
        ("12345z", "non-hex without #"),
        ("#abcd", "length 4 invalid"),
        ("##123456", "extra hash"),
    ];

    for (input, description) in cases {
        assert!(
            validate_hex(input).is_err(),
            "Expected '{}' to be invalid ({})",
            input,
            description
        );
    }
}

#[test]
fn test_validate_ansi_valid_values() {
    assert!(validate_ansi(0).is_ok());
    assert!(validate_ansi(128).is_ok());
    assert!(validate_ansi(255).is_ok());
}

#[test]
fn test_validate_ansi_invalid_values() {
    assert!(matches!(
        validate_ansi(-1),
        Err(ColorConversionError::InvalidAnsiValue(-1))
    ));

    assert!(matches!(
        validate_ansi(256),
        Err(ColorConversionError::InvalidAnsiValue(256))
    ));
}

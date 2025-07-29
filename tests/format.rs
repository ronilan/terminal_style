use terminal_style::format::{background, bold, color, faint, inverse, italic};

#[test]
fn test_background_with_rgb_array() {
    let styled = background([0, 0, 255], "Blue").unwrap();
    assert_eq!(styled, "\x1b[48;5;21mBlue\x1b[0m");
}

#[test]
fn test_background_with_u8() {
    let styled = background(226u8, "Yellow Background").unwrap();
    assert_eq!(styled, "\x1b[48;5;226mYellow Background\x1b[0m");
}

#[test]
fn test_background_with_hex_string() {
    let styled = background("#00FFFF", "Cyan").unwrap();
    assert_eq!(styled, "\x1b[48;5;51mCyan\x1b[0m");
}

#[test]
fn test_background_with_invalid_hex() {
    let result = background("##bad", "Oops");
    assert!(result.is_err());
}

#[test]
fn test_color_with_rgb_array() {
    let styled = color([255, 0, 0], "Red").unwrap();
    assert_eq!(styled, "\x1b[38;5;196mRed\x1b[0m");
}

#[test]
fn test_color_with_u8() {
    let styled = color(82u8, "ANSI Green").unwrap();
    assert_eq!(styled, "\x1b[38;5;82mANSI Green\x1b[0m");
}

#[test]
fn test_color_with_hex_string() {
    let styled = color("#00FF00", "Green").unwrap();
    assert_eq!(styled, "\x1b[38;5;46mGreen\x1b[0m");
}

#[test]
fn test_color_with_invalid_hex() {
    let result = color("#XYZ", "Invalid");
    assert!(result.is_err());
}

#[test]
fn test_bold() {
    let text = "Hello, World!";
    let result = bold(text);
    assert_eq!(result, "\x1b[1mHello, World!\x1b[0m");
}

#[test]
fn test_bold_empty_string() {
    let text = "";
    let result = bold(text);
    assert_eq!(result, "\x1b[1m\x1b[0m");
}

#[test]
fn test_italic() {
    let text = "Hello, World!";
    let result = italic(text);
    assert_eq!(result, "\x1b[3mHello, World!\x1b[0m");
}

#[test]
fn test_italic_empty_string() {
    let text = "";
    let result = italic(text);
    assert_eq!(result, "\x1b[3m\x1b[0m");
}

#[test]
fn test_faint() {
    let text = "Hello, World!";
    let result = faint(text);
    assert_eq!(result, "\x1b[2mHello, World!\x1b[0m");
}

#[test]
fn test_faint_empty_string() {
    let text = "";
    let result = faint(text);
    assert_eq!(result, "\x1b[2m\x1b[0m");
}

#[test]
fn test_inverse() {
    let text = "Hello, World!";
    let result = inverse(text);
    assert_eq!(result, "\x1b[7mHello, World!\x1b[0m");
}

#[test]
fn test_inverse_empty_string() {
    let text = "";
    let result = inverse(text);
    assert_eq!(result, "\x1b[7m\x1b[0m");
}

#[test]
fn test_bold_with_special_characters() {
    let text = "Hello, World! 123 @#$%";
    let result = bold(text);
    assert_eq!(result, "\x1b[1mHello, World! 123 @#$%\x1b[0m");
}

#[test]
fn test_italic_with_special_characters() {
    let text = "Hello, World! 123 @#$%";
    let result = italic(text);
    assert_eq!(result, "\x1b[3mHello, World! 123 @#$%\x1b[0m");
}

#[test]
fn test_faint_with_special_characters() {
    let text = "Hello, World! 123 @#$%";
    let result = faint(text);
    assert_eq!(result, "\x1b[2mHello, World! 123 @#$%\x1b[0m");
}

#[test]
fn test_inverse_with_special_characters() {
    let text = "Hello, World! 123 @#$%";
    let result = inverse(text);
    assert_eq!(result, "\x1b[7mHello, World! 123 @#$%\x1b[0m");
}

#[test]
fn test_bold_with_newlines() {
    let text = "Hello\nWorld!";
    let result = bold(text);
    assert_eq!(result, "\x1b[1mHello\nWorld!\x1b[0m");
}

#[test]
fn test_italic_with_newlines() {
    let text = "Hello\nWorld!";
    let result = italic(text);
    assert_eq!(result, "\x1b[3mHello\nWorld!\x1b[0m");
}

#[test]
fn test_faint_with_newlines() {
    let text = "Hello\nWorld!";
    let result = faint(text);
    assert_eq!(result, "\x1b[2mHello\nWorld!\x1b[0m");
}

#[test]
fn test_inverse_with_newlines() {
    let text = "Hello\nWorld!";
    let result = inverse(text);
    assert_eq!(result, "\x1b[7mHello\nWorld!\x1b[0m");
}

#[test]
fn test_formatting_differences() {
    let text = "Test";
    let bold_result = bold(text);
    let italic_result = italic(text);
    let faint_result = faint(text);
    let inverse_result = inverse(text);

    // All should be different
    assert_ne!(bold_result, italic_result);
    assert_ne!(bold_result, faint_result);
    assert_ne!(bold_result, inverse_result);
    assert_ne!(italic_result, faint_result);
    assert_ne!(italic_result, inverse_result);
    assert_ne!(faint_result, inverse_result);

    // All should start with different escape codes
    assert!(bold_result.starts_with("\x1b[1m"));
    assert!(italic_result.starts_with("\x1b[3m"));
    assert!(faint_result.starts_with("\x1b[2m"));
    assert!(inverse_result.starts_with("\x1b[7m"));

    // All should end with reset
    assert!(bold_result.ends_with("\x1b[0m"));
    assert!(italic_result.ends_with("\x1b[0m"));
    assert!(faint_result.ends_with("\x1b[0m"));
    assert!(inverse_result.ends_with("\x1b[0m"));
}

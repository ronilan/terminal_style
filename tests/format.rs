use terminal_style::format::{background, bold, color, faint, inverse, italic, underline};

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
fn test_underline() {
    let text = "Hello, World!";
    let result = underline(text);
    assert_eq!(result, "\x1b[4mHello, World!\x1b[0m");
}

#[test]
fn test_underline_empty_string() {
    let text = "";
    let result = underline(text);
    assert_eq!(result, "\x1b[4m\x1b[0m");
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

#[test]
fn test_bold_ref_string() {
    let s = String::from("Hello");
    let result = bold(&s);
    assert_eq!(result, "\x1b[1mHello\x1b[0m");
}

#[test]
fn test_color_ref_string() {
    let s = String::from("Red");
    let result: String = color(196u8, &s).unwrap();
    assert_eq!(result, "\x1b[38;5;196mRed\x1b[0m");
}

#[test]
fn test_bold_ref_vector() {
    let texts = vec!["A".to_string(), "B".to_string()];
    let styled: Vec<String> = bold(&texts); // use reference
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[1m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_color_ref_vector() {
    let texts = vec!["Red".to_string(), "Blue".to_string()];
    let styled: Vec<String> = color(196u8, &texts).unwrap();
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[38;5;196m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_bold_ref_2d_vector() {
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];
    let styled: Vec<Vec<String>> = bold(&texts_2d); // reference
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[1m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_color_ref_2d_vector() {
    let texts_2d = vec![
        vec!["One".to_string(), "Two".to_string()],
        vec!["Three".to_string(), "Four".to_string()],
    ];
    let styled: Vec<Vec<String>> = color(46u8, &texts_2d).unwrap(); // reference
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[38;5;46m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}



#[test]
fn test_vector_foreground() {
    let texts = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];
    let colored: Vec<String> = color(196u8, texts.clone()).unwrap(); // ANSI Red
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[38;5;196m{}\x1b[0m", t))
        .collect();
    assert_eq!(colored, expected);
}

#[test]
fn test_vector_background() {
    let texts = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];
    let bg_colored: Vec<String> = background(21u8, texts.clone()).unwrap(); // ANSI Blue BG
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[48;5;21m{}\x1b[0m", t))
        .collect();
    assert_eq!(bg_colored, expected);
}

#[test]
fn test_bold_vector() {
    let texts = vec!["A".to_string(), "B".to_string()];
    let styled: Vec<String> = bold(texts.clone()); // directly on Vec<String>
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[1m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_italic_vector() {
    let texts = vec!["A".to_string(), "B".to_string()];
    let styled: Vec<String> = italic(texts.clone());
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[3m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_faint_vector() {
    let texts = vec!["A".to_string(), "B".to_string()];
    let styled: Vec<String> = faint(texts.clone());
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[2m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_inverse_vector() {
    let texts = vec!["A".to_string(), "B".to_string()];
    let styled: Vec<String> = inverse(texts.clone());
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[7m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_underline_vector() {
    let texts = vec!["A".to_string(), "B".to_string()];
    let styled: Vec<String> = underline(texts.clone());
    let expected: Vec<String> = texts
        .iter()
        .map(|t| format!("\x1b[4m{}\x1b[0m", t))
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_bold_2d_vector() {
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];
    let styled: Vec<Vec<String>> = bold(texts_2d.clone()); // directly on Vec<Vec<String>>
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[1m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_italic_2d_vector() {
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];
    let styled: Vec<Vec<String>> = italic(texts_2d.clone());
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[3m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_faint_2d_vector() {
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];
    let styled: Vec<Vec<String>> = faint(texts_2d.clone());
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[2m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_inverse_2d_vector() {
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];
    let styled: Vec<Vec<String>> = inverse(texts_2d.clone());
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[7m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_underline_2d_vector() {
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];
    let styled: Vec<Vec<String>> = underline(texts_2d.clone());
    let expected: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| row.iter().map(|t| format!("\x1b[4m{}\x1b[0m", t)).collect())
        .collect();
    assert_eq!(styled, expected);
}

#[test]
fn test_2d_vector_foreground() {
    let texts_2d = vec![
        vec!["One".to_string(), "Two".to_string()],
        vec!["Three".to_string(), "Four".to_string()],
    ];
    let colored_2d: Vec<Vec<String>> = color(46u8, texts_2d.clone()).unwrap(); // ANSI Green
    let expected_2d: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| format!("\x1b[38;5;46m{}\x1b[0m", t))
                .collect()
        })
        .collect();
    assert_eq!(colored_2d, expected_2d);
}

#[test]
fn test_2d_vector_background() {
    let texts_2d = vec![
        vec!["One".to_string(), "Two".to_string()],
        vec!["Three".to_string(), "Four".to_string()],
    ];
    let bg_colored_2d: Vec<Vec<String>> = background(226u8, texts_2d.clone()).unwrap(); // ANSI Yellow BG
    let expected_bg_2d: Vec<Vec<String>> = texts_2d
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| format!("\x1b[48;5;226m{}\x1b[0m", t))
                .collect()
        })
        .collect();
    assert_eq!(bg_colored_2d, expected_bg_2d);
}

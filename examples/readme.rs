use terminal_style::{
    format::{bold, underline, color, background},
    color::ColorConversionError,
};

fn main() -> Result<(), ColorConversionError> {
    // --- Single string styling ---
    let text = "Styled!";
    let fg = color("#FF1493", text)?;       // Foreground color (orange)
    let bg = background("#EEDDFF", text)?;   // Background color (green)
    let bolded = bold(fg.clone());                  // Bold formatting

    println!("FG: {}", fg);
    println!("BG: {}", bg);
    println!("Bold: {}", bolded);

    // --- 1D vector of strings ---
    let texts_1d = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];
    let colored_1d = color([255, 0, 0], texts_1d.clone())?;  // Red foreground
    let bolded_1d = bold(texts_1d.clone());

    println!("\n1D Colored vector:");
    for line in &colored_1d {
        println!("{}", line);
    }

    println!("\n1D Bold vector:");
    for line in &bolded_1d {
        println!("{}", line);
    }

    // --- 2D vector of strings ---
    let texts_2d = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string(), "D".to_string()],
    ];

    let bolded_2d: Vec<Vec<String>> = bold(texts_2d.clone());
    let bg_colored_2d = background([255, 105, 180], texts_2d.clone())?; // Green background
    let bold_underline_bg_2d = bold(underline(bg_colored_2d.clone()));

    println!("\n2D Bold vector:");
    for row in &bolded_2d {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }

    println!("\n2D Background colored vector:");
    for row in &bg_colored_2d {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }

    println!("\n2D Bold + Underline + Background colored vector:");
    for row in &bold_underline_bg_2d {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }

    Ok(())
}
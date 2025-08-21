# terminal_style

A minimal Rust library for styling terminal text using ANSI escape codes. Supports 256-color as well as bold, italic, faint, underline, and inverse formatting. Easily apply foreground/background colors from hex, RGB, or ANSI 8-bit values to strings, 1D vectors, and 2D vectors. Perfect for simple CLI tools.

<img src="./media/social.png" width="450"> 

## Installation

`terminal_style` is published as a [crate](https://crates.io/crates/terminal_style) on crates.io.

```bash
cargo add terminal_style
```

## Features

- Convert RGB or Hex to ANSI 256-color
- Apply foreground/background color to strings
- Format text as **bold**, *italic*, faint, inverse, or underline
- Graceful handling of invalid color inputs

## Usage

Formatting functions work with strings, vectors, and 2D vectors of strings seamlessly. It also supports references, so you can pass either owned or borrowed values.

### Supported Input Types

| Input Type           | Output Type       | Description                                |
|----------------------|-----------------|--------------------------------------------|
| `String`             | `String`        | Single string formatting                   |
| `&str`               | `String`        | Single string formatting                   |
| `&String`            | `String`        | Reference forwarding to `String`          |
| `Vec<String>`        | `Vec<String>`   | Format each element individually           |
| `&Vec<String>`       | `Vec<String>`   | Reference forwarding to owned `Vec<String>`|
| `Vec<Vec<String>>`   | `Vec<Vec<String>>` | Format every element in each subvector  |
| `&Vec<Vec<String>>`  | `Vec<Vec<String>>` | Reference forwarding to owned `Vec<Vec<String>>` |


### Example 

```rust

use terminal_style::{
    format::{bold, underline, color, background},
    color::ColorConversionError,
};

fn main() -> Result<(), ColorConversionError> {
    // --- Single string styling ---
    let text = "Styled!";

    // Using `?` to propagate errors if color conversion fails
    let fg = color("#FF1493", text)?;       // Foreground color (pink)
    let bg = background("#EEDDFF", text)?;  // Background color (lavender)
    let bolded = bold(fg.clone());          // Bold formatting

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
    let bg_colored_2d = background([255, 105, 180], texts_2d.clone())?; // Pink background
    let bold_underline_bg_2d = bold(underline(bg_colored_2d.clone()));

    // Output demo
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

```

## Color Conversion Examples

Utility functions enable converting between RGB, HEX, and ANSI 8-bit values.

```rust
use terminal_style::color::*;

fn main() {
    // RGB to Hex
    assert_eq!(rgb_to_hex([255, 165, 0]), "#FFA500");

    // Hex to RGB
    assert_eq!(hex_to_rgb("#00FF00"), [0, 255, 0]);

    // RGB to ANSI
    assert_eq!(rgb_to_ansi8([255, 0, 0]), 196);

    // Hex to ANSI
    assert_eq!(hex_to_ansi8("0000FF"), 21);

    // ANSI to RGB
    assert_eq!(ansi8_to_rgb(46), Some([0, 255, 0]));

    // ANSI to HEX
    assert_eq!(ansi8_to_hex(196), "#FF0000"); // Red
}
```

Additional examples in the `examples` folder.

## Tests

```bash
cargo test
```

## Structure

- `color/`: Utility color conversions (hex, rgb, ansi)
- `format/`: Terminal text styling functions
- `tests/`: Test suite
- `examples/`: Usage examples

## Authors

[Ron Ilan](https://www.ronilan.com)

## License
[MIT](https://en.wikipedia.org/wiki/MIT_License)

###### Coded in Rust with a little help from the LLMs. Based on the lovingly handcrafted [colors.crumb](https://github.com/ronilan/colors.crumb).. Derived from work done on [Impossible.js](https://github.com/ronilan/that-is-impossible). Enjoy.

Fabriqué au Canada : Made in Canada 🇨🇦

use std::io::{self, Write};
use terminal_style::color::{hex_to_ansi8, validate_hex, ColorConversionError};
use terminal_style::format::color;

fn main() -> Result<(), ColorConversionError> {
    loop {
        print!("Enter a hex color value (e.g. #C0ffee or BADA55 or #B52 or aaa), or type 'exit' to quit: ");
        io::stdout().flush().unwrap();

        let mut hex = String::new();
        io::stdin().read_line(&mut hex).unwrap();
        let hex = hex.trim();

        if hex.eq_ignore_ascii_case("exit") {
            break;
        }

        if let Err(err) = validate_hex(hex) {
            eprintln!("❌ {}", err);
            continue;
        }

        let ansi_code = hex_to_ansi8(hex);
        println!("{}", color(ansi_code, "Hello World")?);
    }

    Ok(())
}

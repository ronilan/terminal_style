use std::io::{self, Write};
use terminal_style::color::{hex_to_ansi8, validate_ansi, validate_hex, ColorConversionError};
use terminal_style::format::color;

fn main() -> Result<(), ColorConversionError> {
    // Choose format once
    let format = loop {
        println!("Choose color input format: [ansi] or [hex] (or type 'exit' to quit):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut format_input = String::new();
        io::stdin().read_line(&mut format_input).unwrap();
        let format_input = format_input.trim().to_lowercase();

        match format_input.as_str() {
            "ansi" | "hex" => break format_input,
            "exit" => return Ok(()),
            _ => eprintln!("❌ Invalid format. Please enter 'ansi' or 'hex'."),
        }
    };

    // Keep prompting for value in selected format
    loop {
        match format.as_str() {
            "ansi" => {
                print!("Enter ANSI code (0–255), or 'exit' to quit: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.eq_ignore_ascii_case("exit") {
                    break;
                }

                match input.parse::<i32>() {
                    Ok(num) => match validate_ansi(num) {
                        Ok(()) => println!("{}", color(num as u8, "Hello World")?),
                        Err(err) => eprintln!("❌ {}", err),
                    },
                    Err(_) => eprintln!("❌ Invalid number format"),
                }
            }

            "hex" => {
                print!("Enter hex color (e.g. #C0ffee, BADA55, #B52, aaa), or 'exit' to quit: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.eq_ignore_ascii_case("exit") {
                    break;
                }

                match validate_hex(input) {
                    Ok(()) => {
                        let ansi_code = hex_to_ansi8(input);
                        println!("{}", color(ansi_code, "Hello World")?);
                    }
                    Err(err) => eprintln!("❌ {}", err),
                }
            }

            _ => unreachable!(), // format already validated
        }

        println!(); // spacing between inputs
    }

    Ok(())
}

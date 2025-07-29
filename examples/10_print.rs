use rand::Rng;
use std::io::{stdout, Write};
use std::{thread, time};
use terminal_style::format::color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n: u32 = 0;
    let mut rng = rand::thread_rng();

    loop {
        let base = n as f64 + (rng.gen::<f64>() * 100.0);
        let c = 16 + ((base as u32) % 216);
        let slash = if rng.gen_bool(0.5) { "╱" } else { "╲" };

        let colored_text = color(c as u8, slash)?;
        print!("{}", colored_text);
        stdout().flush().unwrap();

        thread::sleep(time::Duration::from_millis(1));
        n = n.wrapping_add(1);
    }
}

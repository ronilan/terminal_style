use terminal_style::format::bold;
use terminal_style::format::faint;
use terminal_style::format::inverse;
use terminal_style::format::italic;

fn main() {
    println!("=== Format Examples ===\n");

    println!("{}", bold("Bold text"));
    println!("{}", italic("Italic text"));
    println!("{}", faint("Faint text"));
    println!("{}", inverse("Inverse text"));
    println!();

    println!(
        "Combined: {} {} {} {}",
        bold("Bold"),
        italic("Italic"),
        faint("Faint"),
        inverse("Inverse")
    );
    println!();

    println!("With special chars: {}", bold("Bold #123!"));
    println!("Multi-line:\n{}", italic("Line1\nLine2"));
    println!();

    println!("Empty strings: '{}', '{}'", faint(""), inverse(""));
    println!();

    println!(
        "Practical:\n{}: Warning!\n{}: Note\n{}: Hint\n{}: Alert",
        bold("WARNING"),
        italic("Note"),
        faint("Hint"),
        inverse("ALERT")
    );
}

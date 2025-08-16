use terminal_style::format::bold;
use terminal_style::format::faint;
use terminal_style::format::inverse;
use terminal_style::format::italic;
use terminal_style::format::underline;

fn main() {
    println!("=== Format Examples ===\n");

    println!("{}", bold("Bold text"));
    println!("{}", italic("Italic text"));
    println!("{}", faint("Faint text"));
    println!("{}", inverse("Inverse text"));
    println!("{}", underline("Underline text"));
    println!();

    println!(
        "Combined: {} {} {} {} {}",
        bold("Bold"),
        italic("Italic"),
        faint("Faint"),
        inverse("Inverse"),
        underline("Underline")
    );
    println!();

    println!("With special chars: {}", bold("Bold #123!"));
    println!("Multi-line:\n{}", italic("Line1\nLine2"));
    println!();

    println!("Empty strings: '{}', '{}'", faint(""), inverse(""));
    println!();

    println!(
        "Practical:\n{}: Warning!\n{}: Note\n{}: Hint\n{}: Important\n{}: Alert",
        bold("WARNING"),
        italic("Note"),
        faint("Hint"),
        underline("Important"),
        inverse("ALERT")
    );
}

fn main() {
    let text = "Hello!";
    let output = terminal_style::format::bold(text);
    println!("{}", output);
}

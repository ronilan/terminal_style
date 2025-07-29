pub fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

pub fn italic(text: &str) -> String {
    format!("\x1b[3m{}\x1b[0m", text)
}

pub fn faint(text: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", text)
}

pub fn inverse(text: &str) -> String {
    format!("\x1b[7m{}\x1b[0m", text)
}

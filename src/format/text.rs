// format.rs
use super::stylable::Stylable;

// --- Text formatting functions ---

/// Formats the input text as **bold**.
///
/// # Example
///
/// ```
/// use terminal_style::format::bold;
///
/// let s = bold("Hello");
/// assert_eq!(s, "\x1b[1mHello\x1b[0m");
/// ```
pub fn bold<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[1m{}\x1b[0m", s))
}

/// Formats the input text as *italic*.
///
/// # Example
///
/// ```
/// use terminal_style::format::italic;
///
/// let s = italic("Hello");
/// assert_eq!(s, "\x1b[3mHello\x1b[0m");
/// ```
pub fn italic<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[3m{}\x1b[0m", s))
}

/// Formats the input text with *faint* (decreased intensity) style.
///
/// # Example
///
/// ```
/// use terminal_style::format::faint;
///
/// let s = faint("Hello");
/// assert_eq!(s, "\x1b[2mHello\x1b[0m");
/// ```
pub fn faint<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[2m{}\x1b[0m", s))
}

/// Formats the input text with *inverse* (swapped foreground/background) style.
///
/// # Example
///
/// ```
/// use terminal_style::format::inverse;
///
/// let s = inverse("Hello");
/// assert_eq!(s, "\x1b[7mHello\x1b[0m");
/// ```
pub fn inverse<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[7m{}\x1b[0m", s))
}

/// Formats the input text with an **underline**.
///
/// # Example
///
/// ```
/// use terminal_style::format::underline;
///
/// let s = underline("Hello");
/// assert_eq!(s, "\x1b[4mHello\x1b[0m");
/// ```
pub fn underline<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[4m{}\x1b[0m", s))
}

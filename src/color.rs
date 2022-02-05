// At this rate I should jst make this colorprint stuff a crate..
// I copy it to almost every project I make now...

/// Define Text Colors
#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Color {
    /// Get Color as an Integer.
    #[rustfmt::skip]
    pub fn code(self) -> u32 {
        match self {
            Color::Reset   => 0,
            Color::Black   => 30,
            Color::Red     => 31,
            Color::Green   => 32,
            Color::Yellow  => 33,
            Color::Blue    => 34,
            Color::Magenta => 35,
            Color::Cyan    => 36,
            Color::White   => 37,
        }
    }
}

/// Return string with ANSI color codes
pub fn color<T>(text: T, color: Color) -> String
where
    T: std::fmt::Display,
{
    format!("\x1B[0;{}m{}\x1B[0;0m", color.code(), text)
}

/// Return string with ANSI color codes for bold text
pub fn color_bold<T>(text: T, color: Color) -> String
where
    T: std::fmt::Display,
{
    format!("\x1B[1;{}m{}\x1B[0m", color.code(), text)
}

/// Color Print
///
/// Macro for *easy* printing of colored text to the console.
/// ## Example
/// ```rust
/// // A simple print
/// color_print!(Color::Green, "This is a green message!");
///
/// // A more complex print
/// color_print!(Color::Green, "This is a {} message!", "green");
/// ```
macro_rules! color_print {
    ($color:expr, $text:expr) => (
        println!("{}", color::color($text, $color))
    );
    ($color:expr, $($exp:expr),+) => (
        let mut text: String = "{}".to_string();
        $(text = text.replacen("{}", &$exp.to_string(), 1);)*
        println!("{}", color::color(&text, $color))
    );
}

/// Color options
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
    /// Bright black
    BrightBlack,
    /// Bright red
    BrightRed,
    /// Bright green
    BrightGreen,
    /// Bright yellow
    BrightYellow,
    /// Bright blue
    BrightBlue,
    /// Bright magenta
    BrightMagenta,
    /// Bright cyan
    BrightCyan,
    /// Bright white
    BrightWhite,
    /// 256-color mode color
    Color256(u8),
    /// RGB color value
    Rgb(u8, u8, u8),
    /// Current color
    Current,
}

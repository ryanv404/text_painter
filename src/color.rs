/// Color options
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Primary color
    Primary,
    /// Bright black
    BriBlack,
    /// Bright red
    BriRed,
    /// Bright green
    BriGreen,
    /// Bright yellow
    BriYellow,
    /// Bright blue
    BriBlue,
    /// Bright magenta
    BriMagenta,
    /// Bright cyan
    BriCyan,
    /// Bright white
    BriWhite,
    /// 256-color mode color
    Color256(u8),
    /// RGB color value
    Rgb(u8, u8, u8),
}

///// Appends linear gradient to the given string
//pub fn write_gradient(
//    res: &mut String, s: impl AsRef<str>, s_len: usize,
//    start: (u8, u8, u8), end: (u8, u8, u8)
//) {
//    let len = s_len as f32 - 1.;
//
//    let step = if s_len == 1 {
//        (0., 0., 0.)
//    } else {
//        ((end.0 as f32 - start.0 as f32) / len,
//         (end.1 as f32 - start.1 as f32) / len,
//         (end.2 as f32 - start.2 as f32) / len)
//    };
//
//    for (i, c) in s.as_ref().chars().take(s_len).enumerate() {
//        res.push_str(&codes::fg!(
//            start.0 as f32 + step.0 * i as f32,
//            start.1 as f32 + step.1 * i as f32,
//            start.2 as f32 + step.2 * i as f32
//        ));
//        res.push(c);
//    }
//}
//
///// Generates linear color gradient with the given text
//pub fn gradient(
//    s: impl AsRef<str>, start: (u8, u8, u8), end: (u8, u8, u8),
//) -> String {
//    let mut res = String::new();
//    let len = s.as_ref().chars().count();
//    write_gradient(&mut res, s, len, start, end);
//    res
//}

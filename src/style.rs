use std::{fmt, string};

use crate::{
    attr::Attr,
    color::Color,
};

/// Represents styled text.
#[derive(Clone)]
pub struct Style<'a> {
    pub attrs: Option<Vec<Attr>>,
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub buffer: &'a str,
}

impl<'a> fmt::Display for Style<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let csi = "\x1b[";
        let reset = "\x1b[0m";
        if let Some(ref ansi_str) = self.to_ansi_str() {
            write!(f, "{csi}{ansi_str}{}{reset}", self.buffer)
        } else {
            write!(f, "{}", self.buffer)
        }
    }
}

impl<'a> Style<'a> {
    /// Returns the default `StyleBuilder` object with the provided text.
    #[must_use]
    pub const fn new(buf: &'a str) -> Self {
        Self {
            attrs: None,
            fg: None,
            bg: None,
            buffer: buf,
        }
    }

    /// Returns the configured Style object.
    #[must_use]
    pub fn build(&mut self) -> Self {
        self.clone()
    }

    /// True if a new attribute, foreground color, or background color has been set.
    fn has_new_style(&self) -> bool {
        self.has_new_attrs()
            || self.has_new_fg()
            || self.has_new_bg()
    }

    /// True if any new text attributes have been set.
    const fn has_new_attrs(&self) -> bool {
        self.attrs.is_some()
    }

    /// True if a foreground color other than current has been set.
    fn has_new_fg(&self) -> bool {
        self.fg.is_some() && self.fg != Some(Color::Current)
    }

    /// True if a background color other than current has been set.
    fn has_new_bg(&self) -> bool {
        self.bg.is_some() && self.bg != Some(Color::Current)
    }

    fn get_attrs_ansi(&self) -> Option<String> {
        self.attrs.as_ref().map(|attrs| {
            let attrs_str: Vec<String> = attrs
                .iter()
                .map(string::ToString::to_string)
                .collect();
            attrs_str.join(";")
        })
    }

    fn get_fg_ansi(&self) -> Option<String> {
        if self.fg.is_none() || !self.has_new_fg() {
            return None;
        }

        match self.fg.unwrap() {
            Color::Black => Some("30".to_string()),
            Color::Red => Some("31".to_string()),
            Color::Green => Some("32".to_string()),
            Color::Yellow => Some("33".to_string()),
            Color::Blue => Some("34".to_string()),
            Color::Magenta => Some("35".to_string()),
            Color::Cyan => Some("36".to_string()),
            Color::White => Some("37".to_string()),
            Color::BrightBlack => Some("90".to_string()),
            Color::BrightRed => Some("91".to_string()),
            Color::BrightGreen => Some("92".to_string()),
            Color::BrightYellow => Some("93".to_string()),
            Color::BrightBlue => Some("94".to_string()),
            Color::BrightMagenta => Some("95".to_string()),
            Color::BrightCyan => Some("96".to_string()),
            Color::BrightWhite => Some("97".to_string()),
            Color::Color256(num) => Some(format!("38;5;{num}")),
            Color::Rgb(r, g, b) => Some(format!("38;2;{r};{g};{b}")),
            Color::Current => None,
        }
    }

    fn get_bg_ansi(&self) -> Option<String> {
        if self.bg.is_none() || !self.has_new_bg() {
            return None;
        }

        match self.bg.unwrap() {
            Color::Black => Some("40".to_string()),
            Color::Red => Some("41".to_string()),
            Color::Green => Some("42".to_string()),
            Color::Yellow => Some("43".to_string()),
            Color::Blue => Some("44".to_string()),
            Color::Magenta => Some("45".to_string()),
            Color::Cyan => Some("46".to_string()),
            Color::White => Some("47".to_string()),
            Color::BrightBlack => Some("100".to_string()),
            Color::BrightRed => Some("101".to_string()),
            Color::BrightGreen => Some("102".to_string()),
            Color::BrightYellow => Some("103".to_string()),
            Color::BrightBlue => Some("104".to_string()),
            Color::BrightMagenta => Some("105".to_string()),
            Color::BrightCyan => Some("106".to_string()),
            Color::BrightWhite => Some("107".to_string()),
            Color::Color256(num) => Some(format!("48;5;{num}")),
            Color::Rgb(r, g, b) => Some(format!("48;2;{r};{g};{b}")),
            Color::Current => None,
        }
    }

    fn to_ansi_str(&self) -> Option<String> {
        let mut ansi_str = String::new();

        if let Some(ref attrs_ansi) = self.get_attrs_ansi() {
            ansi_str.push_str(attrs_ansi);
            if self.has_new_fg() || self.has_new_bg() {
                ansi_str.push(';');
            }
        }

        if let Some(ref fg_ansi) = self.get_fg_ansi() {
            ansi_str.push_str(fg_ansi);
            if self.has_new_bg() {
                ansi_str.push(';');
            }
        }

        if let Some(ref bg_ansi) = self.get_bg_ansi() {
            ansi_str.push_str(bg_ansi);
        }

        if self.has_new_style() {
            ansi_str.push('m');
            Some(ansi_str)
        } else {
            None
        }
    }

    pub fn black(&mut self) -> &mut Self {
        self.fg = Some(Color::Black);
        self
    }

    pub fn red(&mut self) -> &mut Self {
        self.fg = Some(Color::Red);
        self
    }

    pub fn green(&mut self) -> &mut Self {
        self.fg = Some(Color::Green);
        self
    }

    pub fn yellow(&mut self) -> &mut Self {
        self.fg = Some(Color::Yellow);
        self
    }

    pub fn blue(&mut self) -> &mut Self {
        self.fg = Some(Color::Blue);
        self
    }

    pub fn magenta(&mut self) -> &mut Self {
        self.fg = Some(Color::Magenta);
        self
    }

    pub fn cyan(&mut self) -> &mut Self {
        self.fg = Some(Color::Cyan);
        self
    }

    pub fn white(&mut self) -> &mut Self {
        self.fg = Some(Color::White);
        self
    }

    pub fn color256(&mut self, num: u8) -> &mut Self {
        self.fg = Some(Color::Color256(num));
        self
    }

    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.fg = Some(Color::Rgb(r, g, b));
        self
    }

    pub fn current(&mut self) -> &mut Self {
        self.fg = Some(Color::Current);
        self
    }

    pub fn on_black(&mut self) -> &mut Self {
        self.bg = Some(Color::Black);
        self
    }

    pub fn on_red(&mut self) -> &mut Self {
        self.bg = Some(Color::Red);
        self
    }

    pub fn on_green(&mut self) -> &mut Self {
        self.bg = Some(Color::Green);
        self
    }

    pub fn on_yellow(&mut self) -> &mut Self {
        self.bg = Some(Color::Yellow);
        self
    }

    pub fn on_blue(&mut self) -> &mut Self {
        self.bg = Some(Color::Blue);
        self
    }

    pub fn on_magenta(&mut self) -> &mut Self {
        self.bg = Some(Color::Magenta);
        self
    }

    pub fn on_cyan(&mut self) -> &mut Self {
        self.bg = Some(Color::Cyan);
        self
    }

    pub fn on_white(&mut self) -> &mut Self {
        self.bg = Some(Color::White);
        self
    }

    pub fn on_color256(&mut self, num: u8) -> &mut Self {
        self.bg = Some(Color::Color256(num));
        self
    }

    pub fn on_rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.bg = Some(Color::Rgb(r, g, b));
        self
    }

    pub fn on_current(&mut self) -> &mut Self {
        self.bg = Some(Color::Current);
        self
    }

    pub fn bright_black(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightBlack);
        self
    }

    pub fn bright_red(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightRed);
        self
    }

    pub fn bright_green(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightGreen);
        self
    }

    pub fn bright_yellow(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightYellow);
        self
    }

    pub fn bright_blue(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightBlue);
        self
    }

    pub fn bright_magenta(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightMagenta);
        self
    }

    pub fn bright_cyan(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightCyan);
        self
    }

    pub fn bright_white(&mut self) -> &mut Self {
        self.fg = Some(Color::BrightWhite);
        self
    }

    pub fn on_bright_black(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightBlack);
        self
    }

    pub fn on_bright_red(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightRed);
        self
    }

    pub fn on_bright_green(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightGreen);
        self
    }

    pub fn on_bright_yellow(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightYellow);
        self
    }

    pub fn on_bright_blue(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightBlue);
        self
    }

    pub fn on_bright_magenta(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightMagenta);
        self
    }

    pub fn on_bright_cyan(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightCyan);
        self
    }

    pub fn on_bright_white(&mut self) -> &mut Self {
        self.bg = Some(Color::BrightWhite);
        self
    }

    fn set_attr(&mut self, attr: Attr) {
        if self.attrs.is_none() {
            self.attrs = Some(vec![attr]);
            return;
        }

        if let Some(ref mut attrs) = self.attrs {
            attrs.push(attr);
        }
    }

    pub fn bold(&mut self) -> &mut Self {
        self.set_attr(Attr::Bold);
        self
    }

    pub fn dim(&mut self) -> &mut Self {
        self.set_attr(Attr::Dim);
        self
    }

    pub fn italics(&mut self) -> &mut Self {
        self.set_attr(Attr::Italics);
        self
    }

    pub fn underlined(&mut self) -> &mut Self {
        self.set_attr(Attr::Underlined);
        self
    }

    pub fn inverted(&mut self) -> &mut Self {
        self.set_attr(Attr::Inverted);
        self
    }

    pub fn hidden(&mut self) -> &mut Self {
        self.set_attr(Attr::Hidden);
        self
    }

    pub fn strike(&mut self) -> &mut Self {
        self.set_attr(Attr::Strike);
        self
    }
}

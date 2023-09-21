use std::{ borrow::Cow, fmt, string };

use crate::{ attr::Attr, color::Color };

/// Represents styled text.
#[derive(Clone, Debug)]
pub struct Style<'a> {
    pub attrs: Option<Cow<'a, [Attr]>>,
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub buffer: Cow<'a, str>,
}

impl<'a> fmt::Display for Style<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_ansi_str() {
            Some(ref ansi) => {
                write!(f, "\x1b[{ansi}{}\x1b[0m", self.buffer)
            },
            None => write!(f, "{}", self.buffer),
        }
    }
}

impl<'a> Style<'a> {
    #[must_use]
    pub fn new(buffer: &'a str) -> Self {
        Self {
            attrs: None,
            bg: None,
            fg: None,
            buffer: buffer.into(),
        }
    }

    fn set_attr(&mut self, attr: Attr) {
        match self.attrs {
            Some(ref mut attrs_vec) => attrs_vec.to_mut().push(attr),
            None => {
                // Reserving a little more than the max number of Attributes
                let mut init_vec: Vec<Attr> = Vec::with_capacity(10);
                init_vec.push(attr);
                self.attrs = Some(init_vec.into());
            },
        }
    }

    /// Returns the configured Style object.
    #[must_use]
    pub fn build(self) -> Self {
        self.clone()
    }

    /// True if a new attribute, foreground color, or background color has been set.
    fn has_new_style(&self) -> bool {
        self.attrs.is_some() || self.fg.is_some() || self.bg.is_some()
    }

    fn get_attrs_ansi(&self) -> Option<String> {
        self.attrs.as_ref().map(|attrs| {
            let attrs_str: Vec<String> =
                attrs.iter().map(string::ToString::to_string).collect();
            attrs_str.join(";")
        })
    }

    fn get_fg_ansi(&self, ansi_str: &mut String) {
        if self.fg.is_none() { return; }

        match self.fg {
            Some(Color::Black) => ansi_str.push_str("30"),
            Some(Color::Red) => ansi_str.push_str("31"),
            Some(Color::Green) => ansi_str.push_str("32"),
            Some(Color::Yellow) => ansi_str.push_str("33"),
            Some(Color::Blue) => ansi_str.push_str("34"),
            Some(Color::Magenta) => ansi_str.push_str("35"),
            Some(Color::Cyan) => ansi_str.push_str("36"),
            Some(Color::White) => ansi_str.push_str("37"),
            Some(Color::Primary) => ansi_str.push_str("39"),
            Some(Color::BriBlack) => ansi_str.push_str("90"),
            Some(Color::BriRed) => ansi_str.push_str("91"),
            Some(Color::BriGreen) => ansi_str.push_str("92"),
            Some(Color::BriYellow) => ansi_str.push_str("93"),
            Some(Color::BriBlue) => ansi_str.push_str("94"),
            Some(Color::BriMagenta) => ansi_str.push_str("95"),
            Some(Color::BriCyan) => ansi_str.push_str("96"),
            Some(Color::BriWhite) => ansi_str.push_str("97"),
            Some(Color::Color256(ref num)) => {
                ansi_str.push_str(&format!("38;5;{num}"))
            },
            Some(Color::Rgb(ref r, ref g, ref b)) =>  {
                ansi_str.push_str(&format!("38;2;{r};{g};{b}"))
            },
            _ => {},
        }
    }

    fn get_bg_ansi(&self, ansi_str: &mut String) {
        if self.bg.is_none() { return; }

        match self.bg {
            Some(Color::Black) => ansi_str.push_str("40"),
            Some(Color::Red) => ansi_str.push_str("41"),
            Some(Color::Green) => ansi_str.push_str("42"),
            Some(Color::Yellow) => ansi_str.push_str("43"),
            Some(Color::Blue) => ansi_str.push_str("44"),
            Some(Color::Magenta) => ansi_str.push_str("45"),
            Some(Color::Cyan) => ansi_str.push_str("46"),
            Some(Color::White) => ansi_str.push_str("47"),
            Some(Color::Primary) => ansi_str.push_str("49"),
            Some(Color::BriBlack) => ansi_str.push_str("100"),
            Some(Color::BriRed) => ansi_str.push_str("101"),
            Some(Color::BriGreen) => ansi_str.push_str("102"),
            Some(Color::BriYellow) => ansi_str.push_str("103"),
            Some(Color::BriBlue) => ansi_str.push_str("104"),
            Some(Color::BriMagenta) => ansi_str.push_str("105"),
            Some(Color::BriCyan) => ansi_str.push_str("106"),
            Some(Color::BriWhite) => ansi_str.push_str("107"),
            Some(Color::Color256(ref num)) => {
                ansi_str.push_str(&format!("48;5;{num}"))
            },
            Some(Color::Rgb(ref r, ref g, ref b)) => {
                ansi_str.push_str(&format!("48;2;{r};{g};{b}"))
            },
            _ => {},
        }
    }

    fn to_ansi_str(&self) -> Option<String> {
        let mut ansi_str = String::new();

        if let Some(ref attrs_ansi) = self.get_attrs_ansi() {
            ansi_str.push_str(attrs_ansi);
            if self.fg.is_some() || self.bg.is_some() {
                ansi_str.push(';');
            }
        }

        if self.fg.is_some() {
            self.get_fg_ansi(&mut ansi_str);
            if self.bg.is_some() {
                ansi_str.push(';');
            }
        }

        if self.bg.is_some() {
            self.get_bg_ansi(&mut ansi_str);
        }

        if self.has_new_style() {
            ansi_str.push('m');
            Some(ansi_str)
        } else {
            None
        }
    }

    pub fn black(mut self) -> Self {
        self.fg = Some(Color::Black);
        self
    }

    pub fn red(mut self) -> Self {
        self.fg = Some(Color::Red);
        self
    }

    pub fn green(mut self) -> Self {
        self.fg = Some(Color::Green);
        self
    }

    pub fn yellow(mut self) -> Self {
        self.fg = Some(Color::Yellow);
        self
    }

    pub fn blue(mut self) -> Self {
        self.fg = Some(Color::Blue);
        self
    }

    pub fn magenta(mut self) -> Self {
        self.fg = Some(Color::Magenta);
        self
    }

    pub fn cyan(mut self) -> Self {
        self.fg = Some(Color::Cyan);
        self
    }

    pub fn white(mut self) -> Self {
        self.fg = Some(Color::White);
        self
    }

    pub fn primary(mut self) -> Self {
        self.fg = Some(Color::Primary);
        self
    }

    pub fn color256(mut self, num: u8) -> Self {
        self.fg = Some(Color::Color256(num));
        self
    }

    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg = Some(Color::Rgb(r, g, b));
        self
    }

    pub fn on_black(mut self) -> Self {
        self.bg = Some(Color::Black);
        self
    }

    pub fn on_red(mut self) -> Self {
        self.bg = Some(Color::Red);
        self
    }

    pub fn on_green(mut self) -> Self {
        self.bg = Some(Color::Green);
        self
    }

    pub fn on_yellow(mut self) -> Self {
        self.bg = Some(Color::Yellow);
        self
    }

    pub fn on_blue(mut self) -> Self {
        self.bg = Some(Color::Blue);
        self
    }

    pub fn on_magenta(mut self) -> Self {
        self.bg = Some(Color::Magenta);
        self
    }

    pub fn on_cyan(mut self) -> Self {
        self.bg = Some(Color::Cyan);
        self
    }

    pub fn on_white(mut self) -> Self {
        self.bg = Some(Color::White);
        self
    }

    pub fn on_primary(mut self) -> Self {
        self.bg = Some(Color::Primary);
        self
    }

    pub fn on_color256(mut self, num: u8) -> Self {
        self.bg = Some(Color::Color256(num));
        self
    }

    pub fn on_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg = Some(Color::Rgb(r, g, b));
        self
    }

    pub fn bright_black(mut self) -> Self {
        self.fg = Some(Color::BriBlack);
        self
    }

    pub fn bright_red(mut self) -> Self {
        self.fg = Some(Color::BriRed);
        self
    }

    pub fn bright_green(mut self) -> Self {
        self.fg = Some(Color::BriGreen);
        self
    }

    pub fn bright_yellow(mut self) -> Self {
        self.fg = Some(Color::BriYellow);
        self
    }

    pub fn bright_blue(mut self) -> Self {
        self.fg = Some(Color::BriBlue);
        self
    }

    pub fn bright_magenta(mut self) -> Self {
        self.fg = Some(Color::BriMagenta);
        self
    }

    pub fn bright_cyan(mut self) -> Self {
        self.fg = Some(Color::BriCyan);
        self
    }

    pub fn bright_white(mut self) -> Self {
        self.fg = Some(Color::BriWhite);
        self
    }

    pub fn on_bright_black(mut self) -> Self {
        self.bg = Some(Color::BriBlack);
        self
    }

    pub fn on_bright_red(mut self) -> Self {
        self.bg = Some(Color::BriRed);
        self
    }

    pub fn on_bright_green(mut self) -> Self {
        self.bg = Some(Color::BriGreen);
        self
    }

    pub fn on_bright_yellow(mut self) -> Self {
        self.bg = Some(Color::BriYellow);
        self
    }

    pub fn on_bright_blue(mut self) -> Self {
        self.bg = Some(Color::BriBlue);
        self
    }

    pub fn on_bright_magenta(mut self) -> Self {
        self.bg = Some(Color::BriMagenta);
        self
    }

    pub fn on_bright_cyan(mut self) -> Self {
        self.bg = Some(Color::BriCyan);
        self
    }

    pub fn on_bright_white(mut self) -> Self {
        self.bg = Some(Color::BriWhite);
        self
    }

    pub fn bold(mut self) -> Self {
        self.set_attr(Attr::Bold);
        self
    }

    pub fn dim(mut self) -> Self {
        self.set_attr(Attr::Dim);
        self
    }

    pub fn italics(mut self) -> Self {
        self.set_attr(Attr::Italics);
        self
    }

    pub fn underline(mut self) -> Self {
        self.set_attr(Attr::Underline);
        self
    }

    pub fn blink(mut self) -> Self {
        self.set_attr(Attr::Blink);
        self
    }

    pub fn inverted(mut self) -> Self {
        self.set_attr(Attr::Inverted);
        self
    }

    pub fn hidden(mut self) -> Self {
        self.set_attr(Attr::Hidden);
        self
    }

    pub fn strike(mut self) -> Self {
        self.set_attr(Attr::Strike);
        self
    }
}

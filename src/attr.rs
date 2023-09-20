use std::fmt;

/// Represents text attribute options.
#[derive(Clone, Copy)]
pub enum Attr {
    Bold,
    Dim,
    Italics,
    Underlined,
    Inverted,
    Hidden,
    Strike,
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bold => write!(f, "1"),
            Self::Dim => write!(f, "2"),
            Self::Italics => write!(f, "3"),
            Self::Underlined => write!(f, "4"),
            Self::Inverted => write!(f, "7"),
            Self::Hidden => write!(f, "8"),
            Self::Strike => write!(f, "9"),
        }
    }
}

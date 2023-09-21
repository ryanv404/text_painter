use std::fmt;

/// Represents text attribute options.
#[derive(Clone, Debug)]
pub enum Attr {
    Bold,
    Dim,
    Italics,
    Underline,
    Blink,
    Inverted,
    Hidden,
    Strike,
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            match *self {
                Self::Bold      => 1,
                Self::Dim       => 2,
                Self::Italics   => 3,
                Self::Underline => 4,
                Self::Blink     => 5,
                Self::Inverted  => 7,
                Self::Hidden    => 8,
                Self::Strike    => 9,
           }
        )
    }
}

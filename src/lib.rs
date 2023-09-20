//! `text_styler`

#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

mod attr;
mod color;
mod macros;

mod style;
pub use crate::style::Style;

#[cfg(test)]
mod tests;

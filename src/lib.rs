#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(future_incompatible)]
#![deny(let_underscore)]
#![deny(nonstandard_style)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(unused)]
#![deny(warnings)]

//! `text_styler`
//! A simple Rust library for writing formatted text to a buffer or the terminal.

mod attr;
mod color;
mod macros;

mod style;
pub use crate::style::Style;

#[cfg(test)]
mod tests;

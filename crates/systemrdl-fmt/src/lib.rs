//! A formatter for SystemRDL.
//!
//! ```text
//! source text
//!     |
//!     v  systemrdl-syntax    lossless CST, comments and all
//!     v  rules               one function per node kind
//!     v  formatter           direct emission into a String
//! ```
//!
//! # Why there is no intermediate representation
//!
//! Pretty-printers usually build a document IR (Wadler groups, Oppen's
//! algorithm) because their layout decisions depend on rendered width: whether
//! a list fits on one line cannot be known until everything inside it has been
//! laid out, so the decision has to be deferred and the alternatives measured.
//!
//! None of the rules here are width-dependent. Following the PeakRDL style
//! guide, braces always break, statements are one per line, expressions never
//! break, and a parenthesised list breaks when it holds more than one element.
//! Every one of those is decidable from the tree alone, before a single
//! character is written. With nothing to defer there is nothing for a document
//! IR to represent, so rules write straight into the output buffer.
//!
//! Each decision that does exist is still isolated in its own function
//! returning a layout, rather than being spelled out inline at the point of
//! emission. Should one of them ever need to consult rendered width, it can
//! render flat into a scratch buffer and measure it -- at the size of a
//! register description the cost of that is not worth an IR to avoid.
//!
//! # What the formatter will not do
//!
//! Reformat a file the parser did not fully understand. [`format`] returns
//! [`FormatError`] when the parse reports errors, because the rules assume a
//! tree shape that error recovery does not guarantee, and rewriting a file
//! whose structure was guessed at is how a formatter corrupts code.

mod formatter;
mod rules;

use formatter::Formatter;
use systemrdl_syntax::{ParseError, parse};

/// Knobs. Deliberately few -- a formatter earns its value by ending arguments,
/// not by relocating them into a config file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatOptions {
    /// Spaces per indentation level. The style guide says four.
    pub indent_width: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions { indent_width: 4 }
    }
}

/// Refusal to format input the parser could not read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatError {
    errors: Vec<ParseError>,
}

impl FormatError {
    /// The parse errors that caused the refusal. Never empty.
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot format input with syntax errors: ")?;
        for (i, err) in self.errors.iter().enumerate() {
            if i > 0 {
                write!(f, "; ")?;
            }
            write!(f, "{err}")?;
        }
        Ok(())
    }
}

impl std::error::Error for FormatError {}

/// Formats SystemRDL source with the default options.
pub fn format(src: &str) -> Result<String, FormatError> {
    format_with(src, &FormatOptions::default())
}

/// Formats SystemRDL source.
///
/// # Errors
/// If `src` does not parse cleanly. The source is left for the caller to
/// report on rather than being passed through unchanged, so that a broken file
/// is never silently mistaken for a formatted one.
pub fn format_with(src: &str, opts: &FormatOptions) -> Result<String, FormatError> {
    let parsed = parse(src);
    if !parsed.errors().is_empty() {
        return Err(FormatError {
            errors: parsed.errors().to_vec(),
        });
    }

    let mut f = Formatter::new(src, opts);
    rules::format_node(&mut f, &parsed.syntax());
    Ok(f.finish())
}

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
//!
//! Preprocessor directives need no separate rule against them, which is the
//! point of treating even the conditionals as trivia: a `` `ifdef `` whose
//! branches hand a brace back and forth leaves the braces unbalanced, and so is
//! refused by the same check as any other input the parser could not follow.
//! Everything else formats like a comment -- its own line, indented with the
//! code around it, payload untouched. See the module docs in
//! [`systemrdl_syntax::parser`] for why ignoring a conditional cannot corrupt
//! the file.

mod formatter;
mod rules;

use formatter::Formatter;
use systemrdl_syntax::{ParseError, SyntaxKind, lex, parse};

/// Why no formatted output was produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    /// The input did not parse. Formatting is refused rather than attempted:
    /// the rules assume a tree shape that error recovery does not guarantee.
    Parse(Vec<ParseError>),
    /// Formatting would have changed the code, not just its layout.
    ///
    /// Always a bug in this crate, never something the input can cause. The
    /// output is withheld so that the bug cannot reach a file.
    Corrupted(String),
}

impl FormatError {
    /// The parse errors that caused the refusal, or empty for other causes.
    pub fn errors(&self) -> &[ParseError] {
        match self {
            FormatError::Parse(errors) => errors,
            FormatError::Corrupted(_) => &[],
        }
    }
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::Parse(errors) => {
                write!(f, "cannot format input with syntax errors: ")?;
                for (i, err) in errors.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{err}")?;
                }
                Ok(())
            }
            FormatError::Corrupted(what) => write!(
                f,
                "internal error: formatting would have changed the code ({what}); \
                 this is a bug in systemrdl-fmt, please report it"
            ),
        }
    }
}

impl std::error::Error for FormatError {}

/// Formats SystemRDL source.
///
/// There is nothing to configure, deliberately: a formatter earns its value by
/// ending arguments, not by relocating them into a config file. Indentation is
/// four spaces, which is what the PeakRDL style guide asks for.
///
/// The output is verified before it is returned: see [`verify`]. A caller that
/// gets `Ok` has a guarantee, not just a hope, that only whitespace moved.
///
/// # Errors
/// [`FormatError::Parse`] if `src` does not parse cleanly. The source is left
/// for the caller to report on rather than being passed through unchanged, so
/// that a broken file is never silently mistaken for a formatted one.
///
/// [`FormatError::Corrupted`] if the formatter has a bug.
pub fn format(src: &str) -> Result<String, FormatError> {
    let parsed = parse(src);
    if !parsed.errors().is_empty() {
        return Err(FormatError::Parse(parsed.errors().to_vec()));
    }

    let mut f = Formatter::new(src);
    rules::format_node(&mut f, &parsed.syntax());
    let out = f.finish();

    verify(src, &out)?;
    Ok(out)
}

/// Checks that formatting moved nothing but whitespace.
///
/// The test suite asserts this over the inputs someone thought to write down.
/// Doing it here instead makes it hold for every input there will ever be,
/// which is what justifies a tool that overwrites source files by default. The
/// cost is one extra lex of the output -- nothing next to the parse that
/// produced it.
///
/// Comments are compared alongside the code, trimmed at the end, because a
/// dropped comment is a real loss even though it changes no behaviour. The
/// trim is what lets the formatter tidy trailing spaces inside one.
fn verify(src: &str, out: &str) -> Result<(), FormatError> {
    let (before, after) = (lex(src), lex(out));
    let keep = |(kind, _): &(SyntaxKind, &str)| *kind != SyntaxKind::WHITESPACE;
    let mut before = before.iter().filter(keep);
    let mut after = after.iter().filter(keep);

    loop {
        return match (before.next(), after.next()) {
            (None, None) => Ok(()),
            (Some((a, at)), Some((b, bt))) if a == b && at.trim_end() == bt.trim_end() => continue,
            (Some((a, at)), Some((b, bt))) => Err(FormatError::Corrupted(format!(
                "{a:?} {at:?} became {b:?} {bt:?}"
            ))),
            (Some((a, at)), None) => Err(FormatError::Corrupted(format!("{a:?} {at:?} was lost"))),
            (None, Some((b, bt))) => Err(FormatError::Corrupted(format!(
                "{b:?} {bt:?} appeared from nowhere"
            ))),
        };
    }
}

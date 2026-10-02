//! A formatter for SystemRDL.
//!
//! ```text
//! source text
//!     |
//!     v  syntax              lossless CST, comments and all
//!     v  rules               one function per node kind
//!     v  formatter           whitespace, then alignment
//!     v  String
//! ```
//!
//! # No document IR
//!
//! Pretty-printers usually build a document IR (Wadler, Oppen) because their
//! line breaks depend on rendered width. None here do: following the PeakRDL
//! style guide, braces always break, statements are one per line, expressions
//! never break, and a parameter list breaks when it holds more than one
//! element. All of that is decidable from the tree alone. Only column
//! alignment needs hindsight, and it runs once every line break is final.
//!
//! # What the formatter will not do
//!
//! Reformat a file the parser did not fully understand: [`format()`] returns
//! [`FormatError`] when the parse reports errors, because rewriting a file
//! whose structure was guessed at is how a formatter corrupts code.
//!
//! Preprocessor directives, conditionals included, are trivia: each keeps its
//! own line, and a conditional is flush left. A `` `ifdef `` whose branches
//! trade a brace leaves the braces unbalanced, and so is refused like any other
//! parse error. See [`crate::syntax::parser`] for why ignoring a conditional
//! cannot corrupt the file.

pub mod syntax;

mod formatter;
mod rules;

use crate::syntax::{ParseError, SyntaxKind, lex, parse};
use formatter::{Formatter, line_ending};

/// Why no formatted output was produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    /// The input did not parse. Formatting is refused rather than attempted:
    /// the rules assume a tree shape that error recovery does not guarantee.
    Parse(Vec<ParseError>),
    /// Formatting would have changed the code, not just its layout. Always a
    /// bug in this crate; the output is withheld so it cannot reach a file.
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
                 this is a bug in rdlfmt, please report it"
            ),
        }
    }
}

impl std::error::Error for FormatError {}

/// Formats SystemRDL source.
///
/// There is nothing to configure, deliberately. Indentation is four spaces,
/// as the PeakRDL style guide asks for.
///
/// A `// rdlfmt: off` comment leaves the statements after it as written, until
/// a `// rdlfmt: on` or the end of the enclosing body; `// rdlfmt: skip` does
/// the same for the one statement below it.
///
/// The output is checked before it is returned: `Ok` guarantees that only
/// whitespace moved.
///
/// # Errors
/// [`FormatError::Parse`] if `src` does not parse cleanly, and
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

/// Checks that formatting moved nothing but whitespace: the output lexes to the
/// same tokens, comments and directives included, and keeps its line endings.
///
/// Checking here rather than only in tests is what makes it safe for the CLI to
/// overwrite files by default. It costs one extra lex of the output.
///
/// Comments and directives are compared trimmed at the end, which lets the
/// formatter drop trailing spaces in them.
fn verify(src: &str, out: &str) -> Result<(), FormatError> {
    // The token comparison below ignores whitespace, so it cannot see these.
    let (want, got) = (line_ending(src), line_ending(out));
    if want != got {
        return Err(FormatError::Corrupted(format!(
            "line endings changed from {want:?} to {got:?}"
        )));
    }

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

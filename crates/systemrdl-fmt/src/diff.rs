//! Showing a formatting change as a diff.
//!
//! Part of the binary, not the library -- `mod diff;` is declared in `main.rs`
//! and nothing in `lib.rs` reaches it.
//!
//! The shape follows `cargo fmt`: a `Diff in <path> at line <n>:` header per
//! hunk, then three lines of context around each change, `-` for what is there
//! now and `+` for what formatting would put there. Familiarity is the point --
//! this is read in the same review as a `cargo fmt --check` failure, and a
//! second diff dialect to learn would be a cost with no benefit.

use anstyle::{AnsiColor, Style};
use similar::{ChangeTag, TextDiff};
use std::io::{self, IsTerminal, Write};
use std::path::Path;

/// Lines of unchanged context on each side of a change. Three is what diff,
/// git and rustfmt all settled on.
const CONTEXT: usize = 3;

/// How to colour the three kinds of diff line.
///
/// Held as styles rather than as a boolean so the rendering below has no
/// conditionals in it: [`Palette::plain`] renders to empty strings, so
/// uncoloured output takes exactly the same path as coloured output.
pub struct Palette {
    header: Style,
    delete: Style,
    insert: Style,
}

impl Palette {
    pub fn plain() -> Palette {
        Palette {
            header: Style::new(),
            delete: Style::new(),
            insert: Style::new(),
        }
    }

    pub fn coloured() -> Palette {
        Palette {
            header: Style::new().bold(),
            delete: AnsiColor::Red.on_default(),
            insert: AnsiColor::Green.on_default(),
        }
    }

    /// The palette for `stream`, honouring the environment.
    ///
    /// `NO_COLOR` wins over everything, then `CLICOLOR_FORCE`, then whether the
    /// stream is actually a terminal -- the order the informal conventions ask
    /// for, and the same answer `clap` reaches for its own help output.
    pub fn for_stream(stream: &impl IsTerminal) -> Palette {
        let set = |name: &str| std::env::var_os(name).filter(|value| !value.is_empty());

        let forced = set("CLICOLOR_FORCE").is_some_and(|value| value != "0");
        if set("NO_COLOR").is_none() && (forced || stream.is_terminal()) {
            Palette::coloured()
        } else {
            Palette::plain()
        }
    }
}

/// Writes the difference between `before` and `after`, attributed to `path`.
///
/// Writes nothing at all when the two are equal, so a caller can use "did this
/// produce output" as the answer to "is this file formatted".
pub fn write(
    out: &mut impl Write,
    path: &Path,
    before: &str,
    after: &str,
    palette: &Palette,
) -> io::Result<()> {
    let diff = TextDiff::from_lines(before, after);

    for group in diff.grouped_ops(CONTEXT) {
        // `grouped_ops` never yields an empty group, so the first op is the
        // one that fixes where this hunk starts in the original file.
        let Some(first) = group.first() else { continue };
        let line = first.old_range().start + 1;

        let header = palette.header;
        writeln!(
            out,
            "{header}Diff in {} at line {line}:{header:#}",
            path.display()
        )?;

        for op in &group {
            for change in diff.iter_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => ('-', palette.delete),
                    ChangeTag::Insert => ('+', palette.insert),
                    ChangeTag::Equal => (' ', Style::new()),
                };
                // `value` carries its own line ending, which would put the
                // reset sequence on the next line if written through.
                let text = change.value().trim_end_matches(['\n', '\r']);
                writeln!(out, "{style}{sign}{text}{style:#}")?;
            }
        }
    }

    Ok(())
}

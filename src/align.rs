//! Column alignment, as a pass over the finished output.
//!
//! # Why this is not a rule
//!
//! Everything else in the formatter is decidable from the tree: how wide a
//! statement renders never changes what the rules do with it. Alignment cannot
//! be, because the padding in front of a cell depends on the widest cell in a
//! *neighbouring* row -- a row the streaming rules have either already emitted
//! or not yet reached.
//!
//! So the rules do not decide it. They only say *where* the boundaries between
//! cells are, by handing the writer a source offset to watch for; the writer
//! records the boundary it lands at as a [`Mark`], and this pass -- which sees
//! the whole file at once -- turns the marks into padding.
//!
//! That keeps the width-dependence out of the rules entirely. A rule still
//! writes straight into the buffer and measures nothing, and the claim in the
//! crate docs survives with one word changed: no layout *decision* is
//! width-dependent. Padding is not a decision. Nothing about which line a token
//! lands on, or how deep it is indented, is settled here -- only how many spaces
//! sit in gaps that already exist.
//!
//! # What may align with what
//!
//! Two rows share a column only if they are the same kind of statement, at the
//! same indentation, on adjacent lines. All three are needed. Aligning a
//! parameter against a register instantiation would be aligning coincidences;
//! aligning across indentation would pad a nested body out to its parent's
//! widths; and aligning across a blank line would override the one thing the
//! author has already said about grouping.
//!
//! A column also ends where a row runs out of cells, which is what keeps a
//! register with no address from stretching the address column of its
//! neighbours.

use std::ops::Range;

/// The kind of row a [`Mark`] belongs to.
///
/// Rows align only within a kind, so this is what stops a `#(...)` element from
/// being padded out to the width of the instantiations below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Column {
    /// `external boot_mode boot_mode @ 0x40;`
    Inst,
    /// `longint unsigned NumClusters = 16,`
    Param,
    /// `IDLE = 0;`
    Enum,
}

/// One boundary between two cells of one row.
///
/// A mark closes the cell to its left and opens the one to its right, and the
/// padding computed for it is inserted at [`Mark::offset`] -- which is why the
/// writer records the offset *after* materialising the gap: the separator is
/// part of the cell being closed, so a column can never come out narrower than
/// the one space that was already there.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Mark {
    /// Output line, counted from the start of the file.
    pub(crate) row: usize,
    /// Byte offset into the output at which padding is inserted.
    pub(crate) offset: usize,
    /// Width of the cell being closed, separator included.
    pub(crate) width: usize,
    /// The row's kind, and its indentation depth.
    pub(crate) key: (Column, usize),
}

/// One row's marks, as a range into the mark list.
struct Row {
    line: usize,
    key: (Column, usize),
    marks: Range<usize>,
}

impl Row {
    fn cells(&self) -> usize {
        self.marks.len()
    }
}

/// Pads `out` so that marked cells line up.
///
/// `marks` must arrive in emission order, which is by row and then by column --
/// the order the writer produces them in.
pub(crate) fn align(out: String, marks: &[Mark]) -> String {
    let rows = rows(marks);
    let widest = rows.iter().map(Row::cells).max().unwrap_or(0);

    // Column by column, because padding one column shifts the offsets of every
    // later one but changes no cell's *width*: a cell is measured from the mark
    // that opens it, so nothing to the left of that mark can reach it.
    let mut pads: Vec<(usize, usize)> = Vec::new();
    for col in 0..widest {
        for run in runs(&rows, col) {
            let cell = |row: &Row| marks[row.marks.start + col];
            let width = rows[run.clone()]
                .iter()
                .map(|r| cell(r).width)
                .max()
                .unwrap_or(0);
            for row in &rows[run] {
                let mark = cell(row);
                if width > mark.width {
                    pads.push((mark.offset, width - mark.width));
                }
            }
        }
    }

    splice(out, &mut pads)
}

/// The marks grouped by the row they sit on.
fn rows(marks: &[Mark]) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for (i, mark) in marks.iter().enumerate() {
        match rows.last_mut() {
            Some(row) if row.line == mark.row => row.marks.end = i + 1,
            _ => rows.push(Row {
                line: mark.row,
                key: mark.key,
                marks: i..i + 1,
            }),
        }
    }
    rows
}

/// The maximal runs of two or more rows that share column `col`.
///
/// A run needs adjacent lines, a matching key, and a cell in this column from
/// every member -- the three conditions from the module docs, in one place.
fn runs(rows: &[Row], col: usize) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut i = 0;

    while i < rows.len() {
        if rows[i].cells() <= col {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < rows.len()
            && rows[j].cells() > col
            && rows[j].key == rows[i].key
            && rows[j].line == rows[j - 1].line + 1
        {
            j += 1;
        }
        // A run of one has nothing to line up with.
        if j - i > 1 {
            out.push(i..j);
        }
        i = j;
    }
    out
}

/// Inserts the padding, back to front so that the offsets ahead of each
/// insertion are still the ones the marks were recorded against.
fn splice(mut out: String, pads: &mut [(usize, usize)]) -> String {
    pads.sort_unstable_by_key(|&(offset, _)| std::cmp::Reverse(offset));
    for &(offset, pad) in pads.iter() {
        out.insert_str(offset, &" ".repeat(pad));
    }
    out
}

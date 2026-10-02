//! The output buffer and the whitespace model.
//!
//! # Separation is requested, not written
//!
//! No rule writes a space or a newline. A rule *requests* a minimum separation
//! before whatever is written next, and the request is materialised when that
//! next thing arrives. Requests combine by [`Ord::max`], so the strongest wins.
//!
//! Everything said about the space between two things accumulates in one
//! [`Gap`], which is spent and reset when something is written. Two properties
//! follow:
//!
//! * **No trailing whitespace.** A separation that is never followed by content
//!   is never written.
//! * **Indentation needs no bookkeeping at the call site.** It is written as
//!   part of a newline, so a rule that indents does not need to know which of
//!   its children begins a line.
//!
//! # Whitespace is discarded, its signal is not
//!
//! Source whitespace is never copied; the formatter regenerates all of it. The
//! one thing it carries that cannot be recomputed is whether the author left a
//! blank line, which is kept as the gap's [`Width`]. Whether a gap breaks the
//! line is the rule's decision; a blank line only widens a break the rule
//! already asked for. That is why `addrmap top` and a `{` written two lines
//! below it still end up on one line.
//!
//! # Alignment
//!
//! Column alignment is the one decision that needs hindsight. As rules write,
//! they mark rows and the cell boundaries within them; once every newline is
//! final, [`Formatter::align`] pads adjacent one-line rows of the same kind.
//! Padding never feeds back into layout.

use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::TextSize;
use std::collections::{BTreeMap, BTreeSet};

/// Spaces per indentation level, as the PeakRDL style guide asks for.
const INDENT_WIDTH: usize = 4;

/// The minimum separation required before the next thing written.
///
/// Variant order matters: requests combine with [`Ord::max`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Sep {
    /// Tokens abut: `8'hA5`, `foo[`.
    #[default]
    None,
    /// A single space: around `=`, between `reg` and its name.
    Space,
    /// End the line.
    Newline,
}

/// How wide the gap should be *if* it turns out to be a line break.
///
/// Kept apart from [`Sep`] because a blank line can only widen a break, never
/// create one: ordering it above `Sep::Newline` would let a blank line in front
/// of a `{` strand the brace on a line of its own.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Width {
    /// Nobody has spoken for it. A blank line in the source still can.
    #[default]
    Open,
    /// The author left a blank line here.
    Blank,
    /// A rule has settled it: one line break, whatever the source had.
    Settled,
}

/// The separation accumulating in front of whatever is written next.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Gap {
    sep: Sep,
    width: Width,
}

/// The kind of statement a row holds. Only rows of the same family align with
/// each other; an `Other` row ends any run it interrupts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RowFamily {
    Instantiation,
    ParameterDefinition,
    EnumEntry,
    Other,
}

/// The right edge of a cell, named after what follows it. Naming rather than
/// numbering the columns lets a row omit a cell without shifting the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum AlignPoint {
    InstType,
    InstName,
    InstReset,
    InstAddress,
    InstStride,
    InstAlign,
    ParamName,
    ParamDefault,
    EnumValue,
    TrailingComment,
}

#[derive(Debug, Clone, Copy)]
struct Marker {
    point: AlignPoint,
    pos: usize,
    /// Whether a space was already written here.
    base_space: bool,
}

/// A place in the output, with the facts about its line that alignment needs.
/// `out` only grows until [`Formatter::align`], so they stay true until then.
#[derive(Debug, Clone, Copy)]
struct Pos {
    byte: usize,
    /// How many line breaks precede it.
    line: usize,
    /// Whether nothing but indentation precedes it on its line.
    starts_line: bool,
}

#[derive(Debug)]
struct Row {
    family: RowFamily,
    scope: usize,
    /// Where the first token or cell boundary landed. Set late, because leading
    /// trivia arrives after [`Formatter::begin_row`].
    start: Option<Pos>,
    end: Option<Pos>,
    markers: Vec<Marker>,
    /// Whether a blank line or a directive separates this row from the previous
    /// row of its scope.
    after_break: bool,
}

impl Row {
    /// Whether the row can join an aligned run: a line of its own, with at
    /// least one cell boundary.
    fn alignable(&self) -> bool {
        self.family != RowFamily::Other
            && !self.markers.is_empty()
            && matches!(
                (self.start, self.end),
                (Some(start), Some(end)) if start.line == end.line && start.starts_line
            )
    }
}

/// The rows of one body or parameter list. Rows only align within a scope.
#[derive(Debug, Default)]
struct Scope {
    rows: Vec<usize>,
    /// Whether a blank line or a directive has been written since a row of this
    /// scope last started.
    broken: bool,
}

#[derive(Debug, Clone, Copy)]
struct PendingMarker {
    row: usize,
    point: AlignPoint,
}

pub(crate) struct Formatter<'a> {
    /// The source, for [`Formatter::verbatim`].
    src: &'a str,
    /// Written only through [`Formatter::push`] until [`Formatter::finish`], so
    /// that the two fields below stay in step with it.
    out: String,
    /// How many line breaks `out` holds.
    lines: usize,
    /// Whether the last line of `out` holds nothing but whitespace so far.
    line_blank: bool,
    /// Indentation depth, in levels.
    indent: usize,
    /// The gap in front of the next thing written.
    gap: Gap,
    /// Whether blank lines are kept in the current region. See
    /// [`Formatter::allow_blank_lines`].
    blank_lines: bool,
    /// Whether the source had a newline since the last token, which tells a
    /// trailing comment from one that introduces what follows.
    saw_newline: bool,
    /// Whether the last thing written was a comment.
    after_comment: bool,
    /// The line ending to write. See [`line_ending`].
    eol: &'static str,
    rows: Vec<Row>,
    row_stack: Vec<usize>,
    scopes: Vec<Scope>,
    scope_stack: Vec<usize>,
    pending_markers: Vec<PendingMarker>,
}

/// The line ending a file uses, judged by its first line break.
///
/// Imposing one would turn formatting a CRLF file into a whole-file diff. A
/// mixed file gets the ending of its first break.
pub(crate) fn line_ending(src: &str) -> &'static str {
    match src.find('\n') {
        Some(i) if src.as_bytes()[..i].last() == Some(&b'\r') => "\r\n",
        _ => "\n",
    }
}

impl<'a> Formatter<'a> {
    pub(crate) fn new(src: &'a str) -> Self {
        Formatter {
            src,
            out: String::with_capacity(src.len()),
            lines: 0,
            line_blank: true,
            indent: 0,
            gap: Gap::default(),
            blank_lines: true,
            saw_newline: false,
            after_comment: false,
            eol: line_ending(src),
            rows: Vec::new(),
            row_stack: Vec::new(),
            scopes: vec![Scope::default()],
            scope_stack: vec![0],
            pending_markers: Vec::new(),
        }
    }

    /// Aligns the output and ends it with exactly one newline, or with nothing
    /// if it is empty.
    pub(crate) fn finish(mut self) -> String {
        self.align();
        let trimmed = self.out.trim_end().len();
        self.out.truncate(trimmed);
        if !self.out.is_empty() {
            self.out.push_str(self.eol);
        }
        self.out
    }

    //----------------------------------------------------------------------
    // Separation
    //----------------------------------------------------------------------

    /// Asks for at least `sep` before the next thing written.
    pub(crate) fn request(&mut self, sep: Sep) {
        self.gap.sep = self.gap.sep.max(sep);
    }

    /// Notes that the author left a blank line in the gap now open.
    ///
    /// Widens a break if the gap becomes one; ignored if a rule has settled the
    /// width or blank lines are off in this region.
    pub(crate) fn blank_line(&mut self) {
        if self.blank_lines && self.gap.width == Width::Open {
            self.gap.width = Width::Blank;
        }
    }

    /// Forces the separation to exactly `sep` and settles the width, for where
    /// the accumulated request is wrong rather than too weak: a trailing comment
    /// stays on its line whatever break was pending, and a closing brace starts
    /// a line whatever the last item left.
    pub(crate) fn pin(&mut self, sep: Sep) {
        self.gap = Gap {
            sep,
            width: Width::Settled,
        };
    }

    /// Settles the width of the gap now open without changing its separation.
    ///
    /// For the gap after an opening bracket: its whitespace belongs to the item
    /// that follows, so the bracket's rule cannot yet say whether it breaks.
    pub(crate) fn settle_width(&mut self) {
        self.gap.width = Width::Settled;
    }

    /// Sets whether blank lines survive in the region being formatted, and
    /// returns the previous setting for the caller to restore.
    ///
    /// Blank lines group statements, which is the author's call, so a body
    /// keeps them. Between the elements of a parameter list they say nothing,
    /// so a broken list turns them off.
    pub(crate) fn allow_blank_lines(&mut self, allow: bool) -> bool {
        std::mem::replace(&mut self.blank_lines, allow)
    }

    //----------------------------------------------------------------------
    // Alignment structure
    //----------------------------------------------------------------------

    /// Opens an alignment scope, so that rows in a nested body or parameter
    /// list never align with those outside it.
    pub(crate) fn open_alignment_scope(&mut self) {
        let id = self.scopes.len();
        self.scopes.push(Scope::default());
        self.scope_stack.push(id);
    }

    pub(crate) fn close_alignment_scope(&mut self) {
        debug_assert!(self.scope_stack.len() > 1);
        self.scope_stack.pop();
    }

    /// Begins a row in the current scope. Its leading trivia may follow.
    pub(crate) fn begin_row(&mut self, family: RowFamily) {
        let id = self.rows.len();
        let scope = *self.scope_stack.last().expect("root alignment scope");
        self.rows.push(Row {
            family,
            scope,
            start: None,
            end: None,
            markers: Vec::new(),
            after_break: false,
        });
        self.scopes[scope].rows.push(id);
        self.row_stack.push(id);
    }

    pub(crate) fn end_row(&mut self) {
        let row = self.row_stack.pop().expect("end_row without begin_row");
        // A marker with no following token cannot close a cell.
        self.pending_markers.retain(|marker| marker.row != row);
    }

    /// Marks the gap now open as the right edge of a cell in the current row.
    /// The marker is placed when the next thing is written, after any line
    /// break the gap turns into.
    pub(crate) fn align_before(&mut self, point: AlignPoint) {
        if let Some(&row) = self.row_stack.last() {
            self.pending_markers.push(PendingMarker { row, point });
        }
    }

    //----------------------------------------------------------------------
    // Indentation
    //----------------------------------------------------------------------

    pub(crate) fn indent(&mut self) {
        self.indent += 1;
    }

    pub(crate) fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    /// Writes the separation the gap now open asks for, and spends it.
    fn materialize(&mut self) {
        let gap = std::mem::take(&mut self.gap);
        // Nothing to separate from, so a file's leading comment is not pushed
        // off the first line.
        if self.out.is_empty() {
            return;
        }
        match gap.sep {
            Sep::None => self.materialize_markers(false),
            Sep::Space => {
                self.materialize_markers(true);
                self.push(" ");
            }
            Sep::Newline => {
                self.newline(if gap.width == Width::Blank { 2 } else { 1 });
                self.materialize_markers(false);
            }
        }
    }

    fn materialize_markers(&mut self, base_space: bool) {
        for pending in std::mem::take(&mut self.pending_markers) {
            self.start_row(pending.row);
            self.rows[pending.row].markers.push(Marker {
                point: pending.point,
                pos: self.out.len(),
                base_space,
            });
        }
    }

    fn newline(&mut self, count: usize) {
        for _ in 0..count {
            self.push(self.eol);
        }
        self.push(&" ".repeat(self.indent * INDENT_WIDTH));
    }

    /// Appends `text` to `out`, keeping the line bookkeeping in step.
    fn push(&mut self, text: &str) {
        let mut rest = text;
        while let Some(i) = rest.find('\n') {
            self.extend_line(&rest[..i]);
            // An empty line -- a blank line, or one inside a comment -- ends
            // an aligned run.
            if self.line_blank {
                self.break_run();
            }
            self.out.push('\n');
            self.lines += 1;
            self.line_blank = true;
            rest = &rest[i + 1..];
        }
        self.extend_line(rest);
    }

    /// Appends text that holds no line break.
    fn extend_line(&mut self, text: &str) {
        self.line_blank = self.line_blank && text.chars().all(char::is_whitespace);
        self.out.push_str(text);
    }

    /// The current end of `out`.
    fn cursor(&self) -> Pos {
        Pos {
            byte: self.out.len(),
            line: self.lines,
            starts_line: self.line_blank,
        }
    }

    /// Sets where `row` starts, if that is not yet known, and takes over its
    /// scope's note of a break before it.
    fn start_row(&mut self, row: usize) {
        if self.rows[row].start.is_none() {
            let scope = self.rows[row].scope;
            self.rows[row].start = Some(self.cursor());
            self.rows[row].after_break = std::mem::take(&mut self.scopes[scope].broken);
        }
    }

    /// Ends the aligned run in the current scope.
    fn break_run(&mut self) {
        let scope = *self.scope_stack.last().expect("root alignment scope");
        self.scopes[scope].broken = true;
    }

    //----------------------------------------------------------------------
    // Writing
    //----------------------------------------------------------------------

    /// Writes `text` exactly as given, after any pending separation.
    fn write_raw(&mut self, text: &str) {
        self.materialize();
        self.push(text);
    }

    /// Records what was just written, for the comment that may come next.
    fn wrote(&mut self, comment: bool) {
        self.saw_newline = false;
        self.after_comment = comment;
    }

    /// Writes a significant token.
    ///
    /// The text is copied rather than rebuilt from the kind: `~^` and `^~` are
    /// both `XNOR`, and which one the author wrote is theirs to choose.
    pub(crate) fn token(&mut self, tok: &SyntaxToken) {
        debug_assert!(!tok.kind().is_trivia(), "trivia must go through trivia()");
        // A terminator attaches to a block comment as it would to code. The
        // space a comment keeps after itself gives way, but a line break the
        // author put there does not.
        if self.after_comment
            && self.gap.sep == Sep::Space
            && matches!(tok.kind(), SyntaxKind::SEMICOLON | SyntaxKind::COMMA)
        {
            self.gap.sep = Sep::None;
        }
        self.materialize();
        for i in 0..self.row_stack.len() {
            self.start_row(self.row_stack[i]);
        }
        self.push(tok.text());
        let end = self.cursor();
        for &row in &self.row_stack {
            self.rows[row].end = Some(end);
        }
        self.wrote(false);
    }

    /// Handles one trivia token: drops whitespace, keeps comments and
    /// preprocessor directives.
    pub(crate) fn trivia(&mut self, tok: &SyntaxToken) {
        match tok.kind() {
            SyntaxKind::WHITESPACE => {
                let newlines = tok.text().bytes().filter(|&b| b == b'\n').count();
                // Any run of blank lines counts as one.
                if newlines >= 2 {
                    self.blank_line();
                } else if newlines == 1 && self.after_comment {
                    // Whether a comment ended its line is the author's call,
                    // since no rule governs what follows a comment.
                    self.request(Sep::Newline);
                }
                self.saw_newline |= newlines >= 1;
            }
            kind if kind.is_directive() => {
                // Requested rather than pinned, so a blank line in front of a
                // directive survives.
                self.request(Sep::Newline);
                // A branching directive is always flush left.
                let saved_indent = self.indent;
                if kind == SyntaxKind::COND_DIRECTIVE {
                    self.indent = 0;
                }
                // Trailing spaces are trimmed, except back to a final
                // backslash: there the token either holds a continuation's line
                // break, which is part of it, or spaces that are all that stop
                // the backslash from continuing onto the next line.
                let text = tok.text();
                let trimmed = text.trim_end();
                self.write_raw(if trimmed.ends_with('\\') {
                    text
                } else {
                    trimmed
                });
                self.indent = saved_indent;
                if text.ends_with('\n') && trimmed.ends_with('\\') {
                    // The continuation already opened the empty line that ends
                    // the macro, so that is the blank line.
                    self.settle_width();
                }
                self.break_run();
                // Whatever follows a directive must start a new line, or it
                // becomes part of the directive.
                self.request(Sep::Newline);
                self.wrote(false);
            }
            kind if kind.is_comment() => {
                // A comment after a newline introduces what follows and gets a
                // line of its own; one without stays beside what it trails.
                let inline = !self.saw_newline;
                if self.saw_newline {
                    self.request(Sep::Newline);
                } else if kind == SyntaxKind::LINE_COMMENT {
                    // Pinned, because the parser hands a comment to the item
                    // after it, so in `reg r { // why` the body's newline is
                    // already pending. That is safe only for a line comment:
                    // nothing can follow it on its line.
                    self.pin(Sep::Space);
                } else {
                    self.request(Sep::Space);
                }
                // A trailing comment belongs to the row on its line, unless a
                // pending break moves it to the next one.
                if inline && self.gap.sep != Sep::Newline {
                    self.attach_trailing_comment();
                }
                if kind == SyntaxKind::LINE_COMMENT {
                    self.write_raw(tok.text().trim_end());
                } else {
                    self.write_raw(tok.text());
                }
                if kind == SyntaxKind::LINE_COMMENT {
                    // Anything after a line comment must start a new line, or
                    // it is commented out.
                    self.request(Sep::Newline);
                } else {
                    // Code may follow a block comment on its line; this keeps
                    // `*/` off the next token.
                    self.request(Sep::Space);
                }
                self.wrote(true);
            }
            kind => unreachable!("not trivia: {kind:?}"),
        }
    }

    /// Reproduces `node` as it appears in the source, for statements under
    /// `rdlfmt: off` or `skip`, and for [`SyntaxKind::ERROR`] nodes, which a
    /// successful format never contains.
    ///
    /// Trivia at either end still goes through [`Formatter::trivia`], because
    /// it belongs to the surrounding layout: the source's indentation in front
    /// of the node, or its padding before a trailing comment, is not kept.
    pub(crate) fn verbatim(&mut self, node: &SyntaxNode) {
        let src = self.src;
        let mut start = node.text_range().start();
        let mut end = node.text_range().end();

        for tok in leading_trivia(node) {
            self.trivia(&tok);
            start = tok.text_range().end();
        }
        // Bounded below by `start`, so a node that is all trivia is not
        // emitted twice.
        let trailing = trailing_trivia(node, start);
        if let Some(first) = trailing.first() {
            end = first.text_range().start();
        }

        if start < end {
            self.write_raw(&src[usize::from(start)..usize::from(end)]);
            self.wrote(false);
        }
        for tok in &trailing {
            self.trivia(tok);
        }
    }

    /// Marks the comment about to be written as the trailing comment of the
    /// last row on the current line, if there is one.
    fn attach_trailing_comment(&mut self) {
        let line = self.lines;
        let Some(row) = self
            .rows
            .iter_mut()
            .rev()
            .find(|row| row.end.is_some_and(|end| end.line == line))
        else {
            return;
        };

        // `trivia` has already asked for a space in front of the comment.
        row.markers.push(Marker {
            point: AlignPoint::TrailingComment,
            pos: self.out.len(),
            base_space: true,
        });
    }

    /// Pads each run of alignable rows and inserts the padding into `out`.
    fn align(&mut self) {
        // A comment that more of its row followed is part of a cell, not a
        // trailing comment. That was not known when it was written.
        for row in &mut self.rows {
            if let Some(end) = row.end {
                row.markers.retain(|marker| {
                    marker.point != AlignPoint::TrailingComment || marker.pos >= end.byte
                });
            }
        }

        let mut insertions: BTreeMap<usize, usize> = BTreeMap::new();

        for scope in &self.scopes {
            let mut run: Vec<usize> = Vec::new();
            for &id in &scope.rows {
                let row = &self.rows[id];
                let continues = row.alignable()
                    && !row.after_break
                    && run
                        .last()
                        .is_some_and(|&last| self.rows[last].family == row.family);
                if !continues {
                    self.align_run(&run, &mut insertions);
                    run.clear();
                }
                if row.alignable() {
                    run.push(id);
                }
            }
            self.align_run(&run, &mut insertions);
        }

        if insertions.is_empty() {
            return;
        }

        let mut aligned =
            String::with_capacity(self.out.len() + insertions.values().copied().sum::<usize>());
        let mut cursor = 0;
        for (pos, count) in insertions {
            aligned.push_str(&self.out[cursor..pos]);
            aligned.extend(std::iter::repeat_n(' ', count));
            cursor = pos;
        }
        aligned.push_str(&self.out[cursor..]);
        self.out = aligned;
    }

    /// Pads every column of a run, where a column is a stretch of consecutive
    /// rows that all have a given cell.
    fn align_run(&self, run: &[usize], insertions: &mut BTreeMap<usize, usize>) {
        if run.len() < 2 {
            return;
        }

        // Visited in `AlignPoint` order, which puts the trailing comment last:
        // its column depends on the padding of every cell before it.
        let points: BTreeSet<AlignPoint> = run
            .iter()
            .flat_map(|&id| self.rows[id].markers.iter().map(|marker| marker.point))
            .collect();
        for point in points {
            let mut group: Vec<(usize, Marker)> = Vec::new();
            for &row_id in run {
                let marker = self.rows[row_id]
                    .markers
                    .iter()
                    .find(|marker| marker.point == point)
                    .copied();
                if let Some(marker) = marker {
                    group.push((row_id, marker));
                } else {
                    self.align_column_group(&group, insertions);
                    group.clear();
                }
            }
            self.align_column_group(&group, insertions);
        }
    }

    fn align_column_group(
        &self,
        group: &[(usize, Marker)],
        insertions: &mut BTreeMap<usize, usize>,
    ) {
        if group.len() < 2 {
            return;
        }

        let widths: Vec<usize> = group
            .iter()
            .map(|&(row_id, marker)| {
                let row = &self.rows[row_id];
                if marker.point == AlignPoint::TrailingComment {
                    self.line_width(row, marker, insertions)
                } else {
                    self.cell_width(row, marker)
                }
            })
            .collect();
        let maximum = widths.iter().copied().max().unwrap_or(0);
        let separator = usize::from(maximum > 0);

        for ((_, marker), width) in group.iter().zip(widths) {
            let base = usize::from(marker.base_space);
            let padding = maximum - width + separator.saturating_sub(base);
            if padding > 0 {
                insertions
                    .entry(marker.pos)
                    .and_modify(|old| *old = (*old).max(padding))
                    .or_insert(padding);
            }
        }
    }

    /// The width of `row` up to `marker`, including the padding already
    /// decided for its cells. Trailing comments line up on this rather than on
    /// a cell, because rows may have different cells before them.
    fn line_width(&self, row: &Row, marker: Marker, insertions: &BTreeMap<usize, usize>) -> usize {
        let start = row.start.map_or(marker.pos, |start| start.byte);
        self.out[start..marker.pos].chars().count()
            + insertions
                .range(start..marker.pos)
                .map(|(_, n)| n)
                .sum::<usize>()
    }

    /// The width of the cell that ends at `marker`.
    fn cell_width(&self, row: &Row, marker: Marker) -> usize {
        let start = row
            .markers
            .iter()
            .filter(|candidate| candidate.pos < marker.pos)
            .map(|candidate| candidate.pos)
            .max()
            .or(row.start.map(|start| start.byte))
            .unwrap_or(marker.pos);
        self.out[start..marker.pos]
            .trim_start_matches([' ', '\t'])
            .chars()
            .count()
    }
}

/// The run of trivia at the start of `node`. It sits on the leftmost token,
/// however deep that is.
pub(crate) fn leading_trivia(node: &SyntaxNode) -> impl Iterator<Item = SyntaxToken> {
    tokens(node).take_while(|tok| tok.kind().is_trivia())
}

/// Every token of `node`, in source order.
pub(crate) fn tokens(node: &SyntaxNode) -> impl Iterator<Item = SyntaxToken> {
    let end = node.text_range().end();
    std::iter::successors(node.first_token(), |tok: &SyntaxToken| tok.next_token())
        .take_while(move |tok| tok.text_range().end() <= end)
}

/// The run of trivia at the end of `node`, in source order, starting no
/// earlier than `floor`.
fn trailing_trivia(node: &SyntaxNode, floor: TextSize) -> Vec<SyntaxToken> {
    let mut out: Vec<SyntaxToken> =
        std::iter::successors(node.last_token(), |tok: &SyntaxToken| tok.prev_token())
            .take_while(|tok| tok.text_range().start() >= floor && tok.kind().is_trivia())
            .collect();
    out.reverse();
    out
}

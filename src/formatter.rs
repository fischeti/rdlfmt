//! The output buffer and the whitespace model.
//!
//! # Separation is requested, not written
//!
//! No rule ever writes a space or a newline. Instead it *requests* a minimum
//! separation before whatever is written next, and the request is materialised
//! lazily when that next thing actually arrives. Requests combine by [`Ord`]:
//! the strongest one wins.
//!
//! Everything said about the space between two things accumulates in one value,
//! the [`Gap`], which is spent and reset the moment something is written. That
//! is the whole of the mutable whitespace state: a rule can only speak about
//! the gap now open, and nothing it says can outlive it.
//!
//! Two properties fall out of this, both of which are otherwise fiddly:
//!
//! * **No trailing whitespace, ever.** A separation that is never followed by
//!   content is never written, so a request left pending at the end of a line
//!   or of the file simply evaporates.
//! * **Indentation needs no bookkeeping at the call site.** It is emitted as
//!   part of materialising a newline, so a rule that opens an indent level
//!   does not have to know which of its children begins a line.
//!
//! It also gives the two producers of separation -- layout rules and preserved
//! trivia -- a way to disagree without either having to know about the other.
//! A blank line in the source and a rule asking for a plain newline resolve to
//! a blank line without the rule being consulted.
//!
//! # Whitespace is discarded, its signal is not
//!
//! Source `WHITESPACE` tokens are never copied to the output; the formatter
//! regenerates all of it. The one thing they carry that cannot be recomputed is
//! whether the author left a blank line, so that -- and only that -- is lifted
//! out before the token is dropped.
//!
//! It is lifted out as the gap's [`Width`] rather than as a separation in its
//! own right, because a blank line is a bigger line break and not something
//! that can stand where there was to be no break at all. Whether a gap is a
//! break is the enclosing rule's decision; the author's blank line only says
//! how wide it should be once the rule has decided on one. That is what keeps
//! `addrmap top` and a `{` written two lines below it on one line: the gap in
//! front of a brace is a space however many newlines were typed into it.
//!
//! Where a break *is* the author's to widen is a separate question -- policy
//! for a whole region rather than state of one gap -- and the one thing here
//! that outlives a gap. See [`Formatter::allow_blank_lines`].

use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::TextSize;
use std::collections::BTreeMap;

/// Spaces per indentation level, as the PeakRDL style guide asks for.
///
/// A constant rather than an option: an indent width is the kind of setting
/// that exists only to be argued over, and every file the formatter touches
/// having the same one is the point of running it.
const INDENT_WIDTH: usize = 4;

/// The minimum separation required before the next thing written.
///
/// Variant order is load-bearing: requests combine with [`Ord::max`], so a
/// stronger request always survives a weaker one regardless of arrival order.
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
/// The second axis of a gap, and deliberately not part of the [`Sep`] lattice:
/// a blank line is a bigger break, not a break in its own right, so it can only
/// widen a break someone else decided on. Ordering it above [`Sep::Newline`]
/// and taking the max would let a blank line typed in front of a `{` strand the
/// brace on a line of its own.
///
/// Three states rather than a pair of flags, because two booleans would admit a
/// fourth that means nothing.
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
///
/// One value with one lifetime: it is built up by requests and by the author's
/// whitespace, spent by [`Formatter::materialize`], and reset there -- which is
/// what keeps a decision about one gap from leaking into the next without
/// anyone having to clear a flag by hand.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Gap {
    sep: Sep,
    width: Width,
}

/// A kind of physical row that may participate in an aligned run.
///
/// `Other` is deliberately represented too: a statement which cannot be a row
/// is still a boundary between the rows on either side of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RowFamily {
    Instantiation,
    ParameterDefinition,
    EnumEntry,
    Other,
}

/// The right edge of a semantic cell.
///
/// The value names the thing that follows the cell. Keeping the names semantic
/// rather than numbering columns lets a row omit a later cell without shifting
/// everything after it into the wrong column.
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
    /// Whether the ordinary formatter already put a separating space here.
    base_space: bool,
}

#[derive(Debug)]
struct Row {
    family: RowFamily,
    start: Option<usize>,
    end: Option<usize>,
    markers: Vec<Marker>,
}

#[derive(Debug, Default)]
struct Scope {
    rows: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct PendingMarker {
    row: usize,
    point: AlignPoint,
}

pub(crate) struct Formatter<'a> {
    /// Kept so that [`Formatter::verbatim`] can slice out a node's original
    /// text by byte range. Once every kind has a rule this goes away.
    src: &'a str,
    out: String,
    /// Current indentation depth, in levels rather than columns.
    indent: usize,
    /// The gap in front of the next thing written.
    gap: Gap,
    /// Whether blank lines mean anything where we currently are. Policy for a
    /// whole region rather than state of one gap, which is why it sits out here
    /// and survives being spent; see [`Formatter::allow_blank_lines`].
    blank_lines: bool,
    /// Whether a newline has been seen in the source since the last real
    /// token. This is how a comment tells a trailing annotation (`sw = rw; //
    /// writable`) from one that introduces what follows.
    saw_newline: bool,
    /// Whether the last thing written was a comment still waiting to find out
    /// what separated it from what follows. See [`Formatter::trivia`].
    after_comment: bool,
    /// The line ending to write, taken from the input. See [`line_ending`].
    eol: &'static str,
    /// Alignment metadata over `out`. Rules still emit in one pass; these byte
    /// positions are the small IR retained until `finish` can see every row.
    rows: Vec<Row>,
    row_stack: Vec<usize>,
    scopes: Vec<Scope>,
    scope_stack: Vec<usize>,
    pending_markers: Vec<PendingMarker>,
}

/// The line ending a file uses, judged by its first line break.
///
/// A formatter that imposed its own would rewrite every line of a CRLF file,
/// turning a whitespace tidy-up into a whole-file diff for anyone on Windows.
/// So the input decides, and there is nothing to configure.
///
/// Mixed files are settled by the first break rather than by counting. It is
/// the ending the file already looks like it has, and a tie needs no rule.
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

    /// Finishes the file: exactly one trailing newline, or nothing at all if
    /// there was no content.
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
    /// Not a request: it widens the gap rather than opening one, and if the
    /// separation is still a space or nothing by the time something is written,
    /// this is discarded along with the rest of the whitespace it came from.
    ///
    /// Ignored where a rule has already settled the width, and where blank
    /// lines carry nothing worth keeping. Both are checked here rather than at
    /// the call site because there is only one call site: whitespace, which
    /// knows what the author typed and nothing about where it landed.
    pub(crate) fn blank_line(&mut self) {
        if self.blank_lines && self.gap.width == Width::Open {
            self.gap.width = Width::Blank;
        }
    }

    /// Forces the separation to exactly `sep`, and settles the width with it.
    ///
    /// The counterpart to [`Formatter::request`], for the cases where the
    /// accumulated minimum is not merely too weak but wrong: a trailing comment
    /// belongs on the line it annotates however much the enclosing rule wanted
    /// a break there, and a closing brace starts a line whatever the last item
    /// left pending.
    pub(crate) fn pin(&mut self, sep: Sep) {
        self.gap = Gap {
            sep,
            width: Width::Settled,
        };
    }

    /// Settles the width of the gap now open without touching its separation.
    ///
    /// [`pin`](Formatter::pin) with no opinion on whether the gap breaks, for
    /// the one place that cannot use it: the whitespace after an opening brace
    /// is not a child of the braced node -- the parser hands trivia to the item
    /// that follows it -- so it arrives partway down a recursion the rule has
    /// already entered, by which time saying `Sep::Newline` would be guessing at
    /// what that item wanted.
    ///
    /// Spent with the gap, so it speaks for that one gap and no further.
    pub(crate) fn settle_width(&mut self) {
        self.gap.width = Width::Settled;
    }

    /// Sets whether blank lines survive in the region being formatted, and
    /// returns the previous setting for the caller to restore.
    ///
    /// The counterpart to [`Formatter::settle_width`], which speaks for one
    /// gap; this speaks for everything nested inside a construct, which is what
    /// it takes to cover gaps that arrive several levels down.
    ///
    /// A blank line is grouping, and grouping says something only between
    /// things that stand on their own. Statements do, so a body keeps them:
    /// which registers belong together is the author's to say and not something
    /// the formatter could work out. The elements of a comma-separated list do
    /// not -- they are parts of one construct, laid out one per line because it
    /// grew too long -- so a blank line between two parameters is noise, and
    /// dropping it is the only thing this is currently used for.
    pub(crate) fn allow_blank_lines(&mut self, allow: bool) -> bool {
        std::mem::replace(&mut self.blank_lines, allow)
    }

    //----------------------------------------------------------------------
    // Alignment structure
    //----------------------------------------------------------------------

    /// Opens a list-local alignment scope. Rows in nested bodies and parameter
    /// lists must never contribute widths to their enclosing list.
    pub(crate) fn open_alignment_scope(&mut self) {
        let id = self.scopes.len();
        self.scopes.push(Scope::default());
        self.scope_stack.push(id);
    }

    pub(crate) fn close_alignment_scope(&mut self) {
        debug_assert!(self.scope_stack.len() > 1);
        self.scope_stack.pop();
    }

    /// Begins one candidate row. Leading trivia is allowed to arrive after
    /// this call: the row starts only when `token` sees its first real token.
    pub(crate) fn begin_row(&mut self, family: RowFamily) {
        let id = self.rows.len();
        self.rows.push(Row {
            family,
            start: None,
            end: None,
            markers: Vec::new(),
        });
        let scope = *self.scope_stack.last().expect("root alignment scope");
        self.scopes[scope].rows.push(id);
        self.row_stack.push(id);
    }

    pub(crate) fn end_row(&mut self) {
        let row = self.row_stack.pop().expect("end_row without begin_row");
        // A marker with no following token cannot close a cell.
        self.pending_markers.retain(|marker| marker.row != row);
    }

    /// Marks the pending gap as the right edge of the current semantic cell.
    /// It is attached when the next significant token arrives, so comments in
    /// leading trivia cannot steal a statement's first boundary.
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

    fn materialize(&mut self) {
        // Taken whether or not it is used: a gap describes the space between
        // two things, and once one of them is written it is spent either way.
        let gap = std::mem::take(&mut self.gap);
        // Nothing to separate from. This is what keeps a leading comment from
        // being pushed off the first line by the newline the caller requested
        // before it.
        if self.out.is_empty() {
            return;
        }
        match gap.sep {
            Sep::None => self.materialize_markers(false),
            Sep::Space => {
                self.materialize_markers(true);
                self.out.push(' ');
            }
            Sep::Newline => {
                self.newline(if gap.width == Width::Blank { 2 } else { 1 });
                self.materialize_markers(false);
            }
        }
    }

    fn materialize_markers(&mut self, base_space: bool) {
        for pending in self.pending_markers.drain(..) {
            self.rows[pending.row].markers.push(Marker {
                point: pending.point,
                pos: self.out.len(),
                base_space,
            });
        }
    }

    fn newline(&mut self, count: usize) {
        for _ in 0..count {
            self.out.push_str(self.eol);
        }
        for _ in 0..self.indent * INDENT_WIDTH {
            self.out.push(' ');
        }
    }

    //----------------------------------------------------------------------
    // Writing
    //----------------------------------------------------------------------

    /// Writes `text` after materialising any pending separation.
    ///
    /// The text is emitted exactly as given; nothing here inspects it, so a
    /// caller passing multi-line text owns its interior indentation.
    fn write_raw(&mut self, text: &str) {
        self.materialize();
        self.out.push_str(text);
    }

    /// Writes a significant token verbatim.
    ///
    /// Token text is always copied rather than reconstructed from the kind:
    /// several kinds have more than one spelling (`~^` and `^~` are both
    /// `XNOR`, `0xA5` and `0xa5` are both `HEX_NUMBER`), and which one the
    /// author wrote is not the formatter's business.
    pub(crate) fn token(&mut self, tok: &SyntaxToken) {
        debug_assert!(!tok.kind().is_trivia(), "trivia must go through trivia()");
        self.materialize();
        let start = self.out.len();
        for &row in &self.row_stack {
            self.rows[row].start.get_or_insert(start);
        }
        self.out.push_str(tok.text());
        let end = self.out.len();
        for &row in &self.row_stack {
            self.rows[row].end = Some(end);
        }
        self.saw_newline = false;
        self.after_comment = false;
    }

    /// Handles one trivia token: drops whitespace, keeps comments and
    /// preprocessor directives.
    pub(crate) fn trivia(&mut self, tok: &SyntaxToken) {
        match tok.kind() {
            SyntaxKind::WHITESPACE => {
                let newlines = tok.text().bytes().filter(|&b| b == b'\n').count();
                // Two newlines means one empty line between them. Anything
                // more says the same thing, which is how runs of blank lines
                // get capped.
                if newlines >= 2 {
                    self.blank_line();
                } else if newlines == 1 && self.after_comment {
                    // The one case where a plain source line break survives.
                    // Line breaks are otherwise the rules' decision -- honour
                    // them in general and nothing would ever be normalised --
                    // but a comment's trailing side has no rule to consult, and
                    // whether it ended the line is the author's to say.
                    self.request(Sep::Newline);
                }
                self.saw_newline |= newlines >= 1;
            }
            kind if kind.is_directive() => {
                // Requested rather than pinned, unlike a line comment: the
                // point here is to *raise* the separation to a break, never to
                // overrule a stronger one, and leaving the width open is what
                // lets a blank line the author left in front of an `include`
                // block survive.
                self.request(Sep::Newline);
                // A directive runs to the end of its line, so any spaces at
                // the end of it are outside the macro body in every sense that
                // matters -- and keeping them would leave the one thing this
                // formatter promises never to emit.
                self.write_raw(tok.text().trim_end());
                // Unconditional, for the same reason as a line comment's:
                // whatever follows a directive *must* start a new line, and
                // getting this wrong swallows code into a macro body.
                self.request(Sep::Newline);
                self.saw_newline = false;
                self.after_comment = false;
            }
            kind if kind.is_comment() => {
                // A comment that followed a newline in the source introduces
                // what comes after it and belongs on its own line. One that did
                // not is annotating the token it trails, and stays beside it.
                let inline = !self.saw_newline;
                if self.saw_newline {
                    self.request(Sep::Newline);
                } else if kind == SyntaxKind::LINE_COMMENT {
                    // Pinned rather than requested, because the enclosing rule
                    // has often already asked for a break: the parser hands a
                    // comment to the item that *follows* it, so `reg r { // why`
                    // reaches this point with the body's newline-before-each-
                    // item already pending, and a mere request would lose to it.
                    //
                    // Overriding is safe only for a line comment, which runs to
                    // the end of its line: nothing can follow it there, so
                    // pinning can never pull the next statement up beside it.
                    self.pin(Sep::Space);
                } else {
                    self.request(Sep::Space);
                }
                // A trailing comment belongs to the physical code line, even
                // when rowan handed its trivia to the following CST node.
                if inline {
                    self.attach_trailing_comment();
                }
                self.write_raw(tok.text());
                if kind == SyntaxKind::LINE_COMMENT {
                    // A line comment swallows the rest of its line, so anything
                    // after it *must* start a new one. Getting this wrong
                    // comments out code, which is why it is unconditional here
                    // rather than left to the rules.
                    self.request(Sep::Newline);
                } else {
                    // A block comment may legally be followed on the same line,
                    // so this is only a floor: it keeps `*/` from abutting the
                    // next token, and the whitespace arm above raises it to a
                    // newline if the author ended the line there.
                    self.request(Sep::Space);
                }
                self.saw_newline = false;
                self.after_comment = true;
            }
            kind => unreachable!("not trivia: {kind:?}"),
        }
    }

    /// Reproduces `node` exactly as it appears in the source.
    ///
    /// This began as the fallback for kinds without a rule yet, which is what
    /// made the formatter runnable and testable from its first commit. Every
    /// kind now has one except [`SyntaxKind::ERROR`], and an `ERROR` node exists
    /// only where the parser recorded an error, which [`crate::format`] refuses
    /// outright -- so nothing reaches this in a successful format.
    ///
    /// Kept because it is the right answer for the case it is left holding:
    /// input the parser could not understand should be handed back untouched
    /// rather than reshaped by rules that assume a structure it does not have.
    /// An error-tolerant mode would need exactly this.
    ///
    /// Trivia at either end is routed through [`Formatter::trivia`] rather than
    /// dumped with the rest: it belongs to the *surrounding* layout, not to the
    /// node. Leaving leading trivia in the span would emit the source's
    /// indentation alongside the indentation just generated, and leaving
    /// trailing trivia in it would preserve the column padding in front of a
    /// trailing comment, which is exactly the alignment the formatter exists to
    /// stop maintaining by hand.
    ///
    /// The *interior* does keep its original whitespace, so a construct spread
    /// over several lines keeps the indentation it was written with even when
    /// emitted at a different depth. Every construct that normally spans lines
    /// -- anything with a braced body -- has a real rule, so this is reachable
    /// only via a hand-wrapped statement, and it shrinks with each rule added.
    pub(crate) fn verbatim(&mut self, node: &SyntaxNode) {
        let src = self.src;
        let mut start = node.text_range().start();
        let mut end = node.text_range().end();

        for tok in leading_trivia(node) {
            self.trivia(&tok);
            start = tok.text_range().end();
        }
        // Bounded below by `start` so that a node which is *entirely* trivia
        // has it emitted once, by the loop above, rather than twice.
        let trailing = trailing_trivia(node, start);
        if let Some(first) = trailing.first() {
            end = first.text_range().start();
        }

        if start < end {
            self.write_raw(&src[usize::from(start)..usize::from(end)]);
            self.saw_newline = false;
            self.after_comment = false;
        }
        for tok in &trailing {
            self.trivia(tok);
        }
    }

    /// Adds the comment boundary to the most recent completed row when that
    /// row still occupies the current physical line.
    fn attach_trailing_comment(&mut self) {
        let line_start = self.out.rfind('\n').map_or(0, |pos| pos + 1);
        let Some((_, row)) = self
            .rows
            .iter_mut()
            .enumerate()
            .rev()
            .find(|(_, row)| row.end.is_some_and(|end| end >= line_start))
        else {
            return;
        };

        // `trivia` settles an inline comment to a space before writing it.
        row.markers.push(Marker {
            point: AlignPoint::TrailingComment,
            pos: self.out.len(),
            base_space: true,
        });
    }

    /// Computes padding from the completed rows and inserts it in one rebuild
    /// of the output. Layout is already final at this point.
    fn align(&mut self) {
        let mut insertions: BTreeMap<usize, usize> = BTreeMap::new();

        for scope in &self.scopes {
            let mut run: Vec<usize> = Vec::new();
            let mut previous: Option<usize> = None;

            for &row_id in &scope.rows {
                let row = &self.rows[row_id];
                let eligible = row.family != RowFamily::Other
                    && row.start.zip(row.end).is_some_and(|(start, end)| {
                        !self.out[start..end].contains('\n') && !row.markers.is_empty()
                    });

                let continues = eligible
                    && previous.is_some_and(|prev_id| {
                        let prev = &self.rows[prev_id];
                        prev.family == row.family && !self.breaks_run(prev, row)
                    });

                if !continues {
                    self.align_run(&run, &mut insertions);
                    run.clear();
                }
                if eligible {
                    run.push(row_id);
                    previous = Some(row_id);
                } else {
                    previous = None;
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

    fn breaks_run(&self, previous: &Row, current: &Row) -> bool {
        let (Some(end), Some(start)) = (previous.end, current.start) else {
            return true;
        };
        let between = &self.out[end..start];
        // Ignore the remainder of the previous code line and the indentation
        // before the current one. A comment-only line between them is
        // transparent; an actually empty interior line is a grouping boundary.
        let mut physical = between.split('\n');
        physical.next();
        let mut interior: Vec<&str> = physical.collect();
        interior.pop();
        interior.iter().any(|line| line.trim().is_empty())
            || crate::syntax::lex(between)
                .iter()
                .any(|(kind, _)| kind.is_directive())
    }

    fn align_run(&self, run: &[usize], insertions: &mut BTreeMap<usize, usize>) {
        if run.len() < 2 {
            return;
        }

        for point in [
            AlignPoint::InstType,
            AlignPoint::InstName,
            AlignPoint::InstReset,
            AlignPoint::InstAddress,
            AlignPoint::InstStride,
            AlignPoint::InstAlign,
            AlignPoint::ParamName,
            AlignPoint::ParamDefault,
            AlignPoint::EnumValue,
            AlignPoint::TrailingComment,
        ] {
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
            .map(|&(row_id, marker)| self.cell_width(&self.rows[row_id], marker))
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

    fn cell_width(&self, row: &Row, marker: Marker) -> usize {
        let start = row
            .markers
            .iter()
            .filter(|candidate| candidate.pos < marker.pos)
            .map(|candidate| candidate.pos)
            .max()
            .or(row.start)
            .unwrap_or(marker.pos);
        self.out[start..marker.pos]
            .trim_start_matches([' ', '\t'])
            .chars()
            .count()
    }
}

/// The run of trivia at the very start of `node`.
///
/// Leading trivia sits on the leftmost leaf, however deep that is -- the block
/// comment before `reg my_reg` lands three levels down, inside `COMPONENT_TYPE`
/// -- so this walks the token stream rather than the node's direct children.
fn leading_trivia(node: &SyntaxNode) -> impl Iterator<Item = SyntaxToken> {
    let end = node.text_range().end();
    std::iter::successors(node.first_token(), |tok: &SyntaxToken| tok.next_token())
        .take_while(move |tok| tok.text_range().end() <= end && tok.kind().is_trivia())
}

/// The run of trivia at the end of `node`, in source order.
///
/// `floor` bounds the search from below, so that trivia already emitted as
/// leading is not emitted a second time here.
fn trailing_trivia(node: &SyntaxNode, floor: TextSize) -> Vec<SyntaxToken> {
    let mut out: Vec<SyntaxToken> =
        std::iter::successors(node.last_token(), |tok: &SyntaxToken| tok.prev_token())
            .take_while(|tok| tok.text_range().start() >= floor && tok.kind().is_trivia())
            .collect();
    out.reverse();
    out
}

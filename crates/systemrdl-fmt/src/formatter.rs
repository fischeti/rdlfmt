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

use rowan::TextSize;
use systemrdl_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

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
        }
    }

    /// Finishes the file: exactly one trailing newline, or nothing at all if
    /// there was no content.
    pub(crate) fn finish(mut self) -> String {
        let trimmed = self.out.trim_end().len();
        self.out.truncate(trimmed);
        if !self.out.is_empty() {
            self.out.push('\n');
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
            Sep::None => {}
            Sep::Space => self.out.push(' '),
            Sep::Newline => self.newline(if gap.width == Width::Blank { 2 } else { 1 }),
        }
    }

    fn newline(&mut self, count: usize) {
        for _ in 0..count {
            self.out.push('\n');
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
        self.write_raw(tok.text());
        self.saw_newline = false;
        self.after_comment = false;
    }

    /// Handles one trivia token: drops whitespace, keeps comments.
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
            kind if kind.is_comment() => {
                // A comment that followed a newline in the source introduces
                // what comes after it and belongs on its own line. One that did
                // not is annotating the token it trails, and stays beside it.
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

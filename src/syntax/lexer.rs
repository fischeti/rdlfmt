//! Turning source text into a flat token stream.
//!
//! Nothing is discarded: the tokens' text concatenates back to the input byte
//! for byte, including bytes that failed to lex.

use crate::syntax::kind::SyntaxKind;
use logos::Logos;
use rowan::{TextRange, TextSize};

/// A single token: its kind and byte range. Its text is `&src[range]`, which
/// [`Lexed`] provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexedToken {
    pub kind: SyntaxKind,
    pub range: TextRange,
}

/// A token stream together with the source it was lexed from, so token text
/// always comes from the right string. Text is borrowed from the source, not
/// from `self`.
#[derive(Debug, Clone)]
pub struct Lexed<'a> {
    src: &'a str,
    tokens: Vec<LexedToken>,
}

impl<'a> Lexed<'a> {
    /// The source text these tokens index into.
    pub fn src(&self) -> &'a str {
        self.src
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// The kind of token `i`, or [`SyntaxKind::EOF`] past the end, which spares
    /// the parser a bounds check on every lookahead.
    pub fn kind(&self, i: usize) -> SyntaxKind {
        self.tokens.get(i).map_or(SyntaxKind::EOF, |t| t.kind)
    }

    /// The exact source text of token `i`, or `""` past the end.
    pub fn text(&self, i: usize) -> &'a str {
        match self.tokens.get(i) {
            Some(t) => &self.src[t.range],
            None => "",
        }
    }

    /// The byte range of token `i`, or an empty range at [`Lexed::end`] past
    /// the end.
    pub fn range(&self, i: usize) -> TextRange {
        self.tokens
            .get(i)
            .map_or_else(|| TextRange::empty(self.end()), |t| t.range)
    }

    /// The offset just past the last token, which by the no-gaps invariant is
    /// the end of the source.
    pub fn end(&self) -> TextSize {
        self.tokens
            .last()
            .map_or_else(|| TextSize::new(0), |t| t.range.end())
    }

    /// Every token as a `(kind, text)` pair, trivia included.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (SyntaxKind, &'a str)> + '_ {
        let src = self.src;
        self.tokens.iter().map(move |t| (t.kind, &src[t.range]))
    }
}

/// Lexes `src` into a complete, gap-free token stream.
///
/// # Panics
/// If `src` is larger than 4 GiB, which rowan cannot represent either.
pub fn lex(src: &str) -> Lexed<'_> {
    let src_len = u32::try_from(src.len()).expect("source larger than 4 GiB");

    // A guess at token density, to save a few reallocations.
    let mut out: Vec<LexedToken> = Vec::with_capacity(src.len() / 4);
    let mut lexer = SyntaxKind::lexer(src);

    while let Some(result) = lexer.next() {
        // Both ends are <= src.len(), checked above, so the casts cannot wrap.
        let span = lexer.span();
        let range = TextRange::new(
            TextSize::from(span.start as u32),
            TextSize::from(span.end as u32),
        );
        debug_assert!(u32::from(range.end()) <= src_len);

        let kind = match result {
            Ok(kind) => kind,
            Err(()) => {
                // Fold a run of unrecognised bytes into one token, so it is
                // reported as one error.
                if let Some(last) = out.last_mut()
                    && last.kind == SyntaxKind::LEX_ERROR
                    && last.range.end() == range.start()
                {
                    last.range = TextRange::new(last.range.start(), range.end());
                    continue;
                }
                SyntaxKind::LEX_ERROR
            }
        };

        out.push(LexedToken { kind, range });
    }

    Lexed { src, tokens: out }
}

//! Turning source text into a flat token stream.
//!
//! The one property that matters here: **nothing is discarded**. Concatenating
//! the text of every token this module produces reproduces the input byte for
//! byte, including whitespace, comments, and bytes that failed to lex. That is
//! what makes the lossless syntax tree possible downstream, and it is enforced
//! by the round-trip tests.

use crate::syntax::kind::SyntaxKind;
use logos::Logos;
use rowan::{TextRange, TextSize};

/// A single token: what it is and where it came from.
///
/// Deliberately small and `Copy` -- one of these exists per token in the file,
/// so the whole stream is a flat array of 12-byte records. The text is *not*
/// stored: it is exactly `&src[range]`, so keeping it would be a redundant
/// 16 bytes and a second copy of an invariant that could drift.
///
/// Offsets are rowan's [`TextSize`] (a `u32`) rather than `usize`, both to
/// halve their size and because that is the type rowan itself uses -- storing
/// `usize` here would only mean converting at every tree and diagnostic
/// boundary. They are also what diagnostics need, so a `&str` here would not
/// remove the need for offsets, only make them harder to recover.
///
/// A bare `LexedToken` cannot yield its text; that requires the source it came
/// from, so text lives on [`Lexed`], which owns both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexedToken {
    pub kind: SyntaxKind,
    pub range: TextRange,
}

/// A token stream together with the source it was lexed from.
///
/// Pairing them once is what makes the "same string" precondition of a
/// `token.text(src)` free function unrepresentable: every accessor here is
/// indexed by token position, and the text it returns is by construction a
/// slice of the right string.
///
/// Note that [`Lexed::text`] returns `&'a str`, borrowed from the source
/// rather than from `self`. Callers can therefore hold token text across
/// mutations of whatever else they own -- which is exactly what the parser
/// needs when feeding a token into a tree builder it holds mutably.
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

    /// The kind of token `i`, or [`SyntaxKind::EOF`] past the end.
    ///
    /// Treating the end of input as a kind rather than as `None` removes a
    /// bounds check from every lookahead in the parser. It cannot be mistaken
    /// for a real token: [`lex`] never produces `EOF`.
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

    // A guess, not a bound: measured token density ranges from ~1 byte/token
    // for dense punctuation to ~22 for comment-heavy input, so no divisor is
    // right for every file. This one just trims a few reallocs on typical
    // input; being wrong either way costs nothing but a regrow.
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
                // Unrecognised bytes. logos reports these one chunk at a time;
                // fold a run of them into a single token so that a stretch of
                // garbage surfaces as one error rather than a dozen.
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

//! rowan wiring: the `Language` impl and the concrete tree types.
//!
//! rowan is generic over a `Language` so that one tree implementation can serve
//! any grammar. All the trait does is convert between our [`SyntaxKind`] and the
//! opaque `u16` rowan stores internally, which is why the aliases below are the
//! only things the rest of the crate ever names.

use crate::kind::SyntaxKind;

/// Marker type identifying SystemRDL to rowan. Never instantiated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemRdl {}

impl rowan::Language for SystemRdl {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from_raw(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_raw())
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        rowan::SyntaxKind(kind.to_raw())
    }
}

/// An interior node: has children, has a kind, has no text of its own.
pub type SyntaxNode = rowan::api::SyntaxNode<SystemRdl>;
/// A leaf: carries the exact source text it was lexed from.
pub type SyntaxToken = rowan::api::SyntaxToken<SystemRdl>;
/// Either of the above. Iterating `children_with_tokens()` yields these.
pub type SyntaxElement = rowan::api::SyntaxElement<SystemRdl>;
pub type SyntaxNodeChildren = rowan::api::SyntaxNodeChildren<SystemRdl>;
pub type SyntaxElementChildren = rowan::api::SyntaxElementChildren<SystemRdl>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<SystemRdl>;

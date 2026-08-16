//! A lossless syntax tree for SystemRDL.
//!
//! The pipeline is:
//!
//! ```text
//! source text
//!     |
//!     v  lexer (logos)      flat token stream, nothing discarded
//!     v  parser             hand-written recursive descent
//!     v  rowan              lossless CST: tree.to_string() == source
//! ```
//!
//! The defining property is that last one. Unlike an abstract syntax tree,
//! which throws away whitespace and comments because they carry no meaning,
//! every byte of the input is present in this tree as a token. A formatter
//! needs that: it has to decide where to put a comment, not delete it.

pub mod kind;
pub mod lexer;
pub mod parser;
pub mod tree;

pub use kind::SyntaxKind;
pub use lexer::{Lexed, LexedToken, lex};
pub use parser::{ParseError, Parsed, parse};
pub use tree::{SyntaxElement, SyntaxNode, SyntaxToken, SystemRdl};

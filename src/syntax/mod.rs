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
//! Unlike an abstract syntax tree, it keeps every byte of the input,
//! whitespace and comments included, because a formatter has to place them.

pub mod kind;
pub mod lexer;
pub mod parser;
pub mod tree;

pub use kind::SyntaxKind;
pub use lexer::{Lexed, LexedToken, lex};
pub use parser::{ParseError, Parsed, parse};
pub use tree::{SyntaxElement, SyntaxNode, SyntaxToken, SystemRdl};

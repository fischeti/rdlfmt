//! ANTLR-generated code. Do not edit by hand -- regenerate with `grammar/generate.sh`.
//!
//! The generated files refer to each other via `use super::systemrdlparser::*;`,
//! so they have to live as siblings inside this one module.
//!
//! ANTLR also emits `systemrdlbaselistener.rs` / `systemrdlbasevisitor.rs`, which
//! `generate.sh` deletes -- see the note there. They are the Java target's
//! "override only what you need" base classes, which Rust does not need: the
//! `SystemRDLListener` / `SystemRDLVisitor` traits below already give every hook
//! an empty default body. The base traits are also unusable, since the parser
//! only implements `Listenable` for `dyn SystemRDLListener`.

pub mod systemrdllexer;
pub mod systemrdllistener;
pub mod systemrdlparser;
pub mod systemrdlvisitor;

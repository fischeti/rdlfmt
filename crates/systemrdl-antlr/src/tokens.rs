//! Small helpers for turning raw ANTLR token data into readable text.

use antlr4rust::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, TOKEN_HIDDEN_CHANNEL};

use crate::parser::systemrdllexer::{_LITERAL_NAMES, _SYMBOLIC_NAMES};

// A `skip_leading_off_channel` helper used to live here, to work around
// antlr-rust 0.3.0-beta building a `CommonTokenStream` positioned on a hidden
// token -- so any file opening with a comment failed to parse. antlr4rust 0.5
// does that skip inside `CommonTokenStream::with_channel`, so the helper is gone.

/// Map a numeric token type back to a human-readable name.
///
/// ANTLR generates two parallel tables. Tokens declared as named lexer rules
/// (`ID`, `STRING`, `REG_kw`) have an entry in `_SYMBOLIC_NAMES`; tokens that
/// came from inline literals in parser rules (`';'`, `'{'`) only have an entry
/// in `_LITERAL_NAMES` and are named `T__0`, `T__1`, ... internally.
pub fn token_name(token_type: i32) -> String {
    if token_type == TOKEN_EOF {
        return "EOF".to_string();
    }
    let idx = match usize::try_from(token_type) {
        Ok(i) => i,
        Err(_) => return format!("<{token_type}>"),
    };
    if let Some(Some(name)) = _SYMBOLIC_NAMES.get(idx) {
        return (*name).to_string();
    }
    if let Some(Some(literal)) = _LITERAL_NAMES.get(idx) {
        return (*literal).to_string();
    }
    format!("<{token_type}>")
}

pub fn channel_name(channel: i32) -> &'static str {
    match channel {
        TOKEN_DEFAULT_CHANNEL => "default",
        TOKEN_HIDDEN_CHANNEL => "HIDDEN",
        _ => "other",
    }
}

/// Render token text on a single line so tables stay aligned.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for ch in text.chars() {
        match ch {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

pub fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let head: String = text.chars().take(max - 1).collect();
    format!("{head}\u{2026}")
}

//! Dump every token the lexer produces for a SystemRDL file.
//!
//! This is stage 1 of the pipeline only: characters -> tokens. No parse tree.
//!
//! The tokens are pulled through a `CommonTokenStream` rather than straight off
//! the lexer, for one reason: the stream is what assigns each token its *index*.
//! That index is the handle a formatter uses to ask "what whitespace and
//! comments sat to the left of this token?", so it is worth seeing here.
//!
//! Note the stream keeps hidden-channel tokens in its buffer -- it only hides
//! them from the *parser*. `get(i)` walks the raw buffer and sees everything.
//!
//!     cargo run --bin dump-tokens -- samples/sample.rdl

use std::process::ExitCode;

use antlr4rust::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::int_stream::IntStream;
use antlr4rust::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, TOKEN_HIDDEN_CHANNEL, Token};
use antlr4rust::token_stream::TokenStream;

use systemrdl_antlr::parser::systemrdllexer::SystemRDLLexer;
use systemrdl_antlr::tokens::{channel_name, escape, token_name, truncate};

fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "samples/sample.rdl".to_string());

    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let lexer = SystemRDLLexer::new(InputStream::new(&*source));
    let mut stream = CommonTokenStream::new(lexer);

    // Drain the stream so every token is lexed and buffered. `consume` walks
    // only default-channel tokens, but the hidden ones it steps over are still
    // appended to the buffer on the way.
    while stream.la(1) != TOKEN_EOF {
        stream.consume();
    }

    println!("{path}");
    println!(
        "{:>4}  {:<18} {:<8} {:>4}:{:<3} {:>10}  {}",
        "IDX", "TYPE", "CHANNEL", "LN", "COL", "SPAN", "TEXT"
    );
    println!("{}", "-".repeat(80));

    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut hidden = 0usize;
    let mut comments: Vec<(isize, String)> = Vec::new();

    let total = stream.size();
    for i in 0..total {
        let token = stream.get(i);
        let token_type = token.get_token_type();
        let name = token_name(token_type);

        let text = if token_type == TOKEN_EOF {
            "<end of file>".to_string()
        } else {
            truncate(&escape(token.get_text()), 34)
        };

        println!(
            "{:>4}  {:<18} {:<8} {:>4}:{:<3} {:>4}..{:<4} {}",
            token.get_token_index(),
            name,
            channel_name(token.get_channel()),
            token.get_line(),
            token.get_column(),
            token.get_start(),
            token.get_stop(),
            text,
        );

        if token_type == TOKEN_EOF {
            continue;
        }
        if token.get_channel() == TOKEN_HIDDEN_CHANNEL {
            hidden += 1;
        }
        if name == "SL_COMMENT" || name == "ML_COMMENT" {
            comments.push((i, truncate(&escape(token.get_text()), 30)));
        }
        *counts.entry(name).or_default() += 1;
    }

    let counted = (total as usize) - 1; // exclude EOF
    println!("{}", "-".repeat(80));
    println!(
        "{counted} tokens: {} on the default channel, {hidden} hidden \
         (whitespace + comments the parser never sees)",
        counted - hidden
    );

    let mut by_freq: Vec<_> = counts.into_iter().collect();
    by_freq.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top: Vec<String> = by_freq
        .iter()
        .take(6)
        .map(|(name, n)| format!("{name} x{n}"))
        .collect();
    println!("most frequent: {}", top.join(", "));

    // The formatter's core question, previewed: each comment survived lexing,
    // and the next default-channel token tells you what it belongs to.
    println!("\ncomments, and the token each one precedes:");
    for (idx, text) in &comments {
        let mut j = idx + 1;
        while j < total && stream.get(j).get_channel() != TOKEN_DEFAULT_CHANNEL {
            j += 1;
        }
        let next = stream.get(j);
        println!(
            "  [{idx:>3}] {:<32} -> [{:>3}] {} {:?}",
            text,
            j,
            token_name(next.get_token_type()),
            escape(next.get_text()),
        );
    }

    ExitCode::SUCCESS
}

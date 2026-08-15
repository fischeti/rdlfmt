//! Prints the lossless syntax tree for a SystemRDL file.
//!
//! The counterpart to the ANTLR crate's `dump-tree`, with one visible
//! difference: whitespace and comments appear as tokens, because they are in
//! the tree rather than on a hidden channel.
//!
//!     cargo run -p systemrdl-syntax --bin dump-cst -- samples/sample.rdl
//!     cargo run -p systemrdl-syntax --bin dump-cst -- samples/sample.rdl --no-trivia

use systemrdl_syntax::{SyntaxElement, SyntaxNode, parse};

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let hide_trivia = args.iter().any(|a| a == "--no-trivia");
    let path = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "samples/sample.rdl".to_string());

    let src = match std::fs::read_to_string(&path) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("{path}: {err}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let parsed = parse(&src);

    print_node(&parsed.syntax(), 0, hide_trivia);

    // The property the whole design rests on.
    let round_trips = parsed.syntax().to_string() == src;
    println!();
    println!("round-trips: {round_trips}");

    if !parsed.errors().is_empty() {
        println!("\n{} error(s):", parsed.errors().len());
        for err in parsed.errors() {
            let line = 1 + src[..err.range.start].matches('\n').count();
            println!("  {path}:{line}: {}", err.message);
        }
        return std::process::ExitCode::FAILURE;
    }

    if round_trips {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}

fn print_node(node: &SyntaxNode, indent: usize, hide_trivia: bool) {
    let range = node.text_range();
    println!(
        "{:indent$}{:?}@{}..{}",
        "",
        node.kind(),
        u32::from(range.start()),
        u32::from(range.end()),
        indent = indent
    );
    for child in node.children_with_tokens() {
        match child {
            SyntaxElement::Node(n) => print_node(&n, indent + 2, hide_trivia),
            SyntaxElement::Token(t) => {
                if hide_trivia && t.kind().is_trivia() {
                    continue;
                }
                let range = t.text_range();
                println!(
                    "{:indent$}{:?}@{}..{} {:?}",
                    "",
                    t.kind(),
                    u32::from(range.start()),
                    u32::from(range.end()),
                    t.text(),
                    indent = indent + 2
                );
            }
        }
    }
}

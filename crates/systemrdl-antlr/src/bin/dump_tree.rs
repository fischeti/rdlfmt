//! Parse a SystemRDL file and pretty-print the parse tree.
//!
//! This is stage 2: tokens -> tree. Where `dump-tokens` showed a flat list,
//! this shows the nesting the parser discovered.
//!
//! The walk is driven by a *listener*: `ParseTreeWalker` owns the recursion and
//! calls `enter_every_rule` / `exit_every_rule` / `visit_terminal` as it passes
//! each node. Indentation is just a counter bumped on enter and dropped on
//! exit -- which is the whole listener model in three lines.
//!
//! Terminal nodes print their token index in `[..]`. Cross-reference those with
//! `dump-tokens` output and you will see the indices skip: the gaps are the
//! hidden-channel whitespace and comments, which are lexed and buffered but
//! never attached to the tree.
//!
//!     cargo run --bin dump-tree -- samples/sample.rdl
//!     cargo run --bin dump-tree -- samples/sample.rdl --terse

use std::cell::Cell;
use std::process::ExitCode;
use std::rc::Rc;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::{TOKEN_EOF, Token};
use antlr4rust::token_factory::TokenFactory;
use antlr4rust::tree::{ErrorNode, ParseTreeListener, TerminalNode};
use antlr4rust::{InputStream, Parser};

use systemrdl_antlr::parser::systemrdllexer::SystemRDLLexer;
use systemrdl_antlr::parser::systemrdllistener::SystemRDLListener;
use systemrdl_antlr::parser::systemrdlparser::{
    SystemRDLParser, SystemRDLParserContext, SystemRDLParserContextType, SystemRDLTreeWalker,
    ruleNames,
};
use systemrdl_antlr::tokens::{escape, token_name, truncate};

/// Collects syntax errors instead of letting them scroll past on stderr.
///
/// `syntax_error` takes `&self`, so the counter needs interior mutability.
struct ErrorCollector {
    count: Rc<Cell<usize>>,
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for ErrorCollector {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.count.set(self.count.get() + 1);
        eprintln!("syntax error at {line}:{column}: {msg}");
    }
}

/// Prints the tree as it is walked.
///
/// In `--terse` mode, chains of rule nodes that have exactly one child are
/// collapsed onto a single line. A parse tree is a *concrete* syntax tree, so
/// every layer of the grammar shows up even when it adds no structure --
/// `expr > expr_primary > literal > number > INT` is five nodes describing one
/// integer. Collapsing them makes the shape that actually matters visible.
struct TreeDumper {
    depth: usize,
    terse: bool,
    rules: usize,
    terminals: usize,
    /// Rule names entered but not yet printed, waiting to be flushed as a chain.
    pending: Vec<String>,
    /// Per-level record of whether that level printed a line and took an indent,
    /// so `exit_every_rule` can undo exactly what `enter_every_rule` did.
    indented: Vec<bool>,
}

impl TreeDumper {
    fn new(terse: bool) -> Self {
        Self {
            depth: 0,
            terse,
            rules: 0,
            terminals: 0,
            pending: Vec::new(),
            indented: Vec::new(),
        }
    }

    fn indent(&self) -> String {
        // Box-drawing guides make deep nesting much easier to follow.
        "\u{2502}   ".repeat(self.depth)
    }

    /// Consume any deferred rule names as a `a > b > c > ` line prefix.
    fn take_prefix(&mut self) -> String {
        if self.pending.is_empty() {
            return String::new();
        }
        let chain = self.pending.join(" > ");
        self.pending.clear();
        format!("{chain} > ")
    }
}

impl<'input> ParseTreeListener<'input, SystemRDLParserContextType> for TreeDumper {
    fn visit_terminal(&mut self, node: &TerminalNode<'input, SystemRDLParserContextType>) {
        self.terminals += 1;
        let token = &node.symbol;
        let token_type = token.get_token_type();
        if self.terse && token_type == TOKEN_EOF {
            return;
        }
        let text = if token_type == TOKEN_EOF {
            "<end of file>".to_string()
        } else {
            format!("{:?}", truncate(&escape(token.get_text()), 28))
        };
        let indent = self.indent();
        let prefix = self.take_prefix();
        println!(
            "{indent}{prefix}[{:>3}] {} {text}",
            token.get_token_index(),
            token_name(token_type),
        );
    }

    fn visit_error_node(&mut self, node: &ErrorNode<'input, SystemRDLParserContextType>) {
        let indent = self.indent();
        let prefix = self.take_prefix();
        println!(
            "{indent}{prefix}<error> {:?}",
            escape(node.symbol.get_text())
        );
    }

    fn enter_every_rule(
        &mut self,
        ctx: &(dyn SystemRDLParserContext<'input> + 'input),
    ) -> Result<(), ANTLRError> {
        self.rules += 1;
        // `get_rule_index` indexes into the generated `ruleNames` table, which
        // holds the lowercase parser rules in the order they appear in the .g4.
        let name = ruleNames
            .get(ctx.get_rule_index())
            .copied()
            .unwrap_or("<unknown rule>");

        // A rule with exactly one child adds nesting but no branching, so in
        // terse mode defer it and fold it into whatever line comes next.
        if self.terse && ctx.get_child_count() == 1 {
            self.pending.push(name.to_string());
            self.indented.push(false);
            return Ok(());
        }

        let indent = self.indent();
        let prefix = self.take_prefix();
        println!("{indent}{prefix}{name}");
        self.indented.push(true);
        self.depth += 1;
        Ok(())
    }

    fn exit_every_rule(
        &mut self,
        _ctx: &(dyn SystemRDLParserContext<'input> + 'input),
    ) -> Result<(), ANTLRError> {
        if self.indented.pop().unwrap_or(true) {
            self.depth -= 1;
        }
        Ok(())
    }
}

// The walker requires the listener to be accepted by the generated
// `Listenable` impls. Every method has a default, so an empty impl is enough --
// we only care about the generic `enter_every_rule` / `visit_terminal` hooks.
impl<'input> SystemRDLListener<'input> for TreeDumper {}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let terse = args.iter().any(|a| a == "--terse");
    let path = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "samples/sample.rdl".to_string());

    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let errors = Rc::new(Cell::new(0usize));

    let lexer = SystemRDLLexer::new(InputStream::new(&*source));
    let stream = CommonTokenStream::new(lexer);

    let mut parser = SystemRDLParser::new(stream);
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(ErrorCollector {
        count: Rc::clone(&errors),
    }));

    // `root` is the grammar's start rule: `root: (root_elem ';')* EOF;`
    let tree = match parser.root() {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("error: parse failed: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    println!("{path}");
    println!("{}", "-".repeat(80));

    // `walk` is fallible now: listener callbacks return `Result`, so a listener
    // can abort the walk. Ours never does, but the error still has to be handled.
    let dumper = match SystemRDLTreeWalker::walk(Box::new(TreeDumper::new(terse)), &*tree) {
        Ok(dumper) => dumper,
        Err(e) => {
            eprintln!("error: tree walk failed: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    println!("{}", "-".repeat(80));
    println!(
        "{} rule nodes, {} terminals",
        dumper.rules, dumper.terminals
    );

    let error_count = errors.get();
    if error_count > 0 {
        eprintln!("{error_count} syntax error(s); tree above is the error-recovered shape");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

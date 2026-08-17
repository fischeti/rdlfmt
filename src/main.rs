//! The `rdlfmt` command.
//!
//! # Writing is the default
//!
//! `rdlfmt foo.rdl` rewrites `foo.rdl`. There is no `--write` flag,
//! because the overwhelmingly common thing to want is formatted files, and a
//! flag that is passed every single time is not carrying information -- the
//! same call rustfmt, black and ruff make. `--check`, `--diff` and `--stdout`
//! are there for the times you want something else.
//!
//! What makes that defensible is not this file: [`rdlfmt::format`]
//! verifies its own output before returning it, so a file is only ever replaced
//! by one that lexes to the same code.

mod diff;

use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Cargo's palette, which is what a Rust toolchain user's eye is already
/// calibrated to. `clap` turns colour off on its own when stdout is not a
/// terminal or `NO_COLOR` is set, so this needs no condition around it.
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default())
    .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD));

#[derive(Debug, Parser)]
#[command(
    name = "rdlfmt",
    version,
    about = "Format SystemRDL source",
    long_about = "Format SystemRDL source.\n\n\
                  Rewrites each PATH in place. A directory is searched for `.rdl` \
                  files, skipping anything `.gitignore` or a leading dot excludes. \
                  With no PATH, reads stdin and writes stdout.",
    styles = STYLES,
)]
struct Cli {
    /// Files or directories to format; omit to read stdin
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,

    /// Write nothing; exit 1 if any file is not formatted
    #[arg(short, long)]
    check: bool,

    /// Like --check, but show what would change
    #[arg(short, long, conflicts_with = "check")]
    diff: bool,

    /// Write to stdout instead of rewriting the file
    #[arg(long, conflicts_with_all = ["check", "diff"])]
    stdout: bool,
}

/// What to do with the formatted text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Write,
    Check,
    /// `Check` that says what it found.
    Diff,
    Stdout,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut run = Run {
        mode: if cli.check {
            Mode::Check
        } else if cli.diff {
            Mode::Diff
        } else if cli.stdout {
            Mode::Stdout
        } else {
            Mode::Write
        },
        palette: diff::Palette::for_stream(&std::io::stdout()),
        needs_formatting: 0,
        formatted: 0,
        failed: 0,
    };

    // A lone `-` is the conventional spelling of stdin, and so means the same
    // as passing no path at all.
    let paths: Vec<&PathBuf> = cli
        .paths
        .iter()
        .filter(|path| path.as_os_str() != "-")
        .collect();

    if paths.is_empty() {
        run.stdin();
    } else {
        for path in paths {
            run.path(path);
        }
    }

    run.finish()
}

struct Run {
    mode: Mode,
    palette: diff::Palette,
    /// Files that are not formatted. Only counted under `--check` and `--diff`.
    needs_formatting: usize,
    formatted: usize,
    failed: usize,
}

impl Run {
    fn stdin(&mut self) {
        let mut src = String::new();
        if let Err(err) = std::io::stdin().read_to_string(&mut src) {
            eprintln!("error: reading stdin: {err}");
            self.failed += 1;
            return;
        }

        let Some(out) = self.format(&src, Path::new("<stdin>")) else {
            return;
        };

        match self.mode {
            // Nothing to rewrite, so writing out is all `Write` can mean here.
            Mode::Write | Mode::Stdout => {
                if let Err(err) = std::io::stdout().write_all(out.as_bytes()) {
                    eprintln!("error: writing stdout: {err}");
                    self.failed += 1;
                }
            }
            Mode::Check | Mode::Diff => self.report(Path::new("<stdin>"), &src, &out),
        }
    }

    /// Notes that `path` is not formatted, in whatever detail the mode asks
    /// for. Silent when there is nothing to say, so that a clean run of
    /// `--check` or `--diff` prints nothing at all.
    fn report(&mut self, path: &Path, src: &str, out: &str) {
        if out == src {
            return;
        }
        self.needs_formatting += 1;

        if self.mode == Mode::Diff {
            let mut stdout = std::io::stdout().lock();
            if let Err(err) = diff::write(&mut stdout, path, src, out, &self.palette) {
                eprintln!("error: writing stdout: {err}");
                self.failed += 1;
            }
        } else {
            println!("{} is not formatted", path.display());
        }
    }

    fn path(&mut self, path: &Path) {
        if path.is_dir() {
            for file in self.rdl_files(path) {
                self.file(&file);
            }
        } else {
            self.file(path);
        }
    }

    /// Every `.rdl` file under `dir`, sorted so that output is reproducible.
    ///
    /// What the walk descends into is what `git` would consider part of the
    /// tree: `.gitignore` and `.ignore` are honoured, and entries whose name
    /// starts with a `.` are skipped. Between them that keeps `rdlfmt .` out
    /// of `.git`, `target/` and `build/` without a flag and without `rdlfmt`
    /// needing to know what any of those are.
    ///
    /// Ignore rules only prune what a *walk discovers*. Naming a path
    /// outright -- `rdlfmt build/regs.rdl`, or `rdlfmt build/` -- formats it
    /// either way, because asking for something by name is a clearer
    /// statement of intent than a pattern written for some other tool.
    ///
    /// A walk error is reported against the entry it happened on and the walk
    /// carries on, so one unreadable directory does not cost you the rest of
    /// the tree.
    fn rdl_files(&mut self, dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();

        let walk = ignore::WalkBuilder::new(dir)
            // Off by default outside a git repository, which would make an
            // exported or vendored tree behave differently from the one it
            // came from. The `.gitignore` is the statement of intent; whether
            // `.git` happens to still be next to it is not.
            .require_git(false)
            .build();

        for entry in walk {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("error: {err}");
                    self.failed += 1;
                    continue;
                }
            };
            // `None` only for stdin, which cannot turn up in a walk.
            if !entry.file_type().is_some_and(|ty| ty.is_file()) {
                continue;
            }
            if entry.path().extension().is_some_and(|ext| ext == "rdl") {
                out.push(entry.into_path());
            }
        }

        out.sort();
        out
    }

    fn file(&mut self, path: &Path) {
        let src = match std::fs::read_to_string(path) {
            Ok(src) => src,
            Err(err) => {
                eprintln!("error: {}: {err}", path.display());
                self.failed += 1;
                return;
            }
        };

        let Some(out) = self.format(&src, path) else {
            return;
        };

        match self.mode {
            Mode::Stdout => {
                if let Err(err) = std::io::stdout().write_all(out.as_bytes()) {
                    eprintln!("error: writing stdout: {err}");
                    self.failed += 1;
                }
            }
            Mode::Check | Mode::Diff => self.report(path, &src, &out),
            Mode::Write => {
                // An unchanged file is left alone rather than rewritten with
                // identical bytes, so that formatting a tree does not touch
                // every mtime and set every rebuild going.
                if out == src {
                    return;
                }
                if let Err(err) = std::fs::write(path, &out) {
                    eprintln!("error: {}: {err}", path.display());
                    self.failed += 1;
                    return;
                }
                println!("{}", path.display());
                self.formatted += 1;
            }
        }
    }

    /// Formats `src`, reporting any error against `path`.
    fn format(&mut self, src: &str, path: &Path) -> Option<String> {
        match rdlfmt::format(src) {
            Ok(out) => Some(out),
            Err(err) => {
                for error in err.errors() {
                    let (line, col) = line_col(src, error.range.start);
                    eprintln!("{}:{line}:{col}: {}", path.display(), error.message);
                }
                if err.errors().is_empty() {
                    eprintln!("{}: {err}", path.display());
                }
                self.failed += 1;
                None
            }
        }
    }

    fn finish(self) -> ExitCode {
        if self.failed > 0 {
            eprintln!(
                "{} file{} could not be formatted",
                self.failed,
                plural(self.failed)
            );
            return ExitCode::from(2);
        }
        if self.needs_formatting > 0 {
            eprintln!(
                "{} file{} need{} formatting",
                self.needs_formatting,
                plural(self.needs_formatting),
                if self.needs_formatting == 1 { "s" } else { "" }
            );
            return ExitCode::from(1);
        }
        if self.formatted > 0 {
            // On stderr, so that the paths on stdout stay pipeable.
            eprintln!(
                "{} file{} formatted",
                self.formatted,
                plural(self.formatted)
            );
        }
        ExitCode::SUCCESS
    }
}

fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}

/// Byte offset to 1-based line and column, for diagnostics.
fn line_col(src: &str, offset: usize) -> (usize, usize) {
    let upto = &src[..offset.min(src.len())];
    let line = upto.bytes().filter(|&b| b == b'\n').count() + 1;
    let col = upto
        .rsplit('\n')
        .next()
        .map_or(0, |line| line.chars().count())
        + 1;
    (line, col)
}

//! The `systemrdl-fmt` command.
//!
//! # Writing is the default
//!
//! `systemrdl-fmt foo.rdl` rewrites `foo.rdl`. There is no `--write` flag,
//! because the overwhelmingly common thing to want is formatted files, and a
//! flag that is passed every single time is not carrying information -- the
//! same call rustfmt, black and ruff make. `--check` and `--stdout` are there
//! for the times you want something else.
//!
//! What makes that defensible is not this file: [`systemrdl_fmt::format_with`]
//! verifies its own output before returning it, so a file is only ever replaced
//! by one that lexes to the same code.
//!
use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use systemrdl_fmt::{FormatOptions, format_with};

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
    name = "systemrdl-fmt",
    version,
    about = "Format SystemRDL source",
    long_about = "Format SystemRDL source.\n\n\
                  Rewrites each PATH in place. A directory is searched for `.rdl` \
                  files. With no PATH, reads stdin and writes stdout.",
    styles = STYLES,
)]
struct Cli {
    /// Files or directories to format; omit to read stdin
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,

    /// Write nothing; exit 1 if any file is not formatted
    #[arg(short, long)]
    check: bool,

    /// Write to stdout instead of rewriting the file
    #[arg(long, conflicts_with = "check")]
    stdout: bool,

    /// Spaces per indentation level
    #[arg(long, value_name = "N", default_value_t = 4)]
    indent: usize,
}

/// What to do with the formatted text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Write,
    Check,
    Stdout,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut run = Run {
        mode: if cli.check {
            Mode::Check
        } else if cli.stdout {
            Mode::Stdout
        } else {
            Mode::Write
        },
        options: FormatOptions {
            indent_width: cli.indent,
        },
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
    options: FormatOptions,
    /// Files that are not formatted. Only counted under `--check`.
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
            Mode::Check => {
                if out != src {
                    println!("<stdin> is not formatted");
                    self.needs_formatting += 1;
                }
            }
        }
    }

    fn path(&mut self, path: &Path) {
        if path.is_dir() {
            match rdl_files(path) {
                Ok(files) => {
                    for file in files {
                        self.file(&file);
                    }
                }
                Err(err) => {
                    eprintln!("error: {}: {err}", path.display());
                    self.failed += 1;
                }
            }
        } else {
            self.file(path);
        }
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
            Mode::Check => {
                if out != src {
                    println!("{} is not formatted", path.display());
                    self.needs_formatting += 1;
                }
            }
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
        match format_with(src, &self.options) {
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

/// Every `.rdl` file under `dir`, sorted so that output is reproducible.
///
/// Entries whose name starts with `.` are skipped, which keeps `systemrdl-fmt .`
/// out of `.git` without needing to know what a VCS is.
fn rdl_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];

    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            let hidden = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with('.'));
            if hidden {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rdl") {
                out.push(path);
            }
        }
    }

    out.sort();
    Ok(out)
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

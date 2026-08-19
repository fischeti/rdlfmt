# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-08-19

### Added

- Consecutive one-line component instantiations, parameter definitions, enum
  entries, and trailing comments are aligned into columns. Blank lines,
  multiline statements, and changes in nesting end an aligned run, while a
  comment-only line does not.

### Changed

- Directory walks honour `.gitignore` and `.ignore`, so `target/` and `build/`
  stay out of `rdlfmt .`. Naming a path outright still formats it either way.
- Directory walks are faster: 124 ms to 48 ms on a 48k-entry tree, and 7.1 ms
  to 0.5 ms where an ignore file prunes a build directory outright.

### Fixed

- A symlinked directory is no longer followed, so a link pointing at its own
  ancestor no longer formats the same file dozens of times over.

## [0.1.1] - 2026-08-17

### Added

- A PeakRDL plugin. Installed alongside PeakRDL, the same wheel registers a
  `peakrdl fmt` subcommand — `uv tool install peakrdl-cli --with rdlfmt`.
  Unlike PeakRDL's other subcommands it does not compile or elaborate its
  input, since formatting needs the source text rather than the register model,
  so it formats files that parse but do not yet elaborate.

## [0.1.0] - 2026-08-16

First release.

### Added

- `rdlfmt`, a formatter for SystemRDL 2.0 following the
  [PeakRDL style guide](https://peakrdl.readthedocs.io/en/latest/style-guide.html):
  four-space indentation, opening brace on the statement's line, closing brace
  on its own line before the instance name, spaces around assignment and
  expression operators.
- Line breaks between statements are preserved rather than imposed, so
  deliberate grouping survives and the style guide's `sw`/`hw` exception needs
  no special case. Runs of blank lines collapse to one.
- Comments and preprocessor directives are preserved, including `` `ifdef ``
  branches, which are laid out as trivia rather than parsed as code.
- Line endings are taken from the input: a CRLF file stays CRLF, an LF file
  stays LF.
- Output is verified before it is returned. The result is re-lexed and compared
  token by token against the input; if anything but whitespace moved, the
  output is withheld rather than written. Input that does not parse cleanly is
  refused rather than reformatted from a guessed-at tree.
- CLI: files are rewritten in place by default, with `--check` for CI,
  `--diff`, `--stdout`, directory traversal, and stdin/stdout when given no
  path.
- Library API: `rdlfmt::format`, plus `rdlfmt::syntax` for the lexer and the
  lossless rowan CST underneath. The CLI dependencies sit behind the default
  `cli` feature, so library users can turn them off.

[Unreleased]: https://github.com/fischeti/rdlfmt/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/fischeti/rdlfmt/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/fischeti/rdlfmt/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/fischeti/rdlfmt/releases/tag/v0.1.0

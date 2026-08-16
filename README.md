# rdlfmt

A formatter for [SystemRDL](https://www.accellera.org/downloads/standards/systemrdl) 2.0,
following the [PeakRDL style guide](https://peakrdl.readthedocs.io/en/latest/style-guide.html).

Before:

```systemrdl
addrmap top{
  reg {
      field{sw=rw;
    hw=r;} data[31:0];   // payload
  }ctrl @0x0;


  reg{field{sw=r;hw=w;}status[7:0];}stat@0x4;
};
```

After:

```systemrdl
addrmap top {
    reg {
        field {
            sw = rw;
            hw = r;
        } data[31:0]; // payload
    } ctrl @ 0x0;

    reg {
        field {
            sw = r; hw = w;
        } status[7:0];
    } stat @ 0x4;
};
```

## Install

> **TODO:** not yet published to crates.io — this will not work until it is.

```bash
cargo install rdlfmt
```

Until then, build from source (Rust 1.85+):

```bash
cargo build --release
```

## Usage

Format a file, rewriting it in place — this is the default, and what you want
most of the time:

```bash
rdlfmt regs.rdl
```

Format every `.rdl` file in a directory tree:

```bash
rdlfmt .
```

Check without writing anything. Exits 1 if any file is not formatted, which is
the one for CI:

```bash
rdlfmt --check .
```

Same, but show what would change:

```bash
rdlfmt --diff regs.rdl
```

Write to stdout and leave the file alone:

```bash
rdlfmt --stdout regs.rdl
```

With no path at all, it reads stdin and writes stdout:

```bash
cat regs.rdl | rdlfmt
```

## What it does

The style guide's rules, applied mechanically: four spaces per level and never
tabs, opening brace on the same line as the statement it belongs to, closing
brace on its own line followed by the instance name, spaces around assignment
and expression operators, no space before the `;` that follows a `}`.

Line breaks *between* statements are yours. `rdlfmt` neither forces one
statement per line nor joins them, so grouping you put there on purpose
survives — which is also how the style guide's `sw`/`hw` exception is
accommodated without a special case. Note the two registers in the example
above: the first one's properties stay split, the second's stay joined. Runs of
blank lines collapse to one.

There is nothing to configure, deliberately. A formatter earns its value by
ending arguments, not by relocating them into a config file.

Comments and preprocessor directives survive. The parser builds a lossless
concrete syntax tree, so every byte of the input is present in the tree —
a `` `ifdef `` or a trailing `//` comment is data to be placed, not noise to be
dropped.

## What it will not do

**Format a file it did not fully understand.** If the input has syntax errors,
`rdlfmt` reports them and refuses, rather than reformatting a structure it had
to guess at.

**Change your code.** Every result is verified before it is returned: the output
is re-lexed and compared token by token against the input, and if anything but
whitespace moved, the output is withheld and you get a bug report instead of a
damaged file. That check is what makes rewriting files in place the default.

## Library

The formatter is also a library:

```rust
let formatted = rdlfmt::format(source)?;
```

`rdlfmt::syntax` exposes the lexer and the CST underneath, if you want the tree
rather than the text. Turn off default features to drop the CLI dependencies.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

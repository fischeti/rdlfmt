# rdlfmt

A formatter for [SystemRDL](https://www.accellera.org/downloads/standards/systemrdl) 2.0,
following the [PeakRDL style guide](https://peakrdl.readthedocs.io/en/latest/style-guide.html).

**[Documentation](https://fischeti.github.io/rdlfmt/)**

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

Line breaks *between* statements are yours — note that the first register's
properties stay split and the second's stay joined. There is nothing to
configure, deliberately. See
[What it does](https://fischeti.github.io/rdlfmt/style.html).

## Install

A prebuilt binary, needing no toolchain of any kind:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/fischeti/rdlfmt/releases/latest/download/rdlfmt-installer.sh | sh
```

Or, if you already manage `peakrdl` with a Python tool:

```bash
uv tool install rdlfmt
```

Installed alongside [PeakRDL](https://peakrdl.readthedocs.io), the same wheel
registers a `peakrdl fmt` subcommand. `cargo install rdlfmt` works too. See
[Installation](https://fischeti.github.io/rdlfmt/installation.html) for
Windows, PyPI and building from source.

## Usage

```bash
rdlfmt regs.rdl     # rewrite in place -- the default
rdlfmt .            # every .rdl file in the tree
rdlfmt --check .    # exit 1 if anything is unformatted; the one for CI
rdlfmt --diff .     # ...and show what would change
rdlfmt -            # filter stdin to stdout
```

That last one is what editors want:
[Editor integration](https://fischeti.github.io/rdlfmt/editors.html) has
format-on-save recipes for Neovim, Vim, VS Code, Zed, Helix and Emacs. Full
command line in [Usage](https://fischeti.github.io/rdlfmt/usage.html).

## Library

The formatter is also a library:

```rust
let formatted = rdlfmt::format(source)?;
```

`rdlfmt::syntax` exposes the lexer and the lossless CST underneath. API docs on
[docs.rs](https://docs.rs/rdlfmt).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

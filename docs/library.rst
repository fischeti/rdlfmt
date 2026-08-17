Library
=======

The formatter is also a Rust library.

.. code-block:: bash

    cargo add rdlfmt --no-default-features

.. code-block:: rust

    let formatted = rdlfmt::format(source)?;

``format`` returns ``Result<String, FormatError>``. The error carries the parse
diagnostics, each with a byte range into the input, so a caller can report them
against its own source map.

``rdlfmt::syntax`` exposes the layer underneath, if you want the tree rather
than the text: ``lex`` for the token stream, ``parse`` for a lossless
`rowan <https://docs.rs/rowan>`_ concrete syntax tree with every comment and
directive still in it, and ``SyntaxNode``/``SyntaxToken`` to walk it.

The ``cli`` feature is on by default and pulls in ``clap`` and friends for the
binary. The library half depends on none of them, so
``default-features = false`` drops the lot and leaves ``logos`` and ``rowan``.

Full API documentation is on `docs.rs <https://docs.rs/rdlfmt>`_.

.. code-block:: bash

    cargo run --example dump-cst -- regs.rdl

is the quickest way to see what the tree looks like.

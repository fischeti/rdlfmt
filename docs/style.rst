What it does
============

The `PeakRDL style guide <https://peakrdl.readthedocs.io/en/latest/style-guide.html>`_'s
rules, applied mechanically:

- Four spaces per level of nesting, and never tabs.
- The opening brace on the same line as the statement it belongs to.
- The closing brace on its own line, followed by the instance name.
- Spaces around assignment and expression operators.
- No space before the ``;`` that follows a ``}``.

There is nothing to configure, deliberately. A formatter earns its value by
ending arguments, not by relocating them into a config file.


What stays yours
----------------

Line breaks *between* statements. ``rdlfmt`` neither forces one statement per
line nor joins them, so grouping you put there on purpose survives — which is
also how the style guide's ``sw``/``hw`` exception is accommodated without a
special case. Note the two registers here: the first one's properties stay
split, the second's stay joined.

.. code-block:: systemrdl

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

Runs of blank lines collapse to one.

Comments and preprocessor directives survive. The parser builds a lossless
concrete syntax tree, so every byte of the input is present in the tree — a
:literal:`\`ifdef` or a trailing ``//`` comment is data to be placed, not noise
to be dropped.


What it will not do
-------------------

**Format a file it did not fully understand.** If the input has syntax errors,
``rdlfmt`` reports them and refuses, rather than reformatting a structure it
had to guess at. The rules assume a tree shape that error recovery does not
guarantee, and rewriting a file whose structure was guessed at is how a
formatter corrupts code.

The same check covers a preprocessor conditional that hands a brace back and
forth between its branches: it leaves the braces unbalanced, so it is refused
rather than mangled.

.. _verification:

**Change your code.** Every result is verified before it is returned: the
output is re-lexed and compared token by token against the input, and if
anything but whitespace moved, the output is withheld and you get a bug report
instead of a damaged file. That check is what makes rewriting files in place
the default — and what makes it safe to wire ``rdlfmt`` into
:doc:`save <editors>`.

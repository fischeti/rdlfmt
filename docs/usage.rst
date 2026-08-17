Usage
=====

Format a file, rewriting it in place — this is the default, and what you want
most of the time:

.. code-block:: bash

    rdlfmt regs.rdl

There is no ``--write`` flag. A flag you pass every single time carries no
information, which is the same call ``rustfmt``, ``black`` and ``ruff`` make.
What makes rewriting in place defensible is that every result is
:ref:`verified before it is returned <verification>`.

Format every ``.rdl`` file in a directory tree:

.. code-block:: bash

    rdlfmt .

What the walk descends into is what ``git`` would consider part of the tree:
``.gitignore`` and ``.ignore`` are honoured, and entries whose name begins with
a dot are skipped. Between them that keeps ``.git``, ``target/`` and ``build/``
out of it without a flag and without ``rdlfmt`` needing to know what any of
those are. A ``.gitignore`` applies whether or not there is still a ``.git``
beside it, so an exported or vendored tree behaves like the one it came from.

Ignore rules only prune what a walk *discovers*. Naming a path outright formats
it either way:

.. code-block:: bash

    rdlfmt build/regs.rdl    # formatted, even though build/ is ignored
    rdlfmt build/            # searched, even though build/ is ignored

Files that are already formatted are left untouched rather than rewritten with
identical bytes, so formatting a tree does not bump every mtime and set every
rebuild going.


Other modes
-----------

Check without writing anything. Exits ``1`` if any file is not formatted, which
is the one for CI:

.. code-block:: bash

    rdlfmt --check .

Same, but show what would change:

.. code-block:: bash

    rdlfmt --diff regs.rdl

Write to stdout and leave the file alone:

.. code-block:: bash

    rdlfmt --stdout regs.rdl

With no path at all — or with ``-`` as the path — it reads stdin and writes
stdout:

.. code-block:: bash

    cat regs.rdl | rdlfmt

That last form is the one editors want; see :doc:`editors`.


.. _exit-codes:

Exit codes
----------

.. list-table::
    :header-rows: 1
    :widths: 10 90

    * - Code
      - Meaning
    * - ``0``
      - Everything is formatted, or was formatted.
    * - ``1``
      - Under ``--check`` or ``--diff``: at least one file is not formatted.
    * - ``2``
      - At least one file could not be formatted — a syntax error, or a file
        that could not be read or written. Diagnostics go to stderr as
        ``path:line:col: message``.

Under ``--check`` and ``--diff`` a clean run prints nothing at all. When
rewriting, the path of each file that changed goes to stdout, one per line, and
the summary count goes to stderr — so the paths stay pipeable.


.. _continuous-integration:

Continuous integration
----------------------

GitHub Actions:

.. code-block:: yaml

    - name: Check SystemRDL formatting
      run: uvx rdlfmt --check .

Use ``--diff`` instead of ``--check`` if you would rather the log show what is
wrong than just which files are.


pre-commit
----------

`pre-commit <https://pre-commit.com>`_ can install ``rdlfmt`` from PyPI itself,
so contributors need nothing on their ``PATH``:

.. code-block:: yaml

    repos:
      - repo: local
        hooks:
          - id: rdlfmt
            name: rdlfmt
            entry: rdlfmt
            language: python
            additional_dependencies: [rdlfmt]
            types: [file]
            files: \.rdl$

The hook rewrites the offending files, which is what you want locally: stage
them and commit again.

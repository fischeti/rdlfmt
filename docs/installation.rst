Installation
============

``rdlfmt`` is a single self-contained binary. Pick whichever of these fits the
toolchain you already have.


Prebuilt binary
---------------

Needs no toolchain of any kind:

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/fischeti/rdlfmt/releases/latest/download/rdlfmt-installer.sh | sh

On Windows:

.. code-block:: powershell

    powershell -c "irm https://github.com/fischeti/rdlfmt/releases/latest/download/rdlfmt-installer.ps1 | iex"

Binaries for each platform are also attached to every
`release <https://github.com/fischeti/rdlfmt/releases>`_ if you would rather
place one yourself.


From PyPI
---------

Convenient if you already manage ``peakrdl`` this way. The package is the Rust
binary in a wheel, not a Python program, so it pulls in nothing else:

.. code-block:: bash

    uv tool install rdlfmt

Or without installing at all:

.. code-block:: bash

    uvx rdlfmt --check .

``pip install rdlfmt`` works the same way.


From crates.io
--------------

If you have a Rust toolchain:

.. code-block:: bash

    cargo install rdlfmt


From source
-----------

Rust 1.88 or newer:

.. code-block:: bash

    git clone https://github.com/fischeti/rdlfmt
    cd rdlfmt
    cargo build --release


.. _peakrdl-plugin:

As a PeakRDL plugin
-------------------

Installed into the same environment as
`PeakRDL <https://peakrdl.readthedocs.io>`_, the wheel also registers a
``peakrdl fmt`` subcommand:

.. code-block:: bash

    uv tool install peakrdl-cli --with rdlfmt

.. code-block:: bash

    peakrdl fmt --check .

It takes the same arguments as the ``rdlfmt`` command and returns the same
:ref:`exit codes <exit-codes>` — it runs the very same binary, which ships in
the wheel next to the ``peakrdl`` executable.

Unlike every other PeakRDL subcommand, ``fmt`` does not compile or elaborate
your design. It reads the source text, so it can format a file that does not
yet elaborate.

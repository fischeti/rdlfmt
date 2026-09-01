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

The package is the Rust binary in a wheel, not a Python program, so it pulls
in nothing else:

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

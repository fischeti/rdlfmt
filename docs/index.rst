rdlfmt
======

A formatter for `SystemRDL <https://www.accellera.org/downloads/standards/systemrdl>`_ 2.0,
following the `PeakRDL style guide <https://peakrdl.readthedocs.io/en/latest/style-guide.html>`_.

Before:

.. code-block:: systemrdl

    addrmap top{
      reg {
          field{sw=rw;
        hw=r;} data[31:0];   // payload
      }ctrl @0x0;


      reg{field{sw=r;hw=w;}status[7:0];}stat@0x4;
    };

After:

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

There is nothing to configure, deliberately. A formatter earns its value by
ending arguments, not by relocating them into a config file.


Quick start
-----------

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/fischeti/rdlfmt/releases/latest/download/rdlfmt-installer.sh | sh
    rdlfmt regs.rdl

See :doc:`installation` for the other ways to get it, :doc:`usage` for the rest
of the command line, and :doc:`editors` for format-on-save.


Links
-----

- `Source repository <https://github.com/fischeti/rdlfmt>`_
- `Release notes <https://github.com/fischeti/rdlfmt/releases>`_
- `Issue tracker <https://github.com/fischeti/rdlfmt/issues>`_
- `crates.io <https://crates.io/crates/rdlfmt>`_ · `PyPI <https://pypi.org/project/rdlfmt>`_ · `docs.rs <https://docs.rs/rdlfmt>`_
- `SystemRDL specification <https://www.accellera.org/downloads/standards/systemrdl>`_


.. toctree::
    :hidden:

    installation
    usage
    editors
    style
    library

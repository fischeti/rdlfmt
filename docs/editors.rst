Editor integration
==================

``rdlfmt`` has no language server and no editor plugin, and does not need
either. Run with ``-`` as its path it reads a buffer on stdin and writes the
formatted text to stdout:

.. code-block:: bash

    rdlfmt -

That is exactly the shape every editor's "filter the buffer through an external
command" hook expects, so format-on-save is a handful of lines of config
wherever you work. The recipes below all use it; a failed format exits non-zero
and prints diagnostics on stderr, leaving stdout empty, so a file with a syntax
error is left as you typed it rather than being emptied.

Most editors do not know what a ``.rdl`` file is out of the box, so where a
filetype has to be declared before anything can hook onto it, the recipe does
that first.

.. warning::

    The recipes below are offered in good faith, not tested in CI. What *is*
    tested is the behaviour every one of them rests on: that ``rdlfmt -``
    round-trips a buffer, and that a file it cannot parse leaves stdout empty
    and exits non-zero. The glue around that is your editor's, and editors
    move. If one of these has rotted, please
    `open an issue <https://github.com/fischeti/rdlfmt/issues>`_.


Neovim
------

With `conform.nvim <https://github.com/stevearc/conform.nvim>`_:

.. code-block:: lua

    vim.filetype.add({ extension = { rdl = "systemrdl" } })

    require("conform").setup({
      formatters = {
        rdlfmt = {
          command = "rdlfmt",
          args = { "-" },
          stdin = true,
        },
      },
      formatters_by_ft = {
        systemrdl = { "rdlfmt" },
      },
      format_on_save = {
        timeout_ms = 1000,
        lsp_format = "fallback",
      },
    })


Vim
---

No plugin needed — filter the buffer through ``rdlfmt`` before the write, and
undo it if the format failed:

.. code-block:: vim

    function! s:RdlFmt() abort
      let l:view = winsaveview()
      silent %!rdlfmt -
      if v:shell_error
        silent undo
        echohl ErrorMsg | echom 'rdlfmt: left unformatted' | echohl None
      endif
      call winrestview(l:view)
    endfunction

    augroup rdlfmt
      autocmd!
      autocmd BufWritePre *.rdl call s:RdlFmt()
    augroup END

``winsaveview()``/``winrestview()`` keep the cursor and the scroll position
where they were, which a bare ``%!`` would not.


VS Code
-------

VS Code will not run an arbitrary command as a formatter, so this needs the
`Run on Save <https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave>`_
extension. It runs *after* the write, so this one uses in-place formatting
rather than stdin, and VS Code reloads the file:

.. code-block:: json

    {
      "emeraldwalk.runonsave": {
        "commands": [
          {
            "match": "\\.rdl$",
            "cmd": "rdlfmt ${file}"
          }
        ]
      }
    }

Put that in ``.vscode/settings.json`` to scope it to one project, or in your
user settings for all of them.


Zed
---

Install the `SystemRDL extension <https://zed.dev/extensions/systemrdl>`_ first
— it claims ``.rdl`` and registers the language under the name ``SystemRDL``,
which is what the settings below key off. In ``settings.json``:

.. code-block:: json

    {
      "languages": {
        "SystemRDL": {
          "format_on_save": "on",
          "formatter": {
            "external": {
              "command": "rdlfmt",
              "arguments": ["-"]
            }
          }
        }
      }
    }

The extension brings a tree-sitter grammar with it, so this is the one editor
here that gets highlighting and formatting from the same five minutes of setup.


Helix
-----

Helix has no SystemRDL language built in, so declare one. In
``~/.config/helix/languages.toml``:

.. code-block:: toml

    [[language]]
    name = "systemrdl"
    scope = "source.systemrdl"
    file-types = ["rdl"]
    indent = { tab-width = 4, unit = "    " }
    formatter = { command = "rdlfmt", args = ["-"] }
    auto-format = true

There is no tree-sitter grammar for SystemRDL, so this buys you formatting and
indentation, not highlighting. ``hx --health systemrdl`` will say as much.


Emacs
-----

With `reformatter.el <https://github.com/purcell/reformatter.el>`_:

.. code-block:: elisp

    (define-derived-mode systemrdl-mode prog-mode "SystemRDL"
      "Major mode for SystemRDL sources."
      (setq-local comment-start "// ")
      (setq-local indent-tabs-mode nil)
      (setq-local tab-width 4))

    (add-to-list 'auto-mode-alist '("\\.rdl\\'" . systemrdl-mode))

    (reformatter-define rdlfmt
      :program "rdlfmt"
      :args '("-")
      :lighter " RDLFmt")

    (add-hook 'systemrdl-mode-hook #'rdlfmt-on-save-mode)

``reformatter-define`` also gives you ``M-x rdlfmt-buffer`` and
``M-x rdlfmt-region`` for the times you want it on demand.


Anything else
-------------

If your editor can pipe the buffer through a shell command on save, the command
is ``rdlfmt -`` and there is nothing else to know. If it can only run a command
on a path, use ``rdlfmt <file>`` and let the editor notice the file changed
underneath it.

Formatting on save is not a substitute for a ``rdlfmt --check .`` job in CI —
it is what keeps that job from ever failing. See
:ref:`Continuous integration <continuous-integration>`.

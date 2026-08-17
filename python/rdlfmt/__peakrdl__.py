"""PeakRDL plugin descriptor, exposing the formatter as ``peakrdl fmt``.

PeakRDL only has two plugin hooks, importers and exporters, and a formatter is
neither: it reads SystemRDL source and writes SystemRDL source, so it never
wants the elaborated register model that an exporter is handed. What it wants
is the bytes on disk, comments and all.

So this subclasses the exporter plugin -- the only class PeakRDL will load from
the ``peakrdl.exporters`` entry point group -- but overrides ``main()`` rather
than ``do_export()``. ``main()`` is the method the CLI actually calls; the
compile-and-elaborate step lives in the base class's implementation of it, and
skipping that is what lets ``peakrdl fmt`` format a file that does not yet
elaborate.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import sysconfig
from typing import TYPE_CHECKING

from peakrdl.plugins.exporter import ExporterSubcommandPlugin

if TYPE_CHECKING:
    import argparse

    from peakrdl.plugins.importer import ImporterPlugin


def _binary() -> str:
    """Absolute path to the ``rdlfmt`` executable shipped in this wheel.

    maturin's ``bindings = "bin"`` installs it into the environment's scripts
    directory, which is where the ``peakrdl`` executable itself lives, so it is
    found next to the running interpreter whether or not the environment has
    been activated.
    """
    name = "rdlfmt.exe" if os.name == "nt" else "rdlfmt"
    candidates = [
        os.path.join(os.path.dirname(sys.executable), name),
        os.path.join(sysconfig.get_path("scripts"), name),
    ]
    for candidate in candidates:
        if os.path.isfile(candidate):
            return candidate

    found = shutil.which(name)
    if found:
        return found

    print(
        f"error: could not find the {name} executable. It ships in the rdlfmt "
        "wheel; try reinstalling it.",
        file=sys.stderr,
    )
    sys.exit(2)


class Formatter(ExporterSubcommandPlugin):
    short_desc = "Format SystemRDL source"
    long_desc = (
        "Format SystemRDL source.\n\n"
        "Rewrites each PATH in place. A directory is searched for `.rdl` files. "
        "With no PATH, reads stdin and writes stdout.\n\n"
    )

    # No -o: the formatter rewrites its inputs.
    generates_output_file = False

    def add_arguments(
        self, parser: argparse._ActionsContainer, importers: list[ImporterPlugin]
    ) -> None:
        # Deliberately not calling super(): the base class adds the compile,
        # elaborate and importer argument groups, none of which apply.
        parser.add_argument(
            "paths",
            metavar="PATH",
            nargs="*",
            help="Files or directories to format; omit to read stdin",
        )
        parser.add_argument(
            "-c",
            "--check",
            action="store_true",
            help="Write nothing; exit 1 if any file is not formatted",
        )
        parser.add_argument(
            "-d",
            "--diff",
            action="store_true",
            help="Like --check, but show what would change",
        )
        parser.add_argument(
            "--stdout",
            action="store_true",
            help="Write to stdout instead of rewriting the file",
        )

    def main(
        self, importers: list[ImporterPlugin], options: argparse.Namespace
    ) -> None:
        argv = [_binary()]
        if options.check:
            argv.append("--check")
        if options.diff:
            argv.append("--diff")
        if options.stdout:
            argv.append("--stdout")
        argv += options.paths

        # rdlfmt's exit code is the meaningful one: 1 means "not formatted",
        # which is what a CI job checks for.
        sys.exit(subprocess.call(argv))

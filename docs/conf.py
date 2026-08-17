"""Sphinx configuration for the rdlfmt documentation site."""

import datetime
import tomllib
from pathlib import Path

project = "rdlfmt"
author = "Tim Fischer"
copyright = f"{datetime.date.today().year}, {author}"  # noqa: A001
html_title = project

# Taken from Cargo.toml, so the site cannot claim a version the tool does not
# have. Cargo.toml is the single source of truth for crates.io and PyPI too.
_manifest = tomllib.loads(
    (Path(__file__).parent.parent / "Cargo.toml").read_text(encoding="utf-8")
)
version = release = _manifest["package"]["version"]

extensions = [
    "sphinx_copybutton",
]

# `.venv` is uv's environment for this docs project; without it Sphinx tries to
# render every stray README in site-packages.
exclude_patterns = ["_build", ".venv", "Thumbs.db", ".DS_Store"]

# Most blocks on this site are shell commands, so an untagged one is bash.
highlight_language = "bash"

html_theme = "sphinx_book_theme"
html_static_path = []

html_theme_options = {
    "repository_url": "https://github.com/fischeti/rdlfmt",
    "path_to_docs": "docs",
    "use_download_button": False,
    "use_source_button": True,
    "use_repository_button": True,
    "use_issues_button": True,
    "home_page_in_toc": True,
}

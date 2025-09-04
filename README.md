# comrak

<!-- [![downloads](https://static.pepy.tech/badge/comrak/month)](https://pepy.tech/project/comrak) -->
[![uv](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/uv/main/assets/badge/v0.json)](https://github.com/astral-sh/uv)
[![pdm-managed](https://img.shields.io/badge/pdm-managed-blueviolet)](https://pdm.fming.dev)
[![PyPI](https://img.shields.io/pypi/v/comrak.svg)](https://pypi.org/project/comrak)
[![Supported Python versions](https://img.shields.io/pypi/pyversions/comrak.svg)](https://pypi.org/project/comrak)
[![License](https://img.shields.io/pypi/l/comrak.svg)](https://pypi.python.org/pypi/comrak)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/comrak/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/comrak/master)

Python bindings for the Comrak Rust library, a fast CommonMark/GFM parser

## Installation

```bash
pip install comrak
```

### Requirements

- Python 3.9+

## Features

Fast Markdown to HTML parser in Rust, shipped for Python via PyO3.

## API

### `render_markdown`

Render Markdown to HTML:

```python
from comrak import ExtensionOptions, render_markdown
extension_options = ExtensionOptions()
render_markdown("foo :smile:", extension_options)
# '<p>foo :smile:</p>\n'

extension_options.shortcodes = True
render_markdown("foo :smile:", extension_options)
# '<p>foo 😄</p>\n'
```

### `render_markdown_to_commonmark`

Render Markdown to CommonMark:

```python
from comrak import RenderOptions, ListStyleType, render_markdown_to_commonmark

render_options = RenderOptions()
render_markdown_to_commonmark("- one\n- two\n- three", render_options=render_options)

# '- one\n- two\n- three\n' – default is Dash
render_options.list_style = ListStyleType.Plus
render_markdown_to_commonmark("- one\n- two\n- three", render_options=render_options)
# '+ one\n+ two\n+ three\n'
```

### `parse_markdown`

Parse Markdown into an abstract syntax tree (AST):

```python
from comrak import ExtensionOptions, Document, Text, Paragraph, parse_markdown

extension_options = ExtensionOptions()
extension_options.front_matter_delimiter = "---"

md_content = """---
This is a text in FrontMatter
---

Hello, Markdown!
"""

x = parse_markdown(md_content, extension_options)
assert isinstance(x.node_value, Document)
assert not hasattr(x.node_value, "value")
assert len(x.children) == 2

assert isinstance(x.children[0].node_value, FrontMatter)
assert isinstance(x.children[0].node_value.value, str)
assert x.children[0].node_value.value.strip() == "---\nThis is a text in FrontMatter\n---"

assert isinstance(x.children[1].node_value, Paragraph)
assert len(x.children[1].children) == 1
assert isinstance(x.children[1].children[0].node_value, Text)
assert isinstance(x.children[1].children[0].node_value.value, str)
assert x.children[1].children[0].node_value.value == "Hello, Markdown!"
```

### Options

All options are exposed in a simple manner and can be used with both `render_markdown`, `render_markdown_to_commonmark`, and `parse_markdown`.

Refer to the [Comrak docs](https://docs.rs/comrak/latest/comrak/struct.Options.html) for all available options.

## Benchmarks

Tested with small (8 lines) and medium (1200 lines) markdown strings

- vs. [markdown](https://pypi.org/project/markdown): 15x faster (S/M)
- vs. [markdown2](https://pypi.org/project/markdown2): 20x (S) - 60x (M) faster

## Contributing

Maintained by [lmmx](https://github.com/lmmx). Contributions welcome!

1. **Issues & Discussions**: Please open a GitHub issue or discussion for bugs, feature requests, or questions.
2. **Pull Requests**: PRs are welcome!
   - Install the dev extra (e.g. with [uv](https://docs.astral.sh/uv/): `uv pip install -e .[dev]`)
   - Run tests (when available) and include updates to docs or examples if relevant.
   - If reporting a bug, please include the version and the error message/traceback if available.

## License

Licensed under the 2-Clause BSD License. See [LICENSE](https://github.com/lmmx/comrak/blob/master/LICENSE) for all the details.

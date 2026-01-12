# fontinfo

A command-line tool for displaying detailed information about TrueType and OpenType font files.

## Features

- Font names (family, subfamily, PostScript name)
- Font metrics (units per EM, ascender, descender, weight, width)
- OpenType features (GSUB - Glyph Substitution)
- OpenType features (GPOS - Glyph Positioning)
- Supported scripts and writing systems
- Human-readable descriptions for OpenType feature tags

## Installation

Requires Rust toolchain. Install with:

```bash
cargo install --path .
```

Or build and run directly:

```bash
cargo build --release
```

The binary will be available at `target/release/fontinfo`.

## Usage

```bash
fontinfo <font-file>
```

Example:

```bash
fontinfo /path/to/font.ttf
```

## Output

The tool displays comprehensive font information including:

- Family name, subfamily, and PostScript name
- Font version
- Typographic metrics
- Glyph count and spacing information
- Weight, width, and style properties
- Available OpenType features with descriptions
- Supported scripts and languages

## Dependencies

- [ttf-parser](https://github.com/RazrFalcon/ttf-parser) - Zero-allocation TrueType font parser

## License

See LICENSE file for details.

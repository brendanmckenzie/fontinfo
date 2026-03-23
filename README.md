# fontinfo

A command-line tool for displaying detailed information about TrueType and OpenType font files.

## Features

- Font names (family, subfamily, full name, PostScript name, version)
- Font metrics (units per EM, ascender, descender, line gap, weight, width)
- Style flags (bold, italic, oblique, monospaced)
- Glyph count
- OpenType features (GSUB - Glyph Substitution) with human-readable descriptions
- OpenType features (GPOS - Glyph Positioning) with human-readable descriptions
- Supported scripts and writing systems
- JSON output mode for scripting and tooling integration

## Installation

Requires the [Rust toolchain](https://rustup.rs).

**From GitHub:**

```bash
cargo install --git https://github.com/brendanmckenzie/fontinfo
```

**From a local clone:**

```bash
git clone https://github.com/brendanmckenzie/fontinfo
cd fontinfo
cargo install --path .
```

**Build without installing:**

```bash
cargo build --release
```

The binary will be at `target/release/fontinfo`.

## Usage

```
fontinfo [--json] <font-file>
```

### Flags

| Flag     | Description                          |
|----------|--------------------------------------|
| `--json` | Output font information as JSON      |

### Examples

**Default (human-readable) output:**

```bash
fontinfo /path/to/font.ttf
```

**JSON output:**

```bash
fontinfo --json /path/to/font.ttf
```

**Pipe JSON into jq:**

```bash
fontinfo --json /path/to/font.ttf | jq '.metrics'
```

## Output

### Human-readable

Displays font information in formatted sections:

- **FONT NAMES** — family, subfamily, full name, PostScript name, version
- **FONT METRICS** — units per EM, ascender, descender, line gap, glyph count, weight, width, and style flags
- **OPENTYPE FEATURES (GSUB)** — glyph substitution features with descriptions
- **OPENTYPE FEATURES (GPOS)** — glyph positioning features with descriptions
- **SUPPORTED SCRIPTS** — writing systems the font supports

### JSON

Returns a single JSON object with the following structure:

```json
{
  "file": "/path/to/font.ttf",
  "names": {
    "family": "My Font",
    "subfamily": "Regular",
    "full_name": "My Font Regular",
    "postscript_name": "MyFont-Regular",
    "version": "Version 1.000"
  },
  "metrics": {
    "units_per_em": 1000,
    "ascender": 800,
    "descender": -200,
    "line_gap": 0,
    "glyph_count": 512,
    "is_monospaced": false,
    "is_bold": false,
    "is_italic": false,
    "is_oblique": false,
    "weight": 400,
    "width": "Normal"
  },
  "gsub_features": [
    { "tag": "calt", "description": "Contextual Alternates" },
    { "tag": "liga", "description": "Standard Ligatures" }
  ],
  "gpos_features": [
    { "tag": "kern", "description": "Kerning" }
  ],
  "scripts": ["DFLT", "latn"]
}
```

## Dependencies

- [ttf-parser](https://github.com/RazrFalcon/ttf-parser) — Zero-allocation TrueType/OpenType font parser
- [serde_json](https://github.com/serde-rs/json) — JSON serialization

## License

See LICENSE file for details.

# pyshapdf

**Python bindings for [shapdf](https://github.com/Teddy-van-Jerry/shapdf) - Create Shapes into PDF**

Efficient programmable generation of shapes in PDF using Rust and Python.

## Installation

```sh
pip install pyshapdf
```

## Quick Start

```python
import pyshapdf

script = """
page letter
circle 100mm 150mm 20mm color=blue
rectangle 50mm 50mm 40mm 30mm color=green angle=45deg anchor=center
line 50mm 200mm 150mm 250mm width=2mm color=red cap=round
"""

pyshapdf.render_script(script, "output.pdf")
```

## Script Syntax

The `.shapdf` script language supports:

### Page Commands
```python
page default                    # Default page size
page letter                     # US Letter (8.5" × 11")
page letter-landscape           # US Letter landscape
page a4                         # A4 (210mm × 297mm)
page a4-landscape               # A4 landscape
page size 200mm 300mm           # Custom size
```

### Shape Commands
```python
# Circle
circle <x> <y> <radius> [color=...]

# Rectangle
rectangle <x> <y> <width> <height> [color=...] [anchor=...] [angle=...]

# Line
line <x1> <y1> <x2> <y2> [width=...] [color=...] [cap=...]
```

### Settings
```python
set default_page_size 200mm 300mm
set default_width 2mm
set default_color blue
set default_cap round           # butt | round | square
set default_angle 45deg         # or 0.785rad
```

### Units
- `mm` - millimeters
- `cm` - centimeters
- `in` - inches
- `pt` - points (1/72 inch)
- `deg` - degrees (default for angles)
- `rad` - radians

### Colors
- Named colors: `red`, `green`, `blue`, `yellow`, `black`, `white`, `gray`
- Hex: `#RRGGBB` (e.g., `#ff0000`)
- RGB: `rgb(255, 0, 0)`
- Grayscale: `gray(0.5)`

### Anchor Points (for rectangles)
- `bottomleft` (default)
- `center`
- `topleft`, `topright`, `bottomright`
- `midleft`, `midright`, `midbottom`, `midtop`

## API Reference

### `render_script(script: str, output_path: str) -> None`

Render a `.shapdf` script to a PDF file.

**Parameters:**
- `script` (str): The `.shapdf` script content
- `output_path` (str): Path to write the output PDF file

**Raises:**
- `RuntimeError`: If PDF generation fails

## Examples

### Multi-page Document
```python
import pyshapdf

script = """
# Page 1 - Circles
page letter
circle 100mm 150mm 30mm color=blue
circle 150mm 150mm 30mm color=red

# Page 2 - Rectangles
page a4
rectangle 105mm 148.5mm 50mm 40mm color=green anchor=center angle=30deg
"""

pyshapdf.render_script(script, "multi_page.pdf")
```

### Technical Drawing
```python
import pyshapdf

script = """
page size 200mm 150mm
set default_width 0.5mm
set default_color black

# Grid lines
line 20mm 20mm 180mm 20mm
line 20mm 50mm 180mm 50mm
line 20mm 80mm 180mm 80mm
line 20mm 110mm 180mm 110mm

line 20mm 20mm 20mm 110mm
line 50mm 20mm 50mm 110mm
line 100mm 20mm 100mm 110mm
line 150mm 20mm 150mm 110mm
line 180mm 20mm 180mm 110mm

# Highlight
circle 100mm 65mm 2mm color=red
rectangle 85mm 50mm 30mm 30mm color=#0066ff anchor=center angle=0deg
"""

pyshapdf.render_script(script, "technical.pdf")
```

## Links

- **🌐 Try it online:** [shapdf.wqzhao.org](https://shapdf.wqzhao.org)
- **📚 Rust Documentation:** [docs.rs/shapdf](https://docs.rs/shapdf)
- **💻 Source Code:** [github.com/Teddy-van-Jerry/shapdf](https://github.com/Teddy-van-Jerry/shapdf)
- **🐛 Report Issues:** [GitHub Issues](https://github.com/Teddy-van-Jerry/shapdf/issues)

## License

GPL-3.0-or-later

© 2025 [Teddy van Jerry](https://github.com/Teddy-van-Jerry) ([Wuqiong Zhao](https://wqzhao.org))

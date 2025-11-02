# `shapdf` = `shape` + `pdf`
Create Shapes into PDF

GitHub: [Teddy-van-Jerry/shapdf](https://github.com/Teddy-van-Jerry/shapdf)

## Motivation
- Efficient programmable generation of shapes in PDF (rather than slow compilation of LaTeX [Ti*k*Z](https://tikz.dev/) or Typst [CeTZ](https://cetz-package.github.io/));
- Minimal dependencies in Rust, relying mostly on **PDF primitives**.

## Capabilities
- [x] Shapes
  - [x] Line
  - [x] Circle (filled)
  - [x] Rectangle (filled)
  - [ ] Polygon
  - [ ] Text
- [x] Color
- [ ] Opacity
- [x] Rotation & Anchor
- [x] PDF Stream Compression (feature `compress`)

More features are coming soon!

## Example
The usage of this library is quite simple:
```rust
use shapdf::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = Generator::new("output/shapes.pdf".into());
    generator.add_page(); // use the default page size (US letter)
    generator
        .circle(Mm(20.), Mm(20.), Mm(10.))
        .with_color(NamedColor("blue"))
        .draw();
    generator
        .line(Pt(500.), Pt(600.), Pt(300.), Pt(400.))
        .with_width(Mm(10.))
        .with_cap_type(CapType::Round)
        .with_color(NamedColor("red"))
        .draw();
    generator.add_page_letter();
    generator
        .rectangle(Mm(80.), Mm(180.), Mm(50.), Mm(30.))
        .with_anchor(Anchor::Center)
        .with_angle(Degree(30.))
        .draw();
    generator
        .circle(Mm(80.), Mm(180.), Mm(1.))
        .with_color(NamedColor("green"))
        .draw();
    generator.add_page_a4();
    generator.write_pdf()?;
    println!("PDF generated successfully!");
    Ok(())
}
```
More examples are available in the [`examples`](examples) directory.

## CLI Usage
The binary reads declarative `.shapdf` scripts and renders them to PDF.

Install via `cargo install shapdf` (use `cargo install --path .` when working from a local checkout).

- `shapdf <script.shapdf>` renders the file to `<script>.pdf` in place.
- `shapdf --output output/shape.pdf -` reads the script from `stdin` (e.g. piped from another program).
- Sample scripts and helpers live in [`examples/cli/`](examples/cli/):
  - [`sample_shapes.shapdf`](examples/cli/sample_shapes.shapdf): demonstrates multiple pages, colors, anchors, and rotations.
  - [`generate_sample.sh`](examples/cli/generate_sample.sh): runs the CLI against the script file and writes `sample_shapes.pdf` beside it.
  - [`generate_stdin.sh`](examples/cli/generate_stdin.sh): inlines the same script, pipes it over `stdin`, and produces `sample_shapes_inline.pdf`.

Library consumers can also call `shapdf::render_script_to_pdf(script, output_path)` to execute a script string directly.

### `.shapdf` Script Syntax
- Lines are `command [args] [key=value ...]`; blank lines or those starting with `#`/`//` are ignored.
- Supported commands:
  - `page default|letter|letter-landscape|a4|a4-landscape`
  - `page size <width> <height>` (accepts `mm`, `cm`, `in`, `pt`)
  - `set default_page_size <width> <height>`
  - `set default_width <length>`
  - `set default_color <color>` (named colors, `#RRGGBB`, `rgb(r,g,b)`, or `gray(v)`)
  - `set default_cap butt|round|square`
  - `set default_angle <value>` (`deg` default, or `rad`)
  - `line <x1> <y1> <x2> <y2> [width=...] [color=...] [cap=...]`
  - `circle <x> <y> <radius> [color=...]`
  - `rectangle <x> <y> <width> <height> [color=...] [anchor=...] [angle=...]`
- The first drawing command automatically inserts a default page if none was added.

## Implementation Facts
- Filled circle is actually implemented using [a zero-length line with the rounded line cap](https://stackoverflow.com/a/46897816/15080514).

## License
This project is distributed under the [GPL-3.0 License](LICENSE).

© 2025 [Teddy van Jerry](https://github.com/Teddy-van-Jerry) ([Wuqiong Zhao](https://wqzhao.org))

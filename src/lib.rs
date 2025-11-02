//! # `shapdf` = `shape` + `pdf`
//! Create Shapes into PDF
//!
//! GitHub: [Teddy-van-Jerry/shapdf](https://github.com/Teddy-van-Jerry/shapdf)
//!
//! ## Motivation
//! - Efficient programmable generation of shapes in PDF (rather than lengthy compilation of LaTeX [Ti*k*Z](https://tikz.dev/) or Typst [CeTZ](https://cetz-package.github.io/));
//! - Minimal dependencies in Rust, relying mostly on **PDF primitives**.
//!
//! ## Capabilities
//! - [x] Shapes
//!   - [x] Line
//!   - [x] Circle (filled)
//!   - [x] Rectangle (filled)
//!   - [ ] Polygon
//!   - [ ] Text
//! - [x] Color
//! - [ ] Opacity
//! - [x] Rotation & Anchor
//! - [x] PDF Stream Compression (feature `compress`)
//!
//! More features are coming soon!
//!
//! ## Example
//! The usage of this library is quite simple:
//! ```rust
//! use shapdf::*;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut generator = Generator::new("output/shapes.pdf".into());
//!     generator.add_page(); // use the default page size (US letter)
//!     generator
//!         .circle(Mm(20.), Mm(20.), Mm(10.))
//!         .with_color(NamedColor("blue"))
//!         .draw();
//!     generator
//!         .line(Pt(500.), Pt(600.), Pt(300.), Pt(400.))
//!         .with_width(Mm(10.))
//!         .with_cap_type(CapType::Round)
//!         .with_color(NamedColor("red"))
//!         .draw();
//!     generator.add_page_letter();
//!     generator
//!         .rectangle(Mm(80.), Mm(180.), Mm(50.), Mm(30.))
//!         .with_anchor(Anchor::Center)
//!         .with_angle(Degree(30.))
//!         .draw();
//!     generator
//!         .circle(Mm(80.), Mm(180.), Mm(1.))
//!         .with_color(NamedColor("green"))
//!         .draw();
//!     generator.add_page_a4();
//!     generator.write_pdf()?;
//!     println!("PDF generated successfully!");
//!     Ok(())
//! }
//! ```
//! More examples are available in the [`examples`](https://github.com/Teddy-van-Jerry/shapdf/tree/master/examples) directory.
//!
//! ## Implementation Facts
//! - Filled circle is actually implemented using [a zero-length line with the rounded line cap](https://stackoverflow.com/a/46897816/15080514).
//!
//! ## License
//! This project is distributed under the [GPL-3.0 License](https://github.com/Teddy-van-Jerry/shapdf/blob/master/LICENSE).
//!
//! © 2025 [Teddy van Jerry](https://github.com/Teddy-van-Jerry) ([Wuqiong Zhao](https://wqzhao.org))

mod generator;
mod script;
mod shapes;
mod units;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub use generator::*;
#[cfg(feature = "wasm")]
pub use script::render_script_to_bytes;
#[cfg(not(feature = "wasm"))]
pub use script::render_script_to_pdf;
pub use script::{
    execute_instructions, parse_script, ExecutionError, Instruction, InstructionKind, ParseError,
};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_script(script: &str) -> Result<Vec<u8>, JsValue> {
    render_script_to_bytes(script).map_err(|err| JsValue::from_str(&err.to_string()))
}

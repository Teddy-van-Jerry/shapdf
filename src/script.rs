use crate::{
    generator::Generator,
    shapes::{Anchor, CapType, Shape},
    units::{Degree, Gray, Length, Mm, Pt, Radian, Rgb, RGB},
};
use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub line: usize,
    pub kind: InstructionKind,
}

#[derive(Debug, Clone)]
pub enum InstructionKind {
    AddPage(PageKind),
    DrawLine(LineSpec),
    DrawCircle(CircleSpec),
    DrawRectangle(RectSpec),
    SetDefaultPageSize {
        width: LengthValue,
        height: LengthValue,
    },
    SetDefaultWidth(LengthValue),
    SetDefaultColor(ColorValue),
    SetDefaultCapType(CapType),
    SetDefaultAngle(AngleValue),
}

#[derive(Debug, Clone, Copy)]
pub enum PageKind {
    Default,
    Letter,
    LetterLandscape,
    A4,
    A4Landscape,
    Custom {
        width: LengthValue,
        height: LengthValue,
    },
}

#[derive(Debug, Clone)]
pub struct LineSpec {
    pub x1: LengthValue,
    pub y1: LengthValue,
    pub x2: LengthValue,
    pub y2: LengthValue,
    pub width: Option<LengthValue>,
    pub color: Option<ColorValue>,
    pub cap: Option<CapType>,
}

#[derive(Debug, Clone)]
pub struct CircleSpec {
    pub x: LengthValue,
    pub y: LengthValue,
    pub radius: LengthValue,
    pub color: Option<ColorValue>,
}

#[derive(Debug, Clone)]
pub struct RectSpec {
    pub x: LengthValue,
    pub y: LengthValue,
    pub width: LengthValue,
    pub height: LengthValue,
    pub color: Option<ColorValue>,
    pub anchor: Option<Anchor>,
    pub angle: Option<AngleValue>,
}

#[derive(Debug, Clone, Copy)]
pub struct LengthValue(pub f64);

impl LengthValue {
    pub fn as_pt(self) -> Pt {
        Pt(self.0)
    }

    fn from_length<L: Length>(length: L) -> Self {
        Self(length.to_points())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AngleValue {
    radians: f64,
}

impl AngleValue {
    pub fn as_radian(self) -> Radian {
        Radian(self.radians)
    }

    pub fn as_degree(self) -> Degree {
        Degree(self.radians.to_degrees())
    }

    fn from_radians(radians: f64) -> Self {
        Self { radians }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorValue {
    Named(NamedColorValue),
    Gray(f64),
    RgbFloat { r: f64, g: f64, b: f64 },
    Rgb { r: u8, g: u8, b: u8 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamedColorValue {
    Black,
    White,
    Gray,
    Red,
    Green,
    Blue,
    Yellow,
}

impl NamedColorValue {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "black" => Some(Self::Black),
            "white" => Some(Self::White),
            "gray" | "grey" => Some(Self::Gray),
            "red" => Some(Self::Red),
            "green" => Some(Self::Green),
            "blue" => Some(Self::Blue),
            "yellow" => Some(Self::Yellow),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl ParseError {
    fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line, self.message)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct ExecutionError {
    pub line: usize,
    pub message: String,
}

impl ExecutionError {
    fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line, self.message)
    }
}

impl Error for ExecutionError {}

pub fn parse_script(input: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions = Vec::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let command = parse_line(trimmed, line_no)?;
        if let Some(kind) = command {
            instructions.push(Instruction {
                line: line_no,
                kind,
            });
        }
    }

    Ok(instructions)
}

fn parse_line(line: &str, line_no: usize) -> Result<Option<InstructionKind>, ParseError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return Ok(None);
    }

    let cmd = tokens[0].to_ascii_lowercase();
    match cmd.as_str() {
        "page" => parse_page_command(&tokens, line_no),
        "line" => parse_line_command(&tokens, line_no),
        "circle" => parse_circle_command(&tokens, line_no),
        "rectangle" => parse_rectangle_command(&tokens, line_no),
        "set" => parse_set_command(&tokens, line_no),
        _ => Err(ParseError::new(
            line_no,
            format!("Unknown command '{}'", tokens[0]),
        )),
    }
}

fn parse_page_command(tokens: &[&str], line: usize) -> Result<Option<InstructionKind>, ParseError> {
    let spec = tokens
        .get(1)
        .ok_or_else(|| ParseError::new(line, "Missing page specification"))?
        .to_ascii_lowercase();
    let kind = match spec.as_str() {
        "default" => PageKind::Default,
        "letter" => PageKind::Letter,
        "letter-landscape" => PageKind::LetterLandscape,
        "a4" => PageKind::A4,
        "a4-landscape" => PageKind::A4Landscape,
        "size" => {
            let width = tokens
                .get(2)
                .ok_or_else(|| ParseError::new(line, "Missing page width"))?;
            let height = tokens
                .get(3)
                .ok_or_else(|| ParseError::new(line, "Missing page height"))?;
            PageKind::Custom {
                width: parse_length(width).map_err(|msg| ParseError::new(line, msg))?,
                height: parse_length(height).map_err(|msg| ParseError::new(line, msg))?,
            }
        }
        other => {
            return Err(ParseError::new(
                line,
                format!("Unknown page type '{other}'"),
            ));
        }
    };
    Ok(Some(InstructionKind::AddPage(kind)))
}

fn parse_line_command(tokens: &[&str], line: usize) -> Result<Option<InstructionKind>, ParseError> {
    if tokens.len() < 5 {
        return Err(ParseError::new(
            line,
            "Line requires four positional arguments",
        ));
    }

    let x1 = parse_length(tokens[1]).map_err(|msg| ParseError::new(line, msg))?;
    let y1 = parse_length(tokens[2]).map_err(|msg| ParseError::new(line, msg))?;
    let x2 = parse_length(tokens[3]).map_err(|msg| ParseError::new(line, msg))?;
    let y2 = parse_length(tokens[4]).map_err(|msg| ParseError::new(line, msg))?;

    let mut width = None;
    let mut color = None;
    let mut cap = None;

    for token in tokens.iter().skip(5) {
        let (key, value) = split_key_value(token, line)?;
        match key.as_str() {
            "width" => width = Some(parse_length(value).map_err(|msg| ParseError::new(line, msg))?),
            "color" => color = Some(parse_color(value).map_err(|msg| ParseError::new(line, msg))?),
            "cap" => match value {
                "butt" => cap = Some(CapType::Butt),
                "round" => cap = Some(CapType::Round),
                "square" => cap = Some(CapType::Square),
                other => {
                    return Err(ParseError::new(line, format!("Unknown cap type '{other}'")));
                }
            },
            other => {
                return Err(ParseError::new(
                    line,
                    format!("Unknown line option '{other}'"),
                ));
            }
        }
    }

    Ok(Some(InstructionKind::DrawLine(LineSpec {
        x1,
        y1,
        x2,
        y2,
        width,
        color,
        cap,
    })))
}

fn parse_circle_command(
    tokens: &[&str],
    line: usize,
) -> Result<Option<InstructionKind>, ParseError> {
    if tokens.len() < 4 {
        return Err(ParseError::new(
            line,
            "Circle requires three positional arguments",
        ));
    }

    let x = parse_length(tokens[1]).map_err(|msg| ParseError::new(line, msg))?;
    let y = parse_length(tokens[2]).map_err(|msg| ParseError::new(line, msg))?;
    let radius = parse_length(tokens[3]).map_err(|msg| ParseError::new(line, msg))?;

    let mut color = None;
    for token in tokens.iter().skip(4) {
        let (key, value) = split_key_value(token, line)?;
        match key.as_str() {
            "color" => color = Some(parse_color(value).map_err(|msg| ParseError::new(line, msg))?),
            other => {
                return Err(ParseError::new(
                    line,
                    format!("Unknown circle option '{other}'"),
                ));
            }
        }
    }

    Ok(Some(InstructionKind::DrawCircle(CircleSpec {
        x,
        y,
        radius,
        color,
    })))
}

fn parse_rectangle_command(
    tokens: &[&str],
    line: usize,
) -> Result<Option<InstructionKind>, ParseError> {
    if tokens.len() < 5 {
        return Err(ParseError::new(
            line,
            "Rectangle requires four positional arguments",
        ));
    }

    let x = parse_length(tokens[1]).map_err(|msg| ParseError::new(line, msg))?;
    let y = parse_length(tokens[2]).map_err(|msg| ParseError::new(line, msg))?;
    let width = parse_length(tokens[3]).map_err(|msg| ParseError::new(line, msg))?;
    let height = parse_length(tokens[4]).map_err(|msg| ParseError::new(line, msg))?;

    let mut color = None;
    let mut anchor = None;
    let mut angle = None;

    for token in tokens.iter().skip(5) {
        let (key, value) = split_key_value(token, line)?;
        match key.as_str() {
            "color" => color = Some(parse_color(value).map_err(|msg| ParseError::new(line, msg))?),
            "anchor" => {
                anchor =
                    Some(parse_anchor(value).ok_or_else(|| {
                        ParseError::new(line, format!("Unknown anchor '{value}'"))
                    })?)
            }
            "angle" => angle = Some(parse_angle(value).map_err(|msg| ParseError::new(line, msg))?),
            other => {
                return Err(ParseError::new(
                    line,
                    format!("Unknown rectangle option '{other}'"),
                ));
            }
        }
    }

    Ok(Some(InstructionKind::DrawRectangle(RectSpec {
        x,
        y,
        width,
        height,
        color,
        anchor,
        angle,
    })))
}

fn parse_set_command(tokens: &[&str], line: usize) -> Result<Option<InstructionKind>, ParseError> {
    let target = tokens
        .get(1)
        .ok_or_else(|| ParseError::new(line, "Missing property name"))?
        .to_ascii_lowercase();
    match target.as_str() {
        "default_page_size" => {
            let width = tokens
                .get(2)
                .ok_or_else(|| ParseError::new(line, "Missing page width"))?;
            let height = tokens
                .get(3)
                .ok_or_else(|| ParseError::new(line, "Missing page height"))?;
            Ok(Some(InstructionKind::SetDefaultPageSize {
                width: parse_length(width).map_err(|msg| ParseError::new(line, msg))?,
                height: parse_length(height).map_err(|msg| ParseError::new(line, msg))?,
            }))
        }
        "default_width" => {
            let width = tokens
                .get(2)
                .ok_or_else(|| ParseError::new(line, "Missing width value"))?;
            Ok(Some(InstructionKind::SetDefaultWidth(
                parse_length(width).map_err(|msg| ParseError::new(line, msg))?,
            )))
        }
        "default_color" => {
            let color = tokens
                .get(2)
                .ok_or_else(|| ParseError::new(line, "Missing color value"))?;
            Ok(Some(InstructionKind::SetDefaultColor(
                parse_color(color).map_err(|msg| ParseError::new(line, msg))?,
            )))
        }
        "default_cap" => {
            let cap = tokens
                .get(2)
                .ok_or_else(|| ParseError::new(line, "Missing cap type"))?;
            let cap_type = match cap.to_ascii_lowercase().as_str() {
                "butt" => CapType::Butt,
                "round" => CapType::Round,
                "square" => CapType::Square,
                other => {
                    return Err(ParseError::new(line, format!("Unknown cap type '{other}'")));
                }
            };
            Ok(Some(InstructionKind::SetDefaultCapType(cap_type)))
        }
        "default_angle" => {
            let angle = tokens
                .get(2)
                .ok_or_else(|| ParseError::new(line, "Missing angle value"))?;
            Ok(Some(InstructionKind::SetDefaultAngle(
                parse_angle(angle).map_err(|msg| ParseError::new(line, msg))?,
            )))
        }
        other => Err(ParseError::new(line, format!("Unknown property '{other}'"))),
    }
}

fn split_key_value(token: &str, line: usize) -> Result<(String, &str), ParseError> {
    let (key, value) = token.split_once('=').ok_or_else(|| {
        ParseError::new(line, format!("Expected key=value option, found '{token}'"))
    })?;
    Ok((key.to_ascii_lowercase(), value))
}

fn parse_length(token: &str) -> Result<LengthValue, String> {
    let token = token.trim();
    let (num_part, unit) = split_number_unit(token)?;
    let value: f64 = num_part
        .parse()
        .map_err(|_| format!("Invalid number '{num_part}'"))?;
    if value.is_nan() || value.is_infinite() {
        return Err(format!("Invalid length value '{token}'"));
    }

    let unit_lower = unit.to_ascii_lowercase();

    let length = match unit_lower.as_str() {
        "mm" => LengthValue::from_length(Mm(value)),
        "cm" => LengthValue::from_length(Mm(value * 10.0)),
        "in" | "inch" => LengthValue(value * 72.0),
        "pt" | "" => LengthValue(value),
        _ => return Err(format!("Unsupported unit '{unit}'")),
    };

    Ok(length)
}

fn parse_angle(token: &str) -> Result<AngleValue, String> {
    let (num_part, unit) = split_number_unit(token)?;
    let value: f64 = num_part
        .parse()
        .map_err(|_| format!("Invalid number '{num_part}'"))?;
    if value.is_nan() || value.is_infinite() {
        return Err(format!("Invalid angle value '{token}'"));
    }

    let unit_lower = unit.to_ascii_lowercase();

    let radians = match unit_lower.as_str() {
        "deg" | "degree" | "degrees" | "" => value.to_radians(),
        "rad" | "radian" | "radians" => value,
        _ => return Err(format!("Unsupported angle unit '{unit}'")),
    };

    Ok(AngleValue::from_radians(radians))
}

fn parse_color(token: &str) -> Result<ColorValue, String> {
    let lower = token.to_ascii_lowercase();
    if let Some(named) = NamedColorValue::from_str(&lower) {
        return Ok(ColorValue::Named(named));
    }

    if let Some(hex) = lower.strip_prefix('#') {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| format!("Invalid hex color '{token}'"))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| format!("Invalid hex color '{token}'"))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| format!("Invalid hex color '{token}'"))?;
            return Ok(ColorValue::Rgb { r, g, b });
        } else {
            return Err(format!("Hex colors must be of the form #RRGGBB: '{token}'"));
        }
    }

    if let Some(rgb) = lower.strip_prefix("rgb(") {
        let remaining = rgb
            .strip_suffix(')')
            .ok_or_else(|| format!("Invalid rgb() format '{token}'"))?;
        let parts: Vec<&str> = remaining.split(',').map(|part| part.trim()).collect();
        if parts.len() != 3 {
            return Err(format!("rgb() requires three components: '{token}'"));
        }
        let r: f64 = parts[0]
            .parse()
            .map_err(|_| format!("Invalid rgb component '{}': '{token}'", parts[0]))?;
        let g: f64 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid rgb component '{}': '{token}'", parts[1]))?;
        let b: f64 = parts[2]
            .parse()
            .map_err(|_| format!("Invalid rgb component '{}': '{token}'", parts[2]))?;
        return Ok(ColorValue::RgbFloat { r, g, b });
    }

    if let Some(gray) = lower.strip_prefix("gray(") {
        let value = gray
            .strip_suffix(')')
            .ok_or_else(|| format!("Invalid gray() format '{token}'"))?;
        let g: f64 = value
            .parse()
            .map_err(|_| format!("Invalid gray value '{}': '{token}'", value))?;
        return Ok(ColorValue::Gray(g));
    }

    Err(format!("Unsupported color specification '{token}'"))
}

fn parse_anchor(token: &str) -> Option<Anchor> {
    match token.to_ascii_lowercase().as_str() {
        "center" => Some(Anchor::Center),
        "north" => Some(Anchor::North),
        "south" => Some(Anchor::South),
        "east" => Some(Anchor::East),
        "west" => Some(Anchor::West),
        "northeast" => Some(Anchor::NorthEast),
        "northwest" => Some(Anchor::NorthWest),
        "southeast" => Some(Anchor::SouthEast),
        "southwest" => Some(Anchor::SouthWest),
        _ => None,
    }
}

fn split_number_unit(token: &str) -> Result<(&str, &str), String> {
    let mut index = token.len();
    for (i, ch) in token.char_indices().rev() {
        if ch == '.' || ch == '-' || ch == '+' || ch.is_ascii_digit() {
            index = i + ch.len_utf8();
            break;
        }
    }

    if index == 0 {
        return Err(format!("Missing numeric value in '{token}'"));
    }

    let number = &token[..index];
    let unit = &token[index..];
    Ok((number, unit))
}

pub fn execute_instructions(
    generator: &mut Generator,
    instructions: &[Instruction],
) -> Result<(), ExecutionError> {
    let mut has_page = false;

    for instruction in instructions {
        match &instruction.kind {
            InstructionKind::AddPage(kind) => {
                apply_page(generator, *kind);
                has_page = true;
            }
            InstructionKind::SetDefaultPageSize { width, height } => {
                Generator::set_default_page_size(width.as_pt(), height.as_pt());
            }
            InstructionKind::SetDefaultWidth(width) => {
                Generator::set_default_width(width.as_pt());
            }
            InstructionKind::SetDefaultColor(color) => {
                apply_default_color(color.clone());
            }
            InstructionKind::SetDefaultCapType(cap) => {
                Generator::set_default_cap_type(*cap);
            }
            InstructionKind::SetDefaultAngle(angle) => {
                Generator::set_default_angle(angle.as_degree());
            }
            other => {
                if !has_page {
                    apply_page(generator, PageKind::Default);
                    has_page = true;
                }
                match other {
                    InstructionKind::DrawLine(spec) => apply_line(generator, spec.clone()),
                    InstructionKind::DrawCircle(spec) => apply_circle(generator, spec.clone()),
                    InstructionKind::DrawRectangle(spec) => {
                        apply_rectangle(generator, spec.clone())
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    if !has_page {
        return Err(ExecutionError::new(
            0,
            "Script did not add any page. Add at least one 'page' command.",
        ));
    }

    Ok(())
}

#[cfg(not(feature = "wasm"))]
pub fn render_script_to_pdf(
    script: &str,
    output_path: impl AsRef<std::path::Path>,
) -> Result<(), Box<dyn Error>> {
    let instructions = parse_script(script)?;
    let output: PathBuf = output_path.as_ref().to_path_buf();
    let mut generator = Generator::new(output);
    execute_instructions(&mut generator, &instructions)?;
    generator.write_pdf()?;
    Ok(())
}

#[cfg(feature = "wasm")]
pub fn render_script_to_bytes(script: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let instructions = parse_script(script)?;
    let mut generator = Generator::new(PathBuf::new());
    execute_instructions(&mut generator, &instructions)?;
    Ok(generator.to_pdf_bytes())
}

fn apply_page(generator: &mut Generator, kind: PageKind) {
    match kind {
        PageKind::Default => generator.add_page(),
        PageKind::Letter => generator.add_page_letter(),
        PageKind::LetterLandscape => generator.add_page_letter_landscape(),
        PageKind::A4 => generator.add_page_a4(),
        PageKind::A4Landscape => generator.add_page_a4_landscape(),
        PageKind::Custom { width, height } => {
            generator.add_page_with_size(width.as_pt(), height.as_pt())
        }
    }
}

fn apply_default_color(color: ColorValue) {
    match color {
        ColorValue::Named(name) => match name {
            NamedColorValue::Black => Generator::set_default_color(Rgb(0.0, 0.0, 0.0)),
            NamedColorValue::White => Generator::set_default_color(Rgb(1.0, 1.0, 1.0)),
            NamedColorValue::Gray => Generator::set_default_color(Rgb(0.5, 0.5, 0.5)),
            NamedColorValue::Red => Generator::set_default_color(Rgb(1.0, 0.0, 0.0)),
            NamedColorValue::Green => Generator::set_default_color(Rgb(0.0, 1.0, 0.0)),
            NamedColorValue::Blue => Generator::set_default_color(Rgb(0.0, 0.0, 1.0)),
            NamedColorValue::Yellow => Generator::set_default_color(Rgb(1.0, 1.0, 0.0)),
        },
        ColorValue::Gray(value) => Generator::set_default_color(Gray(value)),
        ColorValue::RgbFloat { r, g, b } => Generator::set_default_color(Rgb(r, g, b)),
        ColorValue::Rgb { r, g, b } => Generator::set_default_color(RGB(r, g, b)),
    }
}

fn apply_line(generator: &mut Generator, spec: LineSpec) {
    let mut shape = generator.line(
        spec.x1.as_pt(),
        spec.y1.as_pt(),
        spec.x2.as_pt(),
        spec.y2.as_pt(),
    );
    if let Some(width) = spec.width {
        shape.with_width(width.as_pt());
    }
    if let Some(cap) = spec.cap {
        shape.with_cap_type(cap);
    }

    if let Some(color) = spec.color.clone() {
        with_color(&mut shape, color);
    } else {
        shape.draw();
    }
}

fn with_color(shape: &mut Shape<'_>, color: ColorValue) {
    match color {
        ColorValue::Named(name) => {
            match name {
                NamedColorValue::Black => shape.with_color(Gray(0.0)),
                NamedColorValue::White => shape.with_color(Gray(1.0)),
                NamedColorValue::Gray => shape.with_color(Gray(0.5)),
                NamedColorValue::Red => shape.with_color(Rgb(1.0, 0.0, 0.0)),
                NamedColorValue::Green => shape.with_color(Rgb(0.0, 1.0, 0.0)),
                NamedColorValue::Blue => shape.with_color(Rgb(0.0, 0.0, 1.0)),
                NamedColorValue::Yellow => shape.with_color(Rgb(1.0, 1.0, 0.0)),
            };
        }
        ColorValue::Gray(value) => {
            shape.with_color(Gray(value));
        }
        ColorValue::RgbFloat { r, g, b } => {
            shape.with_color(Rgb(r, g, b));
        }
        ColorValue::Rgb { r, g, b } => {
            shape.with_color(RGB(r, g, b));
        }
    }
    shape.draw();
}

fn apply_circle(generator: &mut Generator, spec: CircleSpec) {
    let mut shape = generator.circle(spec.x.as_pt(), spec.y.as_pt(), spec.radius.as_pt());
    if let Some(color) = spec.color.clone() {
        with_color(&mut shape, color);
    } else {
        shape.draw();
    }
}

fn apply_rectangle(generator: &mut Generator, spec: RectSpec) {
    let mut shape = generator.rectangle(
        spec.x.as_pt(),
        spec.y.as_pt(),
        spec.width.as_pt(),
        spec.height.as_pt(),
    );

    if let Some(angle) = spec.angle {
        shape.with_angle(angle.as_degree());
    }
    if let Some(anchor) = spec.anchor {
        shape.with_anchor(anchor);
    }

    if let Some(color) = spec.color.clone() {
        with_color(&mut shape, color);
    } else {
        shape.draw();
    }
}

#[cfg(all(test, not(feature = "wasm")))]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_basic_script() {
        let script = r#"
            # Comment
            page default
            line 10mm 20mm 30mm 40mm width=2pt color=#FF0000
            circle 10pt 40pt 5mm
            rectangle 10mm 10mm 30mm 20mm anchor=center angle=45deg color=rgb(0.5,0.5,0.1)
        "#;

        let instructions = parse_script(script).expect("Failed to parse script");
        assert_eq!(instructions.len(), 4);
    }

    #[test]
    fn execute_script_creates_pdf() {
        let script = r#"
            page a4
            line 10mm 20mm 50mm 50mm color=red width=2mm
            circle 30mm 40mm 10mm color=gray(0.2)
            rectangle 60mm 60mm 30mm 20mm angle=30deg color=#00FF00
        "#;

        let instructions = parse_script(script).expect("Failed to parse script");
        let output = std::env::temp_dir().join("shapdf-test.pdf");
        let mut generator = Generator::new(output.clone());
        execute_instructions(&mut generator, &instructions).expect("Execution failed");
        generator
            .write_pdf()
            .expect("writing PDF should succeed in test");

        let content = fs::read(&output).expect("pdf file should exist");
        assert!(content.starts_with(b"%PDF"));
        let _ = fs::remove_file(output);
    }

    #[test]
    fn render_script_to_pdf_writes_file() {
        let script = r#"
            page default
            line 10mm 10mm 50mm 50mm color=blue
        "#;
        let output = std::env::temp_dir().join("shapdf-inline-test.pdf");
        render_script_to_pdf(script, &output).expect("rendering should succeed");
        let metadata = fs::metadata(&output).expect("output file should exist");
        assert!(metadata.len() > 0);
        let _ = fs::remove_file(output);
    }
}

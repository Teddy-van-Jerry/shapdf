use crate::units::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Copy, Clone)]
pub enum CapType {
    Butt,
    Round,
    Square,
}

impl CapType {
    pub fn to_int(&self) -> i32 {
        match self {
            CapType::Butt => 0,
            CapType::Round => 1,
            CapType::Square => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    Center,
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Point(f64, f64),
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::SouthWest
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Line,
    Circle,
    Rectangle,
    Polygon,
    Unknown,
}

impl Default for ShapeType {
    fn default() -> Self {
        ShapeType::Unknown
    }
}

/// A shape to draw on the PDF.
///
/// This struct is mostly internal and should not be used directly.
/// You should use the methods provided by the [`Generator`](crate::Generator) struct.
#[derive(Debug, Default)]
pub struct Shape<'a> {
    pub enum_type: ShapeType,
    pub content_stream: Option<&'a mut Vec<u8>>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub width: Option<f64>,
    pub radius: Option<f64>,
    pub angle: Option<f64>, // angle in radius
    pub anchor: Option<Anchor>,
    pub cap_type: Option<CapType>,
    pub color: Option<(f64, f64, f64)>,
}

static DEFAULT_WIDTH: Lazy<Mutex<f64>> = Lazy::new(|| Pt(1.).to_points().into());
static DEFAULT_CAP_TYPE: Lazy<Mutex<CapType>> = Lazy::new(|| CapType::Butt.into());
static DEFAULT_COLOR: Lazy<Mutex<(f64, f64, f64)>> =
    Lazy::new(|| NamedColor("black").to_rgb().into());
static DEFAULT_ANGLE: Lazy<Mutex<f64>> = Lazy::new(|| Degree(0.).to_degrees().into());
static DEFAULT_ANCHOR: Lazy<Mutex<Anchor>> = Lazy::new(|| Anchor::SouthWest.into());

impl<'a> Shape<'a> {
    pub fn draw(&mut self) {
        if let Some(content) = self.content_stream.as_mut() {
            let (r, g, b) = self.color.unwrap_or(*DEFAULT_COLOR.lock().unwrap());
            let width = self.width.unwrap_or(*DEFAULT_WIDTH.lock().unwrap());
            let cap_type = self
                .cap_type
                .unwrap_or(*DEFAULT_CAP_TYPE.lock().unwrap())
                .to_int();
            content.extend_from_slice(format!("{} {} {} RG\n", r, g, b).as_bytes());
            match self.enum_type {
                ShapeType::Line => {
                    content.extend_from_slice(
                        format!(
                            "{} w\n{} J\n{} {} m\n{} {} l\nS\n",
                            width, cap_type, self.x[0], self.y[0], self.x[1], self.y[1]
                        )
                        .as_bytes(),
                    );
                }
                ShapeType::Circle => content.extend_from_slice(
                    // Ref: https://stackoverflow.com/a/46897816/15080514
                    format!(
                        "{} w\n1 J\n{} {} m\n{} {} l\nS\n",
                        self.radius.unwrap() * 2.0,
                        self.x[0],
                        self.y[0],
                        self.x[0],
                        self.y[0]
                    )
                    .as_bytes(),
                ),
                ShapeType::Rectangle => {
                    let angle = self.angle.unwrap_or(*DEFAULT_ANGLE.lock().unwrap());
                    let cos_theta = angle.cos();
                    let sin_theta = angle.sin();
                    let (width, height) = (self.x[1], self.y[1]);
                    // (cx, cy): rotation center
                    let (cx, cy) = (self.x[0], self.y[0]);
                    // (x0, y0): south west corner of the rectangle before rotation
                    let (x0, y0) = match self.anchor.unwrap_or(*DEFAULT_ANCHOR.lock().unwrap()) {
                        Anchor::Center => (self.x[0] - width / 2.0, self.y[0] - height / 2.0),
                        Anchor::North => (self.x[0] - width / 2.0, self.y[0] - height),
                        Anchor::South => (self.x[0] - width / 2.0, self.y[0]),
                        Anchor::East => (self.x[0] - width, self.y[0] - height / 2.0),
                        Anchor::West => (self.x[0], self.y[0] - height / 2.0),
                        Anchor::NorthEast => (self.x[0] - width, self.y[0] - height),
                        Anchor::NorthWest => (self.x[0], self.y[0] - height),
                        Anchor::SouthEast => (self.x[0] - width, self.y[0]),
                        Anchor::SouthWest => (self.x[0], self.y[0]),
                        Anchor::Point(px, py) => (px, py),
                    };
                    let translate_x = cx - cos_theta * cx + sin_theta * cy;
                    let translate_y = cy - sin_theta * cx - cos_theta * cy;
                    content.extend_from_slice(
                        format!(
                            "{} {} {} {} {} {} cm\n{} {} {} {} re f\n",
                            cos_theta,
                            sin_theta,
                            -sin_theta,
                            cos_theta,
                            translate_x,
                            translate_y,
                            x0,
                            y0,
                            width,
                            height
                        )
                        .as_bytes(),
                    );
                }
                _ => {}
            };
        }
    }

    /// Set the width of the shape and return a mutable reference to self.
    pub fn with_width(&mut self, width: impl Length) -> &mut Self {
        self.width = Some(width.to_points());
        self
    }

    pub fn with_angle(&mut self, angle: impl Angle) -> &mut Self {
        self.angle = Some(angle.to_radians());
        self
    }

    pub fn with_anchor(&mut self, anchor: Anchor) -> &mut Self {
        self.anchor = Some(anchor);
        self
    }

    pub fn with_cap_type(&mut self, cap_type: CapType) -> &mut Self {
        self.cap_type = Some(cap_type);
        self
    }

    pub fn with_color(&mut self, color: impl Color) -> &mut Self {
        self.color = Some(color.to_rgb());
        self
    }

    pub fn get_default_width() -> Pt {
        Pt(*DEFAULT_WIDTH.lock().unwrap())
    }

    pub fn set_default_width(width: impl Length) {
        *DEFAULT_WIDTH.lock().unwrap() = width.to_points();
    }

    pub fn get_default_cap_type() -> CapType {
        *DEFAULT_CAP_TYPE.lock().unwrap()
    }

    pub fn set_default_cap_type(cap_type: CapType) {
        *DEFAULT_CAP_TYPE.lock().unwrap() = cap_type;
    }

    pub fn get_default_color() -> Rgb {
        let (r, g, b) = *DEFAULT_COLOR.lock().unwrap();
        Rgb(r, g, b)
    }

    pub fn set_default_color(color: impl Color) {
        *DEFAULT_COLOR.lock().unwrap() = color.to_rgb();
    }

    pub fn get_default_angle() -> Degree {
        Degree(*DEFAULT_ANGLE.lock().unwrap())
    }

    pub fn set_default_angle(angle: impl Angle) {
        *DEFAULT_ANGLE.lock().unwrap() = angle.to_radians();
    }

    pub fn get_default_anchor() -> Anchor {
        *DEFAULT_ANCHOR.lock().unwrap()
    }

    pub fn set_default_anchor(anchor: Anchor) {
        *DEFAULT_ANCHOR.lock().unwrap() = anchor;
    }
}

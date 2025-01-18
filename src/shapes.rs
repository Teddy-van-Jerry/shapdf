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

#[derive(Debug, Default)]
pub struct Shape<'a> {
    pub enum_type: ShapeType,
    pub content_stream: Option<&'a mut Vec<u8>>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub radius: Option<f64>,
    pub angle: Option<f64>, // angle in radius
    pub cap_type: Option<CapType>,
    pub color: Option<(f64, f64, f64)>,
}

static DEFAULT_WIDTH: Lazy<Mutex<f64>> = Lazy::new(|| Pt(1.).to_points().into());
static DEFAULT_CAP_TYPE: Lazy<Mutex<CapType>> = Lazy::new(|| CapType::Butt.into());
static DEFAULT_COLOR: Lazy<Mutex<(f64, f64, f64)>> =
    Lazy::new(|| NamedColor("black").to_rgb().into());
static DEFAULT_ANGLE: Lazy<Mutex<f64>> = Lazy::new(|| Degree(0.).to_degrees().into());

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
                    let cx = self.x[0] + self.width.unwrap() / 2.0;
                    let cy = self.y[0] + self.height.unwrap() / 2.0;
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
                            self.x[0],
                            self.y[0],
                            self.width.unwrap(),
                            self.height.unwrap()
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

    pub fn with_cap_type(&mut self, cap_type: CapType) -> &mut Self {
        self.cap_type = Some(cap_type);
        self
    }

    pub fn with_color(&mut self, color: impl Color) -> &mut Self {
        self.color = Some(color.to_rgb());
        self
    }

    pub fn set_default_width(width: impl Length) {
        *DEFAULT_WIDTH.lock().unwrap() = width.to_points();
    }

    pub fn set_default_cap_type(cap_type: CapType) {
        *DEFAULT_CAP_TYPE.lock().unwrap() = cap_type;
    }

    pub fn set_default_color(color: impl Color) {
        *DEFAULT_COLOR.lock().unwrap() = color.to_rgb();
    }
}

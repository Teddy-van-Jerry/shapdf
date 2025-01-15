use crate::units::*;
use once_cell::sync::Lazy;

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
    pub content_stream: Option<&'a mut String>,
    pub x1: Option<f64>,
    pub x2: Option<f64>,
    pub y1: Option<f64>,
    pub y2: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub radius: Option<f64>,
    pub cap_type: Option<CapType>,
    pub color: Option<(f64, f64, f64)>,
}

static DEFAULT_WIDTH: Lazy<f64> = Lazy::new(|| Pt(1.).to_points());
static DEFAULT_CAP_TYPE: Lazy<CapType> = Lazy::new(|| CapType::Butt);
static DEFAULT_COLOR: Lazy<(f64, f64, f64)> = Lazy::new(|| (0., 0., 0.));

impl<'a> Shape<'a> {
    pub fn draw(&mut self) {
        if let Some(content) = self.content_stream.as_mut() {
            let (r, g, b) = self.color.unwrap_or(*DEFAULT_COLOR);
            content.push_str(&format!("{} {} {} RG\n", r, g, b));
            content.push_str(&match self.enum_type {
                ShapeType::Line => format!(
                    "{} w\n{} J\n{} {} m\n{} {} l\nS\n",
                    self.width.unwrap_or(*DEFAULT_WIDTH),
                    self.cap_type.unwrap_or(*DEFAULT_CAP_TYPE).to_int(),
                    self.x1.unwrap(),
                    self.y1.unwrap(),
                    self.x2.unwrap(),
                    self.y2.unwrap()
                ),
                _ => "".into(),
            });
        }
    }

    /// Set the width of the shape and return a mutable reference to self.
    pub fn with_width<L: Length>(&mut self, width: L) -> &mut Self {
        self.width = Some(width.to_points());
        self
    }

    pub fn with_cap_type(&mut self, cap_type: CapType) -> &mut Self {
        self.cap_type = Some(cap_type);
        self
    }

    pub fn with_color(&mut self, color: (f64, f64, f64)) -> &mut Self {
        self.color = Some(color);
        self
    }
}

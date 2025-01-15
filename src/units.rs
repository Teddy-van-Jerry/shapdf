pub trait Length {
    fn to_points(&self) -> f64;
}

#[derive(Debug)]
pub struct Mm(pub f64);

impl Length for Mm {
    fn to_points(&self) -> f64 {
        self.0 * 2.83465 // 1 mm = 2.83465 points
    }
}

#[derive(Debug)]
pub struct Cm(pub f64);

impl Length for Cm {
    fn to_points(&self) -> f64 {
        self.0 * 28.3465 // 1 cm = 28.3465 points
    }
}

#[derive(Debug)]
pub struct Inch(pub f64);

impl Length for Inch {
    fn to_points(&self) -> f64 {
        self.0 * 72.0 // 1 inch = 72 points
    }
}

#[derive(Debug)]
pub struct Pt(pub f64);

impl Length for Pt {
    fn to_points(&self) -> f64 {
        self.0
    }
}

pub trait Angle {
    fn to_degrees(&self) -> f64;
    fn to_radians(&self) -> f64;
}

#[derive(Debug)]
pub struct Degrees(pub f64);

impl Angle for Degrees {
    fn to_degrees(&self) -> f64 {
        self.0
    }

    fn to_radians(&self) -> f64 {
        self.0.to_radians()
    }
}

#[derive(Debug)]
pub struct Radians(pub f64);

impl Angle for Radians {
    fn to_degrees(&self) -> f64 {
        self.0.to_degrees()
    }

    fn to_radians(&self) -> f64 {
        self.0
    }
}

use std::{
    fmt,
    ops::{Add, Mul, Sub},
};

pub trait Length:
    Add<Output = Self> + Sub<Output = Self> + Mul<f64, Output = Self> + Sized + Copy
{
    fn to_points(&self) -> f64;

    fn to_mm(&self) -> f64 {
        self.to_points() / 2.83464566929
    }

    fn to_cm(&self) -> f64 {
        self.to_points() / 28.3464566929
    }

    fn to_inch(&self) -> f64 {
        self.to_points() / 72.0
    }

    fn as_pt(&self) -> Pt {
        Pt(self.to_points())
    }

    fn as_mm(&self) -> Mm {
        Mm(self.to_mm())
    }

    fn as_cm(&self) -> Cm {
        Cm(self.to_cm())
    }

    fn as_inch(&self) -> Inch {
        Inch(self.to_inch())
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Mm(pub f64);

impl Length for Mm {
    fn to_points(&self) -> f64 {
        self.0 * 2.83464566929 // 1 mm = 2.83465 points
    }
}

impl fmt::Display for Mm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{}mm", self.to_mm()))
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Cm(pub f64);

impl Length for Cm {
    fn to_points(&self) -> f64 {
        self.0 * 28.3464566929 // 1 cm = 28.3465 points
    }
}

impl fmt::Display for Cm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{}cm", self.to_cm()))
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Inch(pub f64);

impl Length for Inch {
    fn to_points(&self) -> f64 {
        self.0 * 72.0 // 1 inch = 72 points
    }
}

impl fmt::Display for Inch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{}\"", self.to_inch()))
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Pt(pub f64);

impl Length for Pt {
    fn to_points(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{}pt", self.to_points()))
    }
}

pub trait Angle:
    Add<Output = Self> + Sub<Output = Self> + Mul<f64, Output = Self> + Sized + Copy
{
    fn to_degrees(&self) -> f64;
    fn to_radians(&self) -> f64;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Degree(pub f64);

impl Angle for Degree {
    fn to_degrees(&self) -> f64 {
        self.0
    }

    fn to_radians(&self) -> f64 {
        self.0.to_radians()
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{}°", self.to_degrees()))
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Radian(pub f64);

impl Angle for Radian {
    fn to_degrees(&self) -> f64 {
        self.0.to_degrees()
    }

    fn to_radians(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Radian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{}rad", self.to_radians()))
    }
}

macro_rules! impl_add_self {
    ($type:ident) => {
        impl Add for $type {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self {
                    0: self.0 + other.0,
                }
            }
        }
    };
}

macro_rules! impl_sub_self {
    ($type:ident) => {
        impl Sub for $type {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self {
                    0: self.0 - other.0,
                }
            }
        }
    };
}

macro_rules! impl_mul_f64 {
    ($type:ident) => {
        impl Mul<f64> for $type {
            type Output = Self;
            fn mul(self, other: f64) -> Self {
                Self { 0: self.0 * other }
            }
        }
    };
}

macro_rules! impl_add_between_units {
    ($t1:ty, $t2:ty, $f1:ident, $f2:ident) => {
        impl Add<$t2> for $t1 {
            type Output = Self;
            fn add(self, other: $t2) -> Self {
                Self(self.0 + other.$f1())
            }
        }

        impl Add<$t1> for $t2 {
            type Output = $t2;
            fn add(self, other: $t1) -> Self {
                Self(self.0 + other.$f2())
            }
        }
    };
}

impl_add_self!(Mm);
impl_add_self!(Cm);
impl_add_self!(Inch);
impl_add_self!(Pt);
impl_add_self!(Degree);
impl_add_self!(Radian);
impl_sub_self!(Mm);
impl_sub_self!(Cm);
impl_sub_self!(Inch);
impl_sub_self!(Pt);
impl_sub_self!(Degree);
impl_sub_self!(Radian);
impl_mul_f64!(Mm);
impl_mul_f64!(Cm);
impl_mul_f64!(Inch);
impl_mul_f64!(Pt);
impl_mul_f64!(Degree);
impl_mul_f64!(Radian);
impl_add_between_units!(Mm, Cm, to_mm, to_cm);
impl_add_between_units!(Mm, Inch, to_mm, to_inch);
impl_add_between_units!(Mm, Pt, to_mm, to_points);
impl_add_between_units!(Cm, Inch, to_cm, to_inch);
impl_add_between_units!(Cm, Pt, to_cm, to_points);
impl_add_between_units!(Inch, Pt, to_inch, to_points);
impl_add_between_units!(Degree, Radian, to_degrees, to_radians);

pub trait Color {
    fn to_rgb(&self) -> (f64, f64, f64);
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rgb(pub f64, pub f64, pub f64);

impl Color for Rgb {
    fn to_rgb(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Color for RGB {
    fn to_rgb(&self) -> (f64, f64, f64) {
        (
            self.0 as f64 / 255.0,
            self.1 as f64 / 255.0,
            self.2 as f64 / 255.0,
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Gray(pub f64);

impl Color for Gray {
    fn to_rgb(&self) -> (f64, f64, f64) {
        (self.0, self.0, self.0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct NamedColor(pub &'static str);

impl Color for NamedColor {
    fn to_rgb(&self) -> (f64, f64, f64) {
        match self.0 {
            "black" => (0., 0., 0.),
            "white" => (1., 1., 1.),
            "gray" | "grey" => (0.5, 0.5, 0.5),
            "red" => (1., 0., 0.),
            "green" => (0., 1., 0.),
            "blue" => (0., 0., 1.),
            _ => (0., 0., 0.),
        }
    }
}

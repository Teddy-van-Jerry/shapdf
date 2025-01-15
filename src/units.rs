use std::ops::{Add, Sub, Mul};

pub trait Length: Add<Output = Self> + Sub<Output = Self> + Mul<f64, Output = Self> + Sized + Copy {
    fn to_points(&self) -> f64;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Mm(pub f64);

impl Length for Mm {
    fn to_points(&self) -> f64 {
        self.0 * 2.83464566929 // 1 mm = 2.83465 points
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Cm(pub f64);

impl Length for Cm {
    fn to_points(&self) -> f64 {
        self.0 * 28.3464566929 // 1 cm = 28.3465 points
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Inch(pub f64);

impl Length for Inch {
    fn to_points(&self) -> f64 {
        self.0 * 72.0 // 1 inch = 72 points
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Pt(pub f64);

impl Length for Pt {
    fn to_points(&self) -> f64 {
        self.0
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

impl_add_self!(Mm);
impl_add_self!(Cm);
impl_add_self!(Inch);
impl_add_self!(Pt);
impl_sub_self!(Mm);
impl_sub_self!(Cm);
impl_sub_self!(Inch);
impl_sub_self!(Pt);
impl_mul_f64!(Mm);
impl_mul_f64!(Cm);
impl_mul_f64!(Inch);
impl_mul_f64!(Pt);

pub trait Angle {
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

pub trait Color {
    fn to_rgb(&self) -> (f64, f64, f64);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgb(pub f64, pub f64, pub f64);

impl Color for Rgb {
    fn to_rgb(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
}

#[derive(Debug, Default, Copy, Clone)]
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

#[derive(Debug, Default, Copy, Clone)]
pub struct Gray(pub f64);

impl Color for Gray {
    fn to_rgb(&self) -> (f64, f64, f64) {
        (self.0, self.0, self.0)
    }
}

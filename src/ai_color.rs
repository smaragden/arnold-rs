//! AtRGB API
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//use ai_bindings;
pub use ai_bindings::AtRGB;
pub use ai_bindings::AtRGBA;
use std::f32::{INFINITY, NAN};

use std::ops::{Add, Mul};

#[doc(hidden)]
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub trait Color {
    /// Clamp the RGB\[A\] color vector to the specified range.
    fn clamp(&self, lo: f32, hi: f32) -> Self;
    /// Clip negative values.
    fn clip_to_zero(&self) -> Self;
    /// Check for almost black.
    fn is_small(&self, epsilon: f32) -> bool;
    /// Absolute value of color.
    fn abs(&self) -> Self;
    /// Max RGB component of color.
    fn max(&self) -> f32;
    /// Check to see if this color differ by more than a threhsold against another color.
    fn threshold(&self, c2: &Self, t: f32) -> bool;
    /// Convert a RGB color to grey scale (take average of R, G, B)
    fn to_gray(&self) -> f32;
    /// Check to see if an RGB color has any corrupted components (nan or infinite).
    fn is_finite(&self) -> bool;
    /// Barycentric interpolation of triangle vertex colors. 
    fn berp(&self, a: f32, b: f32, c1: &Self, c2: &Self) -> Self;
    //fn heat_map(&self, ...)
}

impl Mul<AtRGB> for f32 {
    type Output = AtRGB;
    fn mul(self, other: AtRGB) -> AtRGB {
        AtRGB{
            r: self * other.r, 
            g: self * other.g, 
            b: self * other.b, 
        }
    }
}

impl<'a> Mul<&'a AtRGB> for f32 {
    type Output = AtRGB;
    fn mul(self, other: &'a AtRGB) -> AtRGB {
        AtRGB{
            r: self * other.r, 
            g: self * other.g, 
            b: self * other.b, 
        }
    }
}

impl Add for AtRGB {
    type Output = AtRGB;
    fn add(self, other: AtRGB) -> AtRGB {
        AtRGB{
            r: self.r + other.r, 
            g: self.g + other.g, 
            b: self.b + other.b, 
        }
    }
}

impl Color for AtRGB {
    fn clamp(&self, lo: f32, hi: f32) -> Self {
        Self {
            r: clamp(self.r, lo, hi),
            g: clamp(self.g, lo, hi),
            b: clamp(self.b, lo, hi),
        }
    }

    fn clip_to_zero(&self) -> Self {
        Self {
            r: self.r.max(0.0),
            g: self.g.max(0.0),
            b: self.b.max(0.0),
        }
    }

    fn is_small(&self, epsilon: f32) -> bool {
        self.r.abs() < epsilon && self.g.abs() < epsilon && self.b.abs() < epsilon
    }

    fn abs(&self) -> Self {
        Self {
            r: self.r.abs(),
            g: self.g.abs(),
            b: self.b.abs(),
        }
    }

    fn max(&self) -> f32 {
        self.r.max(self.g).max(self.b)
    }

    fn threshold(&self, c2: &Self, t: f32) -> bool {
        (self.r - c2.r).abs() >= t || (self.g - c2.g).abs() >= t || (self.b - c2.b).abs() >= t
    }

    fn to_gray(&self) -> f32 {
        (self.r + self.g + self.b) / 3.0
    }

    fn is_finite(&self) -> bool {
        !(self.r.abs() == INFINITY
            || self.r.abs() == NAN
            || self.g.abs() == INFINITY
            || self.g.abs() == NAN
            || self.b.abs() == INFINITY
            || self.b.abs() == NAN)
    }

    fn berp(&self, a: f32, b: f32, c1: &Self, c2: &Self) -> Self {
        let c : f32 = 1.0 - (a+b);
        c*self + a*c1 + b*c2
    }
}

impl Mul<AtRGBA> for f32 {
    type Output = AtRGBA;
    fn mul(self, other: AtRGBA) -> AtRGBA {
        AtRGBA{
            r: self * other.r, 
            g: self * other.g, 
            b: self * other.b, 
            a: self * other.a, 
        }
    }
}

impl<'a> Mul<&'a AtRGBA> for f32 {
    type Output = AtRGBA;
    fn mul(self, other: &'a AtRGBA) -> AtRGBA {
        AtRGBA{
            r: self * other.r, 
            g: self * other.g, 
            b: self * other.b, 
            a: self * other.a, 
        }
    }
}

impl Add for AtRGBA {
    type Output = AtRGBA;
    fn add(self, other: AtRGBA) -> AtRGBA {
        AtRGBA{
            r: self.r + other.r, 
            g: self.g + other.g, 
            b: self.b + other.b, 
            a: self.a + other.a, 
        }
    }
}

impl Color for AtRGBA {
    fn clamp(&self, lo: f32, hi: f32) -> Self {
        Self {
            r: clamp(self.r, lo, hi),
            g: clamp(self.g, lo, hi),
            b: clamp(self.b, lo, hi),
            a: clamp(self.a, lo, hi),
        }
    }

    fn clip_to_zero(&self) -> Self {
        Self {
            r: self.r.max(0.0),
            g: self.g.max(0.0),
            b: self.b.max(0.0),
            a: self.a.max(0.0),
        }
    }

    fn is_small(&self, epsilon: f32) -> bool {
        self.r.abs() < epsilon && self.g.abs() < epsilon && self.b.abs() < epsilon
    }

    fn abs(&self) -> Self {
        Self {
            r: self.r.abs(),
            g: self.g.abs(),
            b: self.b.abs(),
            a: self.a.abs(),
        }
    }

    fn max(&self) -> f32 {
        self.r.max(self.g).max(self.b)
    }

    fn threshold(&self, c2: &Self, t: f32) -> bool {
        (self.r - c2.r).abs() >= t || (self.g - c2.g).abs() >= t || (self.b - c2.b).abs() >= t
    }

    fn to_gray(&self) -> f32 {
        (self.r + self.g + self.b) / 3.0
    }

    fn is_finite(&self) -> bool {
        !(self.r.abs() == INFINITY
            || self.r.abs() == NAN
            || self.g.abs() == INFINITY
            || self.g.abs() == NAN
            || self.b.abs() == INFINITY
            || self.b.abs() == NAN
            || self.a.abs() == INFINITY
            || self.a.abs() == NAN)
    }

    fn berp(&self, a: f32, b: f32, c1: &Self, c2: &Self) -> Self {
        let c : f32 = 1.0 - (a+b);
        c*self + a*c1 + b*c2
    }
}

/// Clamp the RGB\[A\] color vector to the specified range.
pub fn AiRGBClamp<T: Color>(c: &T, lo: f32, hi: f32) -> T {
    c.clamp(lo, hi)
}

/// Clip negative values.
pub fn AiColorClipToZero<T: Color>(c: &T) -> T {
    c.clip_to_zero()
}

/// Check for almost black.
pub fn AiColorIsSmall<T: Color>(c: &T, epsilon: f32) -> bool {
    c.is_small(epsilon)
}

/// Absolute value of color.
pub fn AiColorABS<T: Color>(c: &T) -> T {
    c.abs()
}

/// Max RGB component of color.
///
/// # Note
/// Skipping alpha for AtRGBA.
pub fn AiColorMaxRGB<T: Color>(c: &T) -> f32 {
    c.max()
}

/// Check to see if two colors differ by more than a threhsold.
pub fn AiColorThreshold<T: Color>(c1: &T, c2: &T, t: f32) -> bool {
    c1.threshold(c2, t)
}

/// Convert a RGB color to grey scale (take average of R, G, B)
///
/// # Note
/// Skipping alpha for AtRGBA.
pub fn AiColorToGrey<T: Color>(c: &T) -> f32 {
    c.to_gray()
}

/// Check to see if an RGB color has any corrupted components (nan or infinite).
pub fn AiRGBIsFinite<T: Color>(c1: &T) -> bool {
    c1.is_finite()
}

/// Barycentric interpolation of triangle vertex colors.
pub fn AiBerpRGB<T: Color>(a: f32, b: f32, c0: &T, c1: &T, c2: &T) -> T {
    c0.berp(a, b, c1, c2)
}

/*
AtRGB 	AiBerpRGB (float a, float b, const AtRGB &c0, const AtRGB &c1, const AtRGB &c2)
 	Barycentric interpolation of triangle vertex colors. 
 
AI_API AI_PURE AtRGB 	AiColorHeatMap (const AtRGB *map_colors, const float *map_values, unsigned int map_length, float lookup)
*/

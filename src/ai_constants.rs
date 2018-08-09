#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::f64::EPSILON;

/// pi
pub const AI_PI: f64 = 3.141592653589793;
/// 2 * pi
pub const AI_PITIMES2: f64 = 6.283185307179586;
/// pi / 2
pub const AI_PIOVER2: f64 = 1.5707963267948966;
/// 1 / pi
pub const AI_ONEOVERPI: f64 = 0.3183098861837906;
/// 1 / 2pi
pub const AI_ONEOVER2PI: f64 = 0.1591549430918953;
/// e
pub const AI_E: f64 = 2.718281828459045;
/// log_2(e)
pub const AI_LOG2E: f64 = 1.4426950408889634;
/// ln(2)
pub const AI_LN2: f64 = 0.6931471805599453;
/// sqrt(2)
pub const AI_SQRT2: f64 = 1.4142135623730951;
/// sqrt(3)
pub const AI_SQRT3: f64 = 1.7320508075688772;
/// golden ratio
pub const AI_GOLDEN: f64 = 1.618033988749895;
/// Degrees to Radians
pub const AI_DTOR: f64 = 0.0174532925199433;
/// Radians to Degrees
pub const AI_RTOD: f64 = 57.29577951308232;
/// System epsilon value
pub const AI_EPSILON: f64 = (1.0e-4f64);
/// Lowest possible opacity value
pub const AI_OPACITY_EPSILON: f64 = (1.0e-6f64);
/// Big number
pub const AI_BIG: f64 = (1.0e12f64);
/// Convention for an "infinite" number
pub const AI_INFINITE: f64 = (1.0e30f64);
/// One bit less than 1.0f
pub const AI_ALMOST_ONE: f64 = (1.0f64 - EPSILON / 2.0);

//! AtVector API
//!
//! This module contains vector math types and vector utilities.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use ai_bindings;

use std::f32::{INFINITY, NAN};
use std::ops::{Add, Sub, Mul};

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVector{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

//TODO: Implement Division Trait
impl PartialEq for AtVector {
    fn eq(&self, other: &AtVector) -> bool {
            self.x == other.x
        &&  self.y == other.y
        &&  self.z == other.z
    }
}

impl Add for AtVector {
    type Output = AtVector;
    fn add(self, other: AtVector) -> AtVector {
        AtVector{
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z, 
        }
    }
}

impl<'a, 'b> Sub<&'b AtVector> for &'a AtVector {
    type Output = AtVector;
    fn sub(self, other: &'b AtVector) -> AtVector {
        AtVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for AtVector {
    type Output = AtVector;
    fn mul(self, other: AtVector) -> AtVector {
        AtVector{
            x: self.x * other.x, 
            y: self.y * other.y, 
            z: self.z * other.z, 
        }
    }
}

impl<'a> Mul for &'a AtVector {
    type Output = AtVector;
    fn mul(self, other: &'a AtVector) -> AtVector {
        AtVector{
            x: self.x * other.x, 
            y: self.y * other.y, 
            z: self.z * other.z, 
        }
    }
}

impl<'a> Mul<&'a AtVector> for f32 {
    type Output = AtVector;
    fn mul(self, other: &'a AtVector) -> AtVector {
        AtVector{
            x: self * other.x, 
            y: self * other.y, 
            z: self * other.z, 
        }
    }
}

impl<'a> Mul<f32> for &'a AtVector {
    type Output = AtVector;
    fn mul(self, other: f32) -> AtVector {
        AtVector{
            x: self.x * other, 
            y: self.y * other, 
            z: self.z * other, 
        }
    }
}


impl<'a> Mul<&'a mut AtVector> for f32 {
    type Output = AtVector;
    fn mul(self, other: &'a mut AtVector) -> AtVector {
        AtVector{
            x: self * other.x, 
            y: self * other.y, 
            z: self * other.z, 
        }
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVector2{
    pub x: f32,
    pub y: f32,
}

/// Homogeneous point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtHPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Vector {
    /// Vector Length: ||self||.
    fn length(&self) -> f32;
    /// Dot product: <self, b>
    fn dot(&self, b: &Self) -> f32;
    /// Distance between two points: ||a-b||.
    fn dist(&self, b: &Self) -> f32;
    /// Signed distance between point self and a plane defined by point p and normalized vector n.
    fn dist_plane(&self, p: &Self, n: &Self) -> f32;
    /// Cross product: self x b.
    fn cross(&self, b: &Self) -> Self;
    /// Normalize a vector: self / ||self||. 
    fn normalize(&self) -> Self;
    /// 3D vector linear interpolation (t=0 -> result=self, t=1 -> result=hi)
    fn lerp(&self, t: f32, hi: &Self) -> Self;
    /// Clamp each vector coordinate to the range [self,hi].
    fn clamp(&self, lo: f32, hi: f32) -> Self;
    /// Minimum of self and vector hi, component-wise.
    fn min(&self, b: &Self) -> Self;
    /// Maximum of self and vector b, component-wise.
    fn max(&self, b: &Self) -> Self;
    /// Absolute value of each component.
    fn abs(&self) -> Self;
    /// Element-wise max.
    fn max_element(&self) -> f32;
    /// Element-wise min.
    fn min_element(&self) -> f32;
    /// Barycentric interpolation of a point inside a triangle.
    fn berp_xyz(&self, a: f32, b: f32, p1: &Self, p2: &Self) -> Self;
    /// Check whether a vector has all valid components (not NaN and not infinite)
    fn is_finite(&self) -> bool;
    /// Check for a zero vector, within a small tolerance: ||self|| < epsilon.
    fn is_small(&self, epsilon: f32) -> bool;
    /// Rotate vector a so that it aligns with frame {u,v,w}.
    fn rotate_to_frame(&mut self, u: &Self, v: &Self, v: &Self); // Should mut self
}

impl Vector for AtVector {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn dot(&self, b: &Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    fn dist(&self, b: &Self) -> f32 {
        (b - self).length()
    }

    fn dist_plane(&self, p: &Self, n: &Self) -> f32 {
        self.dot(n) - p.dot(n)
    }

    fn cross(&self, b: &AtVector) -> Self {
        Self {
            x: self.y * b.z - self.z * b.y, 
            y: self.z * b.x - self.x * b.z, 
            z: self.x * b.y - self.y * b.x
        }
    }

    fn normalize(&self) -> Self {
        let mut temp = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if temp != 0.0{
            temp = 1.0 / temp;
        }

        Self {
            x: self.x * temp, 
            y: self.y * temp, 
            z: self.z * temp
        }
    }

    fn lerp(&self, t: f32, hi: &Self) -> Self{
        Self {
            x: self.x * t + (hi.x*1.0-t), 
            y: self.y * t + (hi.y*1.0-t),
            z: self.z * t + (hi.z*1.0-t)
        }
    }

    fn clamp(&self, lo: f32, hi: f32) -> Self{
        Self {
            x: clamp(self.x, lo, hi),
            y: clamp(self.y, lo, hi),
            z: clamp(self.z, lo, hi),
        }
    }

    fn min(&self, b: &Self) -> Self{
        Self {
            x: self.x.min(b.x),
            y: self.y.min(b.y),
            z: self.z.min(b.z),
        }
    }

    fn max(&self, b: &Self) -> Self{
        Self {
            x: self.x.max(b.x),
            y: self.y.max(b.y),
            z: self.z.max(b.z),
        }
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn max_element(&self) -> f32{
        self.x.max(self.y).max(self.z)
    }

    fn min_element(&self) -> f32{
        self.x.min(self.y).min(self.z)
    }

    fn berp_xyz(&self, a: f32, b: f32, p1: &Self, p2: &Self) -> Self{
        let c: f32 = 1.0 - (a + b);
        c*self + a*p1 + b*p2
    }

    fn is_finite(&self) -> bool {
        !(self.x.abs() == INFINITY
            || self.x.abs() == NAN
            || self.y.abs() == INFINITY
            || self.y.abs() == NAN
            || self.z.abs() == INFINITY
            || self.z.abs() == NAN)
    }

    fn is_small(&self, epsilon: f32) -> bool {
        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }

    fn rotate_to_frame(&mut self, u: &Self, v: &Self, w: &Self){
        let tmp = u * self.x + v * self.y + w * self.z;
        self.x = tmp.x;
        self.y = tmp.y;
        self.z = tmp.z;
    }
}

/// Vector Length: ||a||.
pub fn AiV3Length<T: Vector>(a: &T) -> f32 {
    a.length()
}

/// Dot product: <a, b>
pub fn AiV3Dot<T: Vector>(a: &T, b: &T) -> f32 {
    a.dot(b)
}

/// Distance between two points: ||a-b||.
pub fn AiV3Dist<T: Vector + Sub>(a: &AtVector, b: &AtVector) -> f32 {
    (b - a).length()
}

/// Squared distance between two points: ||a-b||^2.
pub fn AiV3Dist2<T: Vector + Sub>(a: &AtVector, b: &AtVector) -> f32 {
    (b - a).length().powf(2.0)
}

/// Signed distance between point x and a plane defined by point p and normalized vector n.
pub fn AiV3DistPlane<T: Vector>(x: &AtVector, p: &AtVector, n: &AtVector) -> f32 {
    x.dist_plane(p, n)
}

/// Cross product: a x b.
pub fn AiV3Cross(a: &AtVector, b: &AtVector) -> AtVector {
    a.cross(b)
}

/// Normalize a vector: a / ||a||. 
pub fn AiV3Normalize(a: &AtVector) -> AtVector {
    a.normalize()
}

/// 3D vector linear interpolation (t=0 -> result=lo, t=1 -> result=hi) 
pub fn AiV3Lerp(t: f32, lo: &AtVector, hi: &AtVector) -> AtVector {
    lo.lerp(t, hi)
}

/// Clamp each vector coordinate to the range [lo,hi]. 
pub fn AiV3Clamp(a: &AtVector, lo: f32, hi: f32) -> AtVector {
    a.clamp(lo, hi)
}

/// Minimum of two vectors, component-wise.
pub fn AiV3Min(a: &AtVector, b: &AtVector) -> AtVector {
    a.min(b)
}

/// Maximum of two vectors, component-wise. 
pub fn AiV3Max(a: &AtVector, b: &AtVector) -> AtVector {
    a.max(b)
}

/// Absolute value of each component. 
pub fn ABS(a: &AtVector) -> AtVector {
    a.abs()
}

/// Element-wise max. 
pub fn AiMaxElement<T: Vector>(a: &T) -> f32 {
    a.max_element()
}

/// Element-wise min. 
pub fn AiMinElement<T: Vector>(a: &T) -> f32 {
    a.min_element()
}

/// Check whether a vector has all valid components (not NaN and not infinite)
pub fn AiV3IsFinite<T: Vector>(a: &T) -> bool {
    a.is_finite()
}

/// Check for a zero vector, within a small tolerance: ||a|| < epsilon. 
pub fn AiV3IsSmall<T: Vector>(a: &T, epsilon: f32) -> bool {
    a.is_small(epsilon)
}

/// Barycentric interpolation of a point inside a triangle. 
pub fn AiBerpXYZ<T: Vector>(a: f32, b: f32, c0: &T, c1: &T, c2: &T) -> T {
    c0.berp_xyz(a, b, c1, c2)
}

/// Rotate vector a so that it aligns with frame {u,v,w}. 
pub fn AiV3RotateToFrame<T: Vector>(a: &mut T, u: &T, v: &T, w: &T) {
    a.rotate_to_frame(u, v, w);
}

/// Barycentric interpolation of UV coordinates inside a 3D triangle.
pub fn AiBerpUV(a: f32, b: f32, u0: f32, v0: f32, u1: f32, v1: f32, u2: f32, v2: f32, u: &mut f32, v: &mut f32) {
    let c : f32 = 1.0 - (a + b);
    *u = c * u0 + a * u1 + b * u2;
    *v = c * v0 + a * v1 + b * v2;
}

/// Build an orthonormal basis aligned with vector N (Frisvad's method).
/// 
/// This is Frisvad's method of building a local reference frame (U,V,W), where W = N. This method is discontinuous at the Z = 0 plane.
/// 
/// # Parameters
///  * `[out] U` - normalized U basis vector
///  * `[out] V` - normalized V basis vector
///  * `N` - normalized vector that will serve as our W basis vector (usually this is a surface normal)
/*
pub fn AiV3BuildLocalFrame(U: &mut AtVector, V: &mut AtVector, N: &AtVector){
    unsafe{ ai_bindings::AiV3BuildLocalFrame(U, V, N) }
}
*/

/// Build an orthonormal basis aligned with vector N (polar method).
/// 
/// Builds local reference frame (U,V,W), where W = N. Uses the parametric tangent vectors in polar coordinates. This is continuous all across the sphere but at the poles.
/// 
/// # Parameters
///  * `[out] U` - normalized U basis vector
///  * `[out] V` - normalized V basis vector
///  * `N` - normalized vector that will serve as our W basis vector (usually this is a surface normal)
/*
pub fn AiV3BuildLocalFramePolar(U: &mut AtVector, V: &mut AtVector, N: &AtVector){
    unsafe{ ai_bindings::AiV3BuildLocalFramePolar(U, V, N) }
}
*/

/// Create a 4D point: pout = (v.x, v.y, v.z, 1) 
pub fn AiV4CreatePoint (v: &AtVector) -> AtHPoint {
    AtHPoint{
        x: v.x,
        y: v.y,
        z: v.z,
        w: 1.0
    }
}

/// Create a 4D vector: vout = (v.x, v.y, v.z, 0)
pub fn AiV4CreateVector (v: &AtVector) -> AtHPoint {
    AtHPoint{
        x: v.x,
        y: v.y,
        z: v.z,
        w: 0.0
    }
}

/// Add two vectors: vout = v1 + v2. 
pub fn AiV4Add(vout: &mut AtHPoint, v1: &AtHPoint, v2: &AtHPoint){
    vout.x = v1.x+v2.x;
    vout.y = v1.y+v2.y;
    vout.z = v1.z+v2.z;
}

/// Substract two vectors: vout = v1 - v2. 
pub fn AiV4Sub(vout: &mut AtHPoint, v1: &AtHPoint, v2: &AtHPoint){
    vout.x = v1.x-v2.x;
    vout.y = v1.y-v2.y;
    vout.z = v1.z-v2.z;
}

/// Scale a vector by a constant: vout = vin * k. 
pub fn AiV4Scale(vout: &mut AtHPoint, vin: &AtHPoint, k: f32){
    vout.x = vin.x*k;
    vout.y = vin.y*k;
    vout.z = vin.z*k;
    vout.w = vin.w*k;
}

/// Negate a vector: vout = -vin. 
pub fn AiV4Neg(vout: &mut AtHPoint, vin: &AtHPoint){
    vout.x = -vin.x;
    vout.y = -vin.y;
    vout.z = -vin.z;
    vout.w = -vin.w;
}

/// Project a homogeneous vector back into 3d: vout = vin.w != 0 ? vin * (1 / vin.w) : (0,0,0) 
pub fn AiV4Project(vout: &mut AtVector, vin: &AtHPoint){
    let mut tmp = AI_V3_ZERO;
    if vin.w != 0.0 {
        tmp = AtVector{x: vin.x / vin.w, y: vin.y / vin.w, z: vin.z / vin.w};
    }
    vout.x = tmp.x;
    vout.y = tmp.y;
    vout.z = tmp.z;
    
}


pub const AI_P3_ZERO  :AtVector = AtVector{x: 0.0, y: 0.0, z: 0.0};
pub const AI_V3_ZERO  :AtVector = AtVector{x: 0.0, y: 0.0, z: 0.0};
pub const AI_V3_HALF  :AtVector = AtVector{x: 0.5, y: 0.5, z: 0.5};
pub const AI_V3_ONE   :AtVector = AtVector{x: 1.0, y: 1.0, z: 1.0};
pub const AI_V3_X     :AtVector = AtVector{x: 1.0, y: 0.0, z: 0.0};
pub const AI_V3_Y     :AtVector = AtVector{x: 0.0, y: 1.0, z: 0.0};
pub const AI_V3_Z     :AtVector = AtVector{x: 0.0, y: 0.0, z: 1.0};
pub const AI_V3_NEGX  :AtVector = AtVector{x:-1.0, y: 0.0, z: 0.0};
pub const AI_V3_NEGY  :AtVector = AtVector{x: 0.0, y: 1.0, z: 0.0};
pub const AI_V3_NEGZ  :AtVector = AtVector{x: 0.0, y: 0.0, z: 1.0};
pub const AI_P2_ZERO  :AtVector2 = AtVector2{x: 0.0, y: 0.0};
pub const AI_P2_ONE   :AtVector2 = AtVector2{x: 1.0, y: 1.0};

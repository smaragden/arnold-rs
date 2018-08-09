//! AtVector API
//!
//! This module contains vector math types and vector utilities.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use ai_bindings::AtHPoint;
pub use ai_bindings::AtVector;
pub use ai_bindings::AtVector2;

use std::ops::Sub;

impl Sub for AtVector {
    type Output = AtVector;
    fn sub(self, other: AtVector) -> AtVector {
        AtVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub trait Vector {
    /// Vector Length: ||a||.
    fn length(&self) -> f32;
    /// Dot product: <a, b>
    fn dot(&self, b: &Self) -> f32;
    fn dist(&self, b: &Self) -> f32;
    //fn dist_plane(&self, p: &Self, n: &Self) -> f32;
    //fn cross(&self, b: &Self) -> Self;
    //fn normalize(&self) -> Self;
    //fn lerp(&self, t: f32, hi: &Self) -> Self;
    //fn clamp(&self, lo: f32, hi: f32) -> Self;
    //fn min(&self) -> Self;
    //fn max(&self) -> Self;
    //fn abs(&self) -> Self;
    //fn max_element(&self) -> f32;
    //fn min_element(&self) -> f32;
    //fn berp_xyz(&self, a: f32, b: f32, p1: &Self, p2: &Self) -> Self;
    //fn is_finite(&self) -> bool;
    //fn is_small(&self, epsilon: f32) -> bool;
    //fn rotate_to_frame(u: &Self, v: &Self, W: &Self); // Should mut this
    //fn berp_uv(a: f32, b: f32, u0: f32, v0: f32, u1: f32, v1: f32, u2: f32, v2: f32, u: *mut f32, v: *mut f32);
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
}

/// Vector Length: ||a||.
pub fn AiV3Length(a: &AtVector) -> f32 {
    a.length()
}

/// Dot product: <a, b>
pub fn AiV3Dot(a: &AtVector, b: &AtVector) -> f32 {
    a.dot(b)
}

/*
AI_DEVICE float 	AiV3Length (const AtVector &a)
 	Vector Length: ||a||. 
 
AI_DEVICE constexpr float 	AiV3Dot (const AtVector &a, const AtVector &b)
 	Dot product: <a, b> 
 
AI_DEVICE float 	AiV3Dist (const AtVector &a, const AtVector &b)
 	Distance between two points: ||a-b||. 
 
AI_DEVICE constexpr float 	AiV3Dist2 (const AtVector &a, const AtVector &b)
 	Squared distance between two points: ||a-b||^2. 
 
AI_DEVICE constexpr float 	AiV3DistPlane (const AtVector &x, const AtVector &p, const AtVector &n)
 	Signed distance between point x and a plane defined by point p and normalized vector n. 
 
AI_DEVICE constexpr AtVector 	AiV3Cross (const AtVector &a, const AtVector &b)
 	Cross product: a x b. 
 
AI_DEVICE AtVector 	AiV3Normalize (const AtVector &a)
 	Normalize a vector: a / ||a||. 
 
AI_DEVICE constexpr AtVector 	AiV3Lerp (float t, const AtVector &lo, const AtVector &hi)
 	3D vector linear interpolation (t=0 -> result=lo, t=1 -> result=hi) 
 
AI_DEVICE constexpr AtVector 	AiV3Clamp (const AtVector &in, float lo, float hi)
 	Clamp each vector coordinate to the range [lo,hi]. 
 
AI_DEVICE constexpr AtVector 	AiV3Min (const AtVector &a, const AtVector &b)
 	Minimum of two vectors, component-wise. 
 
AI_DEVICE constexpr AtVector 	AiV3Max (const AtVector &a, const AtVector &b)
 	Maximum of two vectors, component-wise. 
 
AI_DEVICE AtVector 	ABS (const AtVector &a)
 	Absolute value of each component. 
 
AI_DEVICE float 	AiMaxElement (const AtVector &a)
 	Element-wise max. 
 
AI_DEVICE float 	AiMinElement (const AtVector &a)
 	Element-wise min. 
 
AI_DEVICE AtVector 	AiBerpXYZ (float a, float b, const AtVector &p0, const AtVector &p1, const AtVector &p2)
 	Barycentric interpolation of a point inside a triangle. 
 
AI_API AI_DEVICE AI_PURE bool 	AiV3IsFinite (const AtVector &a)
 	Check whether a vector has all valid components (not NaN and not infinite) 
 
AI_DEVICE bool 	AiV3IsSmall (const AtVector &a, float epsilon=AI_EPSILON)
 	Check for a zero vector, within a small tolerance: ||a|| < epsilon. 
 
AI_DEVICE void 	AiV3RotateToFrame (AtVector &a, const AtVector &u, const AtVector &v, const AtVector &w)
 	Rotate vector a so that it aligns with frame {u,v,w}. 
 
AI_DEVICE void 	AiBerpUV (float a, float b, float u0, float v0, float u1, float v1, float u2, float v2, float *u, float *v)
 	Barycentric interpolation of UV coordinates inside a 3D triangle. 
*/

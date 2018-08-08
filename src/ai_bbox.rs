//! AtBBox API
//!
//! 3D axis-aligned bounding box (uses single-precision)

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
//use ai_bindings::AtVector;

pub use ai_bindings::AtBBox;

/* TODO: Need to implement Vector first
impl AtBBox {
    pub fn from_vectors(p0: *const AtVector, p1: *const AtVector, p2: *const AtVector) -> Self {
        let min = p0;
        let max = p0;
        let min = AiV3Min(min, p1);
        let max = AiV3Max(max, p1);
        let min = AiV3Min(min, p2);
        let max = AiV3Max(max, p2);
        unsafe { AtBBox { min: min, max: max } }
    }
}
*/
/// Compute the "union" of two bboxes.
///
/// # Note
/// this name is misleading since it's the bbox of bboxes, not the union (which can be disjoint)
pub fn AiBBoxUnion(b1: *const AtBBox, b2: *const AtBBox) -> AtBBox {
    unsafe { ai_bindings::AiBBoxUnion_NOINLINE(b1, b2) }
}

pub fn AiBBoxIntersection(b1: *const AtBBox, b2: *const AtBBox) -> AtBBox {
    unsafe { ai_bindings::AiBBoxIntersection_NOINLINE(b1, b2) }
}

pub fn AiBBoxLerp(k: f32, lo: *const AtBBox, hi: *const AtBBox) -> AtBBox {
    unsafe { ai_bindings::AiBBoxLerp_NOINLINE(k, lo, hi) }
}

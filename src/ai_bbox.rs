//! AtBBox API
//!
//! 3D axis-aligned bounding box (uses single-precision)

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
use ai_vector::{AtVector, AiV3Min, AiV3Max};

/// 3D axis-aligned bounding box (uses single-precision)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBBox {
    pub min: AtVector,
    pub max: AtVector,
}

impl AtBBox {
    pub fn from_vectors(p0: &AtVector, p1: &AtVector, p2: &AtVector) -> Self {
        let min = p0;
        let max = p0;
        let min = AiV3Min(min, p1);
        let max = AiV3Max(max, p1);
        let min = AiV3Min(&min, p2);
        let max = AiV3Max(&max, p2);
        AtBBox { min: min, max: max }
    }
}

/// 2D axis-aligned bounding box (uses integers)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBBox2 {
    pub minx: ::std::os::raw::c_int,
    pub miny: ::std::os::raw::c_int,
    pub maxx: ::std::os::raw::c_int,
    pub maxy: ::std::os::raw::c_int,
}

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

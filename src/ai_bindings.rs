//! Generated bindings to Arnold C API

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

use ai_bbox::{AtBBox, AtBBox2}; // reimplemented in ai_vector
use ai_vector::{AtVector, AtVector2}; // reimplemented in ai_vector
include!(concat!(env!("OUT_DIR"), "/arnold_bindings.rs"));

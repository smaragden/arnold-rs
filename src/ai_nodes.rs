#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

use ai_bindings;
use ai_string::AtString;

fn AiNode(nentry_name: &str, name: &str, parent: *const ai_bindings::AtNode) -> *mut ai_bindings::AtNode{
    unsafe{
        return ai_bindings::AiNode(AtString::from(nentry_name), AtString::from(name), parent);
    }
}
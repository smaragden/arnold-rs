#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

use ai_bindings;

use std::ffi::{CString};

fn AiASSWrite(filename: &str, mask: i32, open_procs: bool, binary: bool) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        ai_bindings::AiASSWrite(filename.as_ptr(), mask, open_procs, binary)
    }
}

fn AiASSWriteWithMetadata(filename: &str, mask: i32, open_procs: bool, binary: bool, mds: *const ai_bindings::AtMetadataStore) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        ai_bindings::AiASSWriteWithMetadata(filename.as_ptr(), mask, open_procs, binary, mds)
    }
}
    

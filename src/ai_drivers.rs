#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;

use std::ffi::{CStr, CString};
use std::ptr;

/// Driver's driver_extension method declaration.
///
/// # Returns
/// a NULL-terminated array of filename extensions which this driver is capable of writing. For example, a 'jpeg' driver might return the following array: { "jpeg", "jpg", NULL }
pub fn AiDriverExtension(node_entry: *const ai_bindings::AtNodeEntry) -> Vec<&'static str> {
    let char_ptr = unsafe { ai_bindings::AiDriverExtension(node_entry) };
    let mut p = char_ptr;
    let mut vec = Vec::new();
    unsafe {
        while *p != ptr::null() {
            let s = CStr::from_ptr(*p);
            vec.push(s.to_str().unwrap());
            p = p.offset(1);
        }
    }
    vec
}

/// Get correct driver node type from an extension.
///
/// # Parameters
/// * `extension` - a string containing the file extension, e.g. "tiff"
/// # Returns
/// the [`AtNodeEntry`](ai_bindings::AtNodeEntry) corresponding to extension
pub fn AiFindDriverType(extension: &str) -> *const ai_bindings::AtNodeEntry {
    unsafe { ai_bindings::AiFindDriverType(CString::new(extension).unwrap().as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_msg::{AiMsgSetConsoleFlags, AI_LOG_NONE};
    use ai_render::{AiBegin, AiEnd, AI_SESSION_BATCH};
    use ai_node_entry::{AiNodeEntryLookUp, AiNodeEntryGetNameAtString};
    use ai_string::AtString;
    #[test]
    fn driver_extensions() {
        AiBegin(AI_SESSION_BATCH);
        AiMsgSetConsoleFlags(AI_LOG_NONE);
        let node_entry = AiNodeEntryLookUp(AtString::from("driver_exr"));
        let ext = AiDriverExtension(node_entry);
        assert_eq!(&["exr"], &ext[..]);
        println!("extension: {:?}", ext);
        AiEnd();
    }
    #[test]
    fn find_driver() {
        AiBegin(AI_SESSION_BATCH);
        AiMsgSetConsoleFlags(AI_LOG_NONE);
        let node_entry = AiFindDriverType("exr");
        assert_eq!("driver_exr", AiNodeEntryGetNameAtString(node_entry).to_str());
        println!("extension: {}", AiNodeEntryGetNameAtString(node_entry));
        AiEnd();
    }
}
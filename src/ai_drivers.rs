#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
/// Driver's driver_extension method declaration.
///
/// # Returns
/// a NULL-terminated array of filename extensions which this driver is capable of writing. For example, a 'jpeg' driver might return the following array: { "jpeg", "jpg", NULL }
pub fn AiDriverExtension(node_entry: *const ai_bindings::AtNodeEntry) -> &'static str {
    let char_ptr = unsafe { ai_bindings::AiDriverExtension(node_entry) };
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    c_str.to_str()
}

/// Get correct driver node type from an extension.
///
/// # Parameters
/// * `extension` - a string containing the file extension, e.g. "tiff"
/// # Returns
/// the [`AtNodeEntry`](ai_bindings::AtNodeEntry) corresponding to extension
pub fn AiFindDriverType(extension: &str) -> *const ai_bindings::AtNodeEntry {
    unsafe { ai_bindings::AiFindDriverType(extension.into()) }
}

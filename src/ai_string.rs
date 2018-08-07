#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CStr;
use std::fmt;
#[allow(dead_code)]
use std::ptr;
use std::str;

use ai_bindings::AiAtStringHash;
use ai_bindings::AiAtStringLength;
use ai_bindings::AiCreateAtStringData_private;
pub use ai_bindings::AtString;

impl AtString {
    pub fn length(&self) -> usize {
        unsafe { AiAtStringLength(self.data) }
    }

    pub fn empty(&self) -> bool {
        self.length() == 0
    }

    pub fn hash(&self) -> usize {
        unsafe { AiAtStringHash(self.data) }
    }
}

impl<'a> From<&'a str> for AtString {
    fn from(s: &'a str) -> Self {
        if s.len() == 0 {
            AtString { data: ptr::null() }
        } else {
            AtString {
                data: unsafe { AiCreateAtStringData_private(s.to_owned().as_ptr() as *const i8) },
            }
        }
    }
}

impl From<String> for AtString {
    fn from(s: String) -> Self {
        if s.len() == 0 {
            AtString { data: ptr::null() }
        } else {
            AtString {
                data: unsafe { AiCreateAtStringData_private(s.as_ptr() as *const i8) },
            }
        }
    }
}

impl fmt::Display for AtString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let slice = CStr::from_ptr(self.data);
            write!(f, "{}", str::from_utf8(slice.to_bytes()).unwrap())
        }
    }
}

impl PartialEq for AtString {
    fn eq(&self, other: &AtString) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_msg::{AiMsgSetConsoleFlags, AI_LOG_NONE};
    use ai_render::{AiBegin, AiEnd, AI_SESSION_BATCH};
    #[test]
    fn string_length() {
        AiBegin(AI_SESSION_BATCH);
        AiMsgSetConsoleFlags(AI_LOG_NONE);
        let rust_str = "TEST";
        let rust_string = rust_str.to_string();
        let at_string_str = AtString::from(rust_str);
        let at_string_string = AtString::from(rust_string);
        assert_eq!(at_string_str.length(), 4);
        assert_eq!(at_string_string.length(), 4);
        AiEnd();
    }
    #[test]
    fn string_cmp() {
        AiBegin(AI_SESSION_BATCH);
        AiMsgSetConsoleFlags(AI_LOG_NONE);
        let rust_str = "TEST";
        let rust_string = rust_str.to_string();
        let at_string_str = AtString::from(rust_str);
        let at_string_string = AtString::from(rust_string);
        assert_eq!(at_string_str, at_string_string);
        assert_eq!(AtString::from("åäö"), AtString::from("åäö"));
        AiEnd();
    }
    #[test]
    fn string_empty() {
        AiBegin(AI_SESSION_BATCH);
        AiMsgSetConsoleFlags(AI_LOG_NONE);
        let rust_str = "";
        let at_string_str = AtString::from(rust_str);
        let at_string_string = AtString::from("");
        assert!(at_string_str.empty());
        assert!(at_string_string.empty());
        assert!(AtString::from("").empty());
        assert!(AtString::from(format!("")).empty());
        AiEnd();
    }
    #[test]
    fn string_hash() {
        AiBegin(AI_SESSION_BATCH);
        AiMsgSetConsoleFlags(AI_LOG_NONE);
        assert_eq!(AtString::from("åäö").hash(), 6401777111767391186);
        AiEnd();
    }
}

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;

use std::ffi::{CString};

pub use ai_bindings::AI_LOG_NONE;

pub fn AiMsgInfo(msg: &str){
    unsafe {
        ai_bindings::AiMsgInfo(CString::new(msg).unwrap().as_ptr());
    }
}

pub fn AiMsgDebug(msg: &str){
    unsafe {
        ai_bindings::AiMsgDebug(CString::new(msg).unwrap().as_ptr());
    }
}

pub fn AiMsgWarning(msg: &str){
    unsafe {
        ai_bindings::AiMsgWarning(CString::new(msg).unwrap().as_ptr());
    }
}

pub fn AiMsgError(msg: &str){
    unsafe {
        ai_bindings::AiMsgError(CString::new(msg).unwrap().as_ptr());
    }
}

pub fn AiMsgFatal(msg: &str){
    unsafe {
        ai_bindings::AiMsgFatal(CString::new(msg).unwrap().as_ptr());
    }
}

pub fn AiMsgSetConsoleFlags(flags: i32){
    unsafe{
        ai_bindings::AiMsgSetConsoleFlags(flags)
    }
}
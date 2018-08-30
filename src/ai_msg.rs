#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;

use std::ffi::{CString};

pub const AI_SEVERITY_INFO: u32 = 0;
pub const AI_SEVERITY_WARNING: u32 = 1;
pub const AI_SEVERITY_ERROR: u32 = 2;
pub const AI_SEVERITY_FATAL: u32 = 3;

pub const AI_LOG_NONE: i32 = 0;
pub const AI_LOG_INFO: i32 = 1;
pub const AI_LOG_WARNINGS: i32 = 2;
pub const AI_LOG_ERRORS: i32 = 4;
pub const AI_LOG_DEBUG: i32 = 8;
pub const AI_LOG_STATS: i32 = 16;
pub const AI_LOG_ASS_PARSE: i32 = 32;
pub const AI_LOG_PLUGINS: i32 = 64;
pub const AI_LOG_PROGRESS: i32 = 128;
pub const AI_LOG_NAN: i32 = 256;
pub const AI_LOG_TIMESTAMP: i32 = 512;
pub const AI_LOG_BACKTRACE: i32 = 1024;
pub const AI_LOG_MEMORY: i32 = 2048;
pub const AI_LOG_COLOR: i32 = 4096;
pub const AI_LOG_ALL: i32 = 8191;

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
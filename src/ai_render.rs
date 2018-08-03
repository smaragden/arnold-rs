#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

use ai_bindings;

use std::ptr;
use std::os::raw::c_void;


use ai_bindings::AtSessionMode;
pub use ai_bindings::AtSessionMode::AI_SESSION_BATCH;
pub use ai_bindings::AtSessionMode::AI_SESSION_INTERACTIVE;

use ai_bindings::AtRenderMode;
pub use ai_bindings::AtRenderMode::AI_RENDER_MODE_CAMERA;
pub use ai_bindings::AtRenderMode::AI_RENDER_MODE_FREE;

use ai_bindings::AtRenderErrorCode;
use ai_bindings::AtRenderUpdateCallback;

use ai_bindings::AtBlockingCall;
pub use ai_bindings::AtBlockingCall::AI_NON_BLOCKING;
pub use ai_bindings::AtBlockingCall::AI_BLOCKING;

pub fn AiBegin(mode: AtSessionMode){
    unsafe{
        ai_bindings::AiBegin(mode)
    }
}

pub fn AiEnd(){
    unsafe{
        ai_bindings::AiEnd()
    }
}

pub fn AiGetSessionMode() -> AtSessionMode {
    unsafe{ 
        ai_bindings::AiGetSessionMode() 
    }
}

pub fn AiRenderBegin(mode: AtRenderMode, update_callback: AtRenderUpdateCallback, _callback_private_data: *mut c_void) -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRenderBegin(mode, update_callback, ptr::null_mut())
    }
}

pub fn AiRenderEnd() -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRenderEnd()
    }
}

pub fn AiRenderInterrupt(blocking: AtBlockingCall){
    unsafe {
        ai_bindings::AiRenderInterrupt(blocking)
    }
}

pub fn AiRenderAbort(blocking: AtBlockingCall){
    unsafe {
        ai_bindings::AiRenderAbort(blocking)
    }
}

pub fn AiRender(mode: AtRenderMode) -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRender(mode)
    }
}

//TODO: Reason about extended function signature as we dont have overloading or default arguments.
/*
pub fn AiBeginBatch(){
    unsafe{
        ai_bindings::AiBegin(AI_SESSION_BATCH)
    }
}

pub fn AiBeginInteractive(){
    unsafe{
        ai_bindings::AiBegin(AI_SESSION_INTERACTIVE)
    }
}

pub fn AiRenderBeginCamera() -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRenderBegin(AI_RENDER_MODE_CAMERA, None, ptr::null_mut())
    }
}

pub fn AiRenderBeginFree() -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRenderBegin(AI_RENDER_MODE_FREE, None, ptr::null_mut())
    }
}

pub fn AiRenderInterruptNonBlocking(){
    unsafe {
        ai_bindings::AiRenderInterrupt(AI_NON_BLOCKING)
    }
}

pub fn AiRenderInterruptBlocking(){
    unsafe {
        ai_bindings::AiRenderInterrupt(AI_BLOCKING)
    }
}

pub fn AiRenderAbortNonBlocking(){
    unsafe {
        ai_bindings::AiRenderAbort(AI_NON_BLOCKING)
    }
}
pub fn AiRenderAbortBlocking(){
    unsafe {
        ai_bindings::AiRenderAbort(AI_BLOCKING)
    }
}

pub fn AiRenderCamera() -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRender(AI_RENDER_MODE_CAMERA)
    }
}
pub fn AiRenderFree() -> AtRenderErrorCode {
    unsafe {
        ai_bindings::AiRender(AI_RENDER_MODE_FREE)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn render_testing(){        
        AiBegin(AI_SESSION_BATCH);
        AiRender(AI_RENDER_MODE_FREE);
        AiRenderEnd();
        AiEnd();
    }
}
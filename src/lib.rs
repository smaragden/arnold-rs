//include!("ai_array.rs");
//include!("ai_string.rs");
//include!("ai_render.rs");
#[allow(dead_code)]
mod ai_bindings;

// ai_string
pub use self::ai_string::AtString;
pub mod ai_string;

// ai_render
pub use self::ai_render::AiBegin;
pub use self::ai_render::AiEnd;
pub mod ai_render;

pub mod ai_msg;

//pub use self::ai_render::AiBegin;
//pub use self::ai_render::AiEnd;
//pub mod ai_render;

/*

use std::str;
use std::fmt;
use std::ffi::{CString, CStr};

use std::os::raw::c_char;


pub fn AiMsgInfo(msg: &str){
    unsafe {
        bindings::AiMsgInfo(CString::new(msg).unwrap().as_ptr());
    }
}

fn AiNode(nentry_name: String, name: String, parent: *const bindings::AtNode) -> *mut bindings::AtNode{
    let nentry_name = AtString::from(nentry_name);
    let name = AtString::from(name);
    unsafe{
        return bindings::AiNode(nentry_name, name, parent);
    }
}

fn AiASSWrite(filename: &str, mask: i32, open_procs: bool, binary: bool) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        return bindings::AiASSWrite(filename.as_ptr(), mask as i32, open_procs, binary);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_testing(){
        AiBegin(AI_SESSION_BATCH);
        let sessm = AiGetSessionMode();
        println!("SessionMode: {:?}", sessm);
        let my_name = "TEST";
        let test = AtString::new(my_name);
        println!("After: {:?}", my_name.as_ptr());
        println!("test: {:?}", test);
        let it = unsafe{bindings::AiUniverseGetNodeEntryIterator(bindings::AI_NODE_ALL)};
        while unsafe{!bindings::AiNodeEntryIteratorFinished(it)}{
            let node = unsafe{bindings::AiNodeEntryIteratorGetNext(it)};
            let name = unsafe{bindings::AiNodeEntryGetNameAtString(node)};            
            let new_node = unsafe{bindings::AiNode(name, name, ptr::null_mut())};
            unsafe{println!("node: {} {:?}, nodeis: {} {} |||| {}", name, name, bindings::AiNodeIs(new_node, AtString::from("plane".to_string())), bindings::AiNodeIs(new_node, name), name.hash())};
        }
        unsafe{bindings::AiNode(AtString::from("plane\0".to_string()), AtString::from("plane_test".to_string()), ptr::null_mut())};
        AiASSWrite("/tmp/all.ass", bindings::AI_NODE_ALL as i32, false, false);
        unsafe{bindings::AiNodeEntryIteratorDestroy(it)};
        AiEnd();
    }
}
*/
/*
use raw::AtSessionMode;
use raw::AiBegin;
use raw::AiEnd;
use raw::AiMsgSetConsoleFlags;

fn AiNode(nentry_name: &str, name: &str, parent: *const raw::AtNode) -> *mut raw::AtNode{
    let nentry_name = CString::new(nentry_name).unwrap();
    let name = CString::new(name).unwrap();
    unsafe{
        return raw::AiNode(raw::AtString{data: nentry_name.as_ptr()}, raw::AtString{data: name.as_ptr()}, parent);
    }
}

macro_rules! AiNode {
    ($nentry_name:expr) => {AiNode($nentry_name, "", ptr::null())};
    ($nentry_name:expr, $name:expr)  => {AiNode($nentry_name, $name, ptr::null())};
    ($nentry_name:expr, $name:expr, $parent:expr)  => {AiNode($nentry_name, $name, $parent)};
}


fn AiASSWrite(filename: &str, mask: i32, open_procs: bool, binary: bool) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        return raw::AiASSWrite(filename.as_ptr(), mask as i32, open_procs, binary);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    use std::ffi::CString;

    #[test]
    fn universe_testing(){
        unsafe {
            AiBegin(AtSessionMode::AI_SESSION_INTERACTIVE);
            AiEnd();
        }
    }

    #[test]
    fn ass_write_testing(){
        let filename = CString::new("/tmp/test.ass").unwrap();
        let node_name =  CString::new("box").unwrap();
        unsafe {
            AiMsgSetConsoleFlags(AI_LOG_ALL);
            AiBegin(AtSessionMode::AI_SESSION_INTERACTIVE);
            AiNode!("box", "test");
            AiASSWrite("/tmp/test.ass", AI_NODE_ALL, false, false);
            raw::AiEnd();
        }
    }
}
*/
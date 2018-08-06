#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]

use ai_bindings;
use ai_string::AtString;

use std::str;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

fn AiNode(nentry_name: &str, name: &str, parent: *const ai_bindings::AtNode) -> *mut ai_bindings::AtNode{
    unsafe{
        return ai_bindings::AiNode(AtString::from(nentry_name), AtString::from(name), parent);
    }
}

fn AiNodeDeclare(node: *mut ai_bindings::AtNode, param: &str, declaration: &str) -> bool{
    let declaration = CString::new(declaration).unwrap();
    unsafe {
        ai_bindings::AiNodeDeclare(node, AtString::from(param), declaration.as_ptr())
    }
}

fn AiNodeLookUpUserParameter(node: *mut ai_bindings::AtNode, param: &str) -> *const AtUserParamEntry{
    unsafe {
        ai_bindings::AiNodeLookUpUserParameter(node, AtString::from(param))
    }
}

fn AiNodeIs(node: *mut ai_bindings::AtNode, name: &str) -> bool{
    unsafe {
        ai_bindings::AiNodeIs(node, AtString::from(name))
    }
}

fn AiNodeReset(node: *mut ai_bindings::AtNode){
    unsafe {
        ai_bindings::AiNodeReset(node)
    }
}

fn AiNodeResetParameter(node: *mut ai_bindings::AtNode, param: &str){
    let param = CString::new(param).unwrap();
    unsafe {
        ai_bindings::AiNodeResetParameter(node, param.as_ptr())
    }
}

fn AiNodeClone(node: *const ai_bindings::AtNode, new_name: &str, parent: *const ai_bindings::AtNode) -> *mut ai_bindings::AtNode{
    unsafe {
        ai_bindings::AiNodeClone(node, AtString::from(new_name), parent)
    }
}

fn AiNodeDestroy(node: *mut ai_bindings::AtNode) -> bool{
    unsafe {
        ai_bindings::AiNodeDestroy(node)
    }
}

fn AiNodeReplace(node: *mut ai_bindings::AtNode, new_node: *mut ai_bindings::AtNode, remove: bool){
    unsafe {
        ai_bindings::AiNodeReplace(node, new_node, remove)
    }
}

fn AiNodeLink(node: *mut ai_bindings::AtNode, input: &str, target: *mut ai_bindings::AtNode) -> bool {
    let input = CString::new(input).unwrap();
    unsafe {
        ai_bindings::AiNodeLink(node, input.as_ptr(), target)
    }
}

fn AiNodeLinkOutput(node: *mut ai_bindings::AtNode, output: &str, target: *mut ai_bindings::AtNode, input: &str) -> bool {
    let output = CString::new(output).unwrap();
    let input = CString::new(input).unwrap();
    unsafe {
        ai_bindings::AiNodeLinkOutput(node, output.as_ptr(), target, input.as_ptr())
    }
}

fn AiNodeUnlink(node: *mut ai_bindings::AtNode, input: &str) -> bool {
    let input = CString::new(input).unwrap();
    unsafe {
        ai_bindings::AiNodeUnlink(node, input.as_ptr())
    }
}

fn AiNodeIsLinked(node: *mut ai_bindings::AtNode, input: &str) -> bool {
    let input = CString::new(input).unwrap();
    unsafe {
        ai_bindings::AiNodeIsLinked(node, input.as_ptr())
    }
}

fn AiNodeGetLink(node: *mut ai_bindings::AtNode, input: &str, comp: *mut i32) -> *mut ai_bindings::AtNode {
    let input = CString::new(input).unwrap();
    unsafe {
        ai_bindings::AiNodeGetLink(node, input.as_ptr(), comp)
    }
}

fn AiNodeGetName(node: *mut ai_bindings::AtNode, input: &str, comp: *mut i32) -> &str {    
    let data = unsafe {
        ai_bindings::AiNodeGetName(node)
    };
    let slice = CStr::from_ptr(data);
    str::from_utf8(slice.to_bytes()).unwrap()
}

fn AiNodeGetNodeEntry(node: *mut ai_bindings::AtNode) -> *const ai_bindings::AtNodeEntry {    
    unsafe {
        ai_bindings::AiNodeGetNodeEntry(node)
    }
}

fn AiNodeGetLocalData(node: *const ai_bindings::AtNode) -> *mut c_void {    
    unsafe {
        ai_bindings::AiNodeGetLocalData(node)
    }
}

fn AiNodeSetLocalData(node: *mut ai_bindings::AtNode, data: *mut c_void) {    
    unsafe {
        ai_bindings::AiNodeSetLocalData(node, data)
    }
}

fn AiNodeGetPluginData(node: *const ai_bindings::AtNode) -> *mut c_void {    
    unsafe {
        ai_bindings::AiNodeGetPluginData(node)
    }
}

fn AiNodeSetDisabled(node: *mut ai_bindings::AtNode, disabled: bool){    
    unsafe {
        ai_bindings::AiNodeSetDisabled(node, disabled)
    }
}

fn AiNodeIsDisabled(node: *mut ai_bindings::AtNode) -> bool {    
    unsafe {
        ai_bindings::AiNodeIsDisabled(node)
    }
}

fn AiNodeGetParent(node: *mut ai_bindings::AtNode) -> *mut ai_bindings::AtNode {    
    unsafe {
        ai_bindings::AiNodeGetParent(node)
    }
}

fn AiNodeGetUserParamIterator(node: *mut ai_bindings::AtNode) -> *mut ai_bindings::AtUserParamIterator {    
    unsafe {
        ai_bindings::AiNodeGetUserParamIterator(node)
    }
}
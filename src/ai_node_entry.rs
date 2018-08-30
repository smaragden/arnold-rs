//! AtParamEntry API
//! 
//! For a discussion of Arnold's object-oriented system of pluggable nodes, please refer to [AtNode API](../ai_nodes/index.html).
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
pub use ai_bindings::{
    AtNodeEntry, 
    AtParamEntry, 
    AtParamIterator,
    AtMetaDataIterator,
    AtNodeMethods,
    AtMetaDataEntry
};
use ai_bindings::AtString;

use std::ffi::{CStr, CString};
use std::str;

// Nodes
/// Undefined type. 
pub const AI_NODE_UNDEFINED: u32 = 0;
/// Options node (following the "singleton" pattern, there is only one options node) 
pub const AI_NODE_OPTIONS: u32 = 1;
/// Camera nodes (persp_camera, fisheye_camera, etc) 
pub const AI_NODE_CAMERA: u32 = 2;
/// Light source nodes (spot_light, etc) 
pub const AI_NODE_LIGHT: u32 = 4;
/// Geometry nodes (sphere, polymesh, etc) 
pub const AI_NODE_SHAPE: u32 = 8;
/// Shader nodes (lambert, etc) 
pub const AI_NODE_SHADER: u32 = 16;
/// EXPERIMENTAL: override nodes support "delayed parameter overrides" for procedural nodes. 
pub const AI_NODE_OVERRIDE: u32 = 32;
/// Output driver nodes (driver_tiff, etc) 
pub const AI_NODE_DRIVER: u32 = 64;
/// Pixel sample filter nodes (box_filter, etc. 
pub const AI_NODE_FILTER: u32 = 128;
/// Color manager nodes (Syncolor, OCIO, etc) 
pub const AI_NODE_COLOR_MANAGER: u32 = 2048;
/// Operator plug-in nodes. 
pub const AI_NODE_OPERATOR: u32 = 4096;
/// Bitmask including all node types, used by [AiASSWrite()](../ai_dotass/fn.AiASSWrite.html)
pub const AI_NODE_ALL: u32 = 65535;

// Shapes
pub const AI_NODE_SHAPE_PROCEDURAL: u32 = 256;
pub const AI_NODE_SHAPE_VOLUME: u32 = 512;
pub const AI_NODE_SHAPE_IMPLICIT: u32 = 1024;

/// Look up a node entry from a name string.
/// 
/// Search all installed node entries for a given node name. If found, return a pointer to the node entry.
/// 
/// # Parameters
/// * `name` - input node name (such as "lambert")
/// # Returns
/// pointer to the AtNodeEntry that matches the input name
pub fn AiNodeEntryLookUp(name: AtString) -> *const AtNodeEntry{
    unsafe { ai_bindings::AiNodeEntryLookUp(name) }
}

/// Return the name of the given AtNodeEntry as an AtString.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// AtString name string, or NULL AtString if nentry is NULL
pub fn AiNodeEntryGetNameAtString(nentry: *const AtNodeEntry) -> AtString {
    unsafe {ai_bindings::AiNodeEntryGetNameAtString(nentry)}
}

/// Return the type of the given AtNodeEntry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// type of node (AI_NODE_SHADER, etc), or 0 if nentry is NULL
pub fn AiNodeEntryGetType(nentry: *const AtNodeEntry) -> i32 {
    unsafe {ai_bindings::AiNodeEntryGetType(nentry)}
}

/// Return the type of the given AtNodeEntry as a string.
/// 
/// For example, a node of type AI_NODE_LIGHT would return "light"
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// type string, or NULL if nentry is NULL
pub fn AiNodeEntryGetTypeName(nentry: *const AtNodeEntry) -> &'static str {
    let slice = unsafe {
        let data = ai_bindings::AiNodeEntryGetTypeName(nentry);
        CStr::from_ptr(data)
    };
    str::from_utf8(slice.to_bytes()).unwrap()
}

/// Return the derived type of the given AtNodeEntry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// derived type of node (AI_NODE_SHAPE_VOLUME, etc), or 0 if nentry is either NULL or a non-derived type
pub fn AiNodeEntryGetDerivedType(nentry: *const AtNodeEntry) -> i32 {
    unsafe {ai_bindings::AiNodeEntryGetDerivedType(nentry)}
}

/// Return the derived type of the given AtNodeEntry as a string, if applicable.
/// 
/// For example, a node of type AI_NODE_SHAPE_VOLUME would return "volume"
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// type string, or NULL if nentry is NULL or not a derived type
pub fn AiNodeEntryGetDerivedTypeName(nentry: *const AtNodeEntry) -> &'static str {
    let slice = unsafe {
        let data = ai_bindings::AiNodeEntryGetDerivedTypeName(nentry);
        CStr::from_ptr(data)
    };
    str::from_utf8(slice.to_bytes()).unwrap()
}

/// Return the output type of the given AtNodeEntry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// output type (AI_TYPE_RGB, etc)
pub fn AiNodeEntryGetOutputType(nentry: *const AtNodeEntry) -> i32 {
    unsafe {ai_bindings::AiNodeEntryGetOutputType(nentry)}
}

/// Return the filename (so/dll path) of an AtNodeEntry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// string with the path ot the dynamic library where this node was installed from, NULL if this is a built-in node
pub fn AiNodeEntryGetFilename(nentry: *const AtNodeEntry) -> &'static str {
    let slice = unsafe {
        let data = ai_bindings::AiNodeEntryGetFilename(nentry);
        CStr::from_ptr(data)
    };
    str::from_utf8(slice.to_bytes()).unwrap()
}

/// Return the version that this node was linked with.
/// 
/// There are restrictions as to what previously-compiled plug-in nodes can function with a given Arnold library. For example, a plug-in node compiled and linked with Arnold 3.1.x can't be loaded (installed) on Arnold 3.2.x.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// string containing the Arnold version that this node was linked with.
pub fn AiNodeEntryGetVersion(nentry: *const AtNodeEntry) -> &'static str {
    let slice = unsafe {
        let data = ai_bindings::AiNodeEntryGetVersion(nentry);
        CStr::from_ptr(data)
    };
    str::from_utf8(slice.to_bytes()).unwrap()
}

/// Return the number of instances of a particular node.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// number of nodes of this type that have been created so far (for example, how many "sphere" nodes exist in memory), or 0 if nentry is NULL
pub fn AiNodeEntryGetCount(nentry: *const AtNodeEntry) -> i32 {
    unsafe {ai_bindings::AiNodeEntryGetCount(nentry)}
}

/// Return the number of parameters of a given AtNodeEntry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// # Returns
/// number of parameters in the node, or 0 if nentry is NULL
pub fn AiNodeEntryGetNumParams(nentry: *const AtNodeEntry) -> i32 {
    unsafe {ai_bindings::AiNodeEntryGetNumParams(nentry)}
}

/// Return the i'th parameter in an AtNodeEntry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// * `i` - parameter index, in 0..num_params-1
/// # Returns
/// the i'th parameter entry in this node
pub fn AiNodeEntryGetParameter(nentry: *const AtNodeEntry, i: i32) -> *const AtParamEntry {
    unsafe {ai_bindings::AiNodeEntryGetParameter(nentry, i)}
}

/// Look up a parameter in a node entry from a name string.
/// 
/// Searches the parameter entries of a given node looking for a parameter that matches the name string. If found, returns a pointer to the parameter entry.
/// 
/// # Parameters
/// * `nentry` - input node entry
/// * `name` - parameter name that we are looking for (such as "samples")
/// # Returns
/// pointer to the AtParamEntry that matches the parameter name, or NULL if either nentry is NULL or the parameter is not found
pub fn AiNodeEntryLookUpParameter<T: Into<AtString>>(nentry: *const AtNodeEntry, name: T) -> *const AtParamEntry {
    unsafe {ai_bindings::AiNodeEntryLookUpParameter(nentry, name.into())}
}

/// Creates and returns a new AtParamIterator for this node entry.
/// 
/// # Parameters
/// * `nentry`- node entry whose parameters will be iterated over
/// # Returns
/// an iterator over all built-in parameters on this node entry
pub fn AiNodeEntryGetParamIterator(nentry: *const AtNodeEntry) -> *const AtParamIterator {
    unsafe {ai_bindings::AiNodeEntryGetParamIterator(nentry)}
}

/// Creates and returns a new AtMetaDataIterator for this node entry.
/// 
/// # Parameters
/// * `nentry` - node entry whose metadata will be iterated over
/// * `param`- selects a specific parameter to get its metadata or NULL for node metadata
/// # Returns
/// an iterator over metadata on this node entry
pub fn AiNodeEntryGetMetaDataIterator(nentry: *const AtNodeEntry, param: &str) -> *const AtMetaDataIterator {
    let param = CString::new(param).unwrap();
    unsafe {ai_bindings::AiNodeEntryGetMetaDataIterator(nentry, param.as_ptr())}
}

/// Install a new node in the system.
/// 
/// An Arnold-based app can use this function to extend the renderer with custom types of nodes, such as new shaders or cameras. The user must provide the node type, name, and methods. Successive calls to AiNode() can create nodes of this type.
/// 
/// # Examples:
/// ```
///     AiNodeEntryInstall(AI_NODE_SHAPE, AI_TYPE_UNDEFINED, "sphere", "<built-in>", SphMethods, AI_VERSION);
///     AiNodeEntryInstall(AI_NODE_SHADER, AI_TYPE_FLOAT, "fBm_noise", "./shaders/fBm_noise.so", fBmMethods, AI_VERSION);
/// ```
/// # Parameters
/// * `type` - type of node (AI_NODE_CAMERA, AI_NODE_SHADER, etc)
/// * `output_type` - for shader nodes, which can link their output to the input of another shader, this is the output type (AI_TYPE_RGB, etc)
/// * `name` - name string of the newly created node type
/// * `filename` - string identifier that hints at how this node was created, usually the path of the dynamic library where it was loaded from but, in general, it should be set to whatever makes sense for the app that's installing the node
/// * `methods` - pointer to the node methods
/// * `version` - Arnold version string that this node is linked with
pub fn AiNodeEntryInstall(entry_type: i32, output_type: u8, name: &str, filename: &str, methods: *const AtNodeMethods, version: &str){
    let name = CString::new(name).unwrap();
    let filename = CString::new(filename).unwrap();
    let version = CString::new(version).unwrap();
    unsafe {ai_bindings::AiNodeEntryInstall(entry_type, output_type, name.as_ptr(), filename.as_ptr(), methods, version.as_ptr())}
}

/// Uninstall a node from the system.
/// 
/// Remove the node with the given name from the system. Successive calls to AiNode() using this node name will fail.
/// 
/// # Parameters
/// * `name` - name of the node to be removed from the system
pub fn AiNodeEntryUninstall(name: &str){
    let name = CString::new(name).unwrap();
    unsafe {ai_bindings::AiNodeEntryUninstall(name.as_ptr()) }
}

/// Destroys a param iterator when it is no longer needed.
/// 
/// # Parameters
/// * `iter` - param iterator that will be deallocated
pub fn AiParamIteratorDestroy(iter: *mut AtParamIterator){
    unsafe {ai_bindings::AiParamIteratorDestroy(iter) }
}

/// Returns current param entry and points param iterator to the next one.
/// 
/// This function is designed to be used inside a loop, as illustrated by the following example, which prints all the built-in parameters of a given AtNodeEntry:
/// ```
///     AtParamIterator *iter = AiNodeEntryGetParamIterator(nentry);
///     while (!AiParamIteratorFinished(iter))
///     {
///        AtParamEntry *pentry = AiParamIteratorGetNext(iter);
///        printf("%s\n", AiParamGetName(pentry));
///     }
///     AiParamIteratorDestroy(iter);
/// ```
/// # Parameters
/// * `iter` - a param iterator
/// # Returns
/// the current param entry pointed by the iterator, or NULL if there are no more parameters to iterate over
pub fn AiParamIteratorGetNext(iter: *mut AtParamIterator) -> *const AtParamEntry {
    unsafe {ai_bindings::AiParamIteratorGetNext(iter) }
}

/// Returns true if there are no more parameters to iterate over.
/// 
/// # Parameters
/// * `iter` - a param iterator
/// # Returns
/// true if the param iterator has moved past the last parameter
pub fn AiParamIteratorFinished(iter: *const AtParamIterator) -> bool {
    unsafe {ai_bindings::AiParamIteratorFinished(iter) }
}

/// Destroys a metadata iterator when it is no longer needed.
/// 
/// # Parameters
/// * `iter` - metadata iterator that will be deallocated
pub fn AiMetaDataIteratorDestroy(iter: *mut AtMetaDataIterator){
    unsafe {ai_bindings::AiMetaDataIteratorDestroy(iter) }
}

/// Returns current metadata entry and points metadata iterator to the next one.
/// 
/// This function is designed to be used inside a loop, as illustrated by the following example, which prints all the metadata of a given AtNodeEntry:
/// ```
///     AtMetaDataIterator *iter = AiNodeEntryGetMetaDataIterator(nentry);
///     while (!AiMetaDataIteratorFinished(iter))
///     {
///        const AtMetaDataEntry *entry = AiMetaDataIteratorGetNext(iter);
///        printf("%s\n", entry->name);
///     }
///     AiMetaDataIteratorDestroy(iter);
/// ```
/// # Parameters
/// * `iter` - a metadata iterator
/// # Returns
/// the current metadata entry pointed by the iterator, or NULL if there is no more metadata to iterate over
pub fn AiMetaDataIteratorGetNext(iter: *mut AtMetaDataIterator) -> *const AtMetaDataEntry {
    unsafe {ai_bindings::AiMetaDataIteratorGetNext(iter) }
}

/// Returns true if there is no more metadata to iterate over.
/// 
/// # Parameters
/// * `iter` - a metadata iterator
/// # Returns
/// true if the metadata iterator has moved past the last entry
pub fn AiMetaDataIteratorFinished(iter: *const AtMetaDataIterator) -> bool {
    unsafe {ai_bindings::AiMetaDataIteratorFinished(iter) }
}
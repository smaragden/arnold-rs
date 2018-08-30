//! AtNode API
//!
//! Arnold has a pluggable node-management system for the standard primitives such as lights, cameras, geometry, shaders, filters, drivers, etc.
//!
//! Each node type has a set of associated methods (member functions) and parameters. Like C++'s objects, Arnold's node system allows new nodes to inherit both parameters and methods from the "parent" node. For example, all nodes have a "name" parameter and a "node_initialize{}" method (among others) because they are declared in the base node from which all nodes inherit.
//!
//! In addition to these standard methods and parameters, derived nodes may have their own type-specific parameters and methods. For example, all shaders have a "shader_evaluate{}" method and all driver nodes have a "driver_write_bucket{}" method (among others). User-derived nodes may have their own new parameters but are not permitted to add new methods.
//!
//! Optional "node_plugin_initialize{}" and "node_plugin_cleanup{}" methods can be used for initializing a plugin, if there is some initialization or data to be shared between nodes of the same type. These methods are only called if a node of this type is created. AiNodeGetPluginData() can be used to retrieve the plugin data.
//!
//! Two of the important data-structures in this object-oriented-like system are:
//!
//! **AtNodeEntry** – this contains the "description" of a particular node type ("polymesh", "persp_camera", etc.)  
//! **AtString** – this contains the "instantiation" of a particular node type

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
use ai_string::AtString;
use ai_vector::{AtVector, AtVector2};

use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::{ptr, str};

/// Create a fresh instantiation of a node.
///
/// Arnold-based apps would call this function to populate the scene with nodes. Node instantiations can be created out of built-in nodes (such as "sphere" or "lambert"), manually-installed nodes (using AiNodeEntryInstall()), or plug-in nodes contained in a .so/.dll dynamic library (using AiLoadPlugins()).
///
/// The system automatically destroys all nodes at AiEnd() time prior to shutdown.
///
/// # Note
/// When creating a node that will be contained in a procedural node, it is CRITICAL that the proper parent procedural pointer is given through the "parent" parameter. Failure to do so will result in some issues due to incomplete node initialization.
/// # Parameters
/// * `nentry_name` - name of the type of node to be created ("polymesh", etc)
/// * `name` - name of the new node to be created
/// * `parent` - parent of the new node, or NULL for none (global scope)
/// # Returns
/// pointer to a new [`AtString`](::ai_string::AtString) of the requested type, or NULL if the node could not be created (the node wasn't previously installed, or it was in the ignore list)
/// # Rust
/// To get default behaviour:
/// ```
/// AiNode("flat", "flatShader1", None)
/// ```
pub fn AiNode<T: Into<AtString>>(
    nentry_name: T,
    name: Option<&str>,
    parent: Option<*const ai_bindings::AtNode>,
) -> *mut ai_bindings::AtNode {
    unsafe {
        let name = name.unwrap_or("");
        return ai_bindings::AiNode(
            nentry_name.into(),
            name.into(),
            parent.unwrap_or(ptr::null_mut()),
        );
    }
}

/// Search for a specific node in the scene with the given name string  
///
/// Search for a specific node in the scene with the given name string (which can be a simple name, a relative path or an absolute path).
///
/// Performs a recursive search for the given name string, starting from the contents of the specified parent node, then continuing up towards the root of the scene, until the node is found. If the parent given is null, the search is performed only at the root level.
///
/// The name string could be one a simple name, a relative path (with parent names separated by '^' followed by the node name, or an absolute path, with a similar syntax to relative paths, but starting with '^'). These are some examples:
/// ```
/// AiNodeLookUpByName("myshader", myprocedural);
/// AiNodeLookUpByName("materials^wood^cherry");
/// AiNodeLookUpByName("^myproc^mymesh");
/// ```
/// # Parameters
/// * `name`- node path to look up
/// * `parent` - if not null, search will begin within the contents of this node, then it will continue recursively up towards the root of the scene. If null, search will be limited to nodes in the root level.
/// # Returns
/// pointer to a node whose "name" parameter matches the given name string, or NULL if there are no nodes with that name
/// # Rust
/// To get default behaviour:
/// ```
/// AiNodeLookUpByName("flatShader1", None)
/// ```
pub fn AiNodeLookUpByName<T: Into<AtString>>(
    name: T,
    parent: Option<*const ai_bindings::AtNode>,
) -> *mut ai_bindings::AtNode {
    unsafe { ai_bindings::AiNodeLookUpByName(name.into(), parent.unwrap_or(ptr::null_mut())) }
}

/// Declare a user-defined parameter for this node.
///
/// Adds new parameter of type 'declaration' to the specified node instantiation. declaration is a string of the form "class type", where:
///
/// class := { constant, uniform, varying }
/// type := { BYTE, INT, BOOL, FLOAT, RGB, POINT, STRING, etc. }
/// The scope of the different classes is:
///
/// constant values exist on a per-object basis
/// uniform values exist on a per-face basis
/// varying values exist on a per-vertex basis
/// In the case where the type is ARRAY, then the user should also supply the array type as well.
///
/// In the following example, we are declaring two user-defined parameters, "bar" and "heatmap", for one specific polymesh node whose name is "mymesh":
/// ```
/// AtNode *mymesh = AiNode("polymesh");
/// AiNodeDeclare(mymesh, AtString("bar"), "constant BOOL");
/// AiNodeDeclare(mymesh, AtString("heatmap"), "constant ARRAY RGB");
/// ```
/// # Parameters
/// * `node` - input node
/// * `name` - name of the new parameter
/// * `declaration` - declaration string for the class and type of the new parameter
/// # Returns
/// true if the parameter could be succesfully added: it didn't already exist and the declaration string wasn't malformed
/// # See also
/// User-Data API
pub fn AiNodeDeclare<T: Into<AtString>>(
    node: *mut ai_bindings::AtNode,
    param: T,
    declaration: &str,
) -> bool {
    let declaration = CString::new(declaration).unwrap();
    unsafe { ai_bindings::AiNodeDeclare(node, param.into(), declaration.as_ptr()) }
}

/// Return the user-defined parameter entry that matches a given name.
///
/// This function searches the user-defined parameter entries of a given node looking for a parameter that matches the given string. If found, returns a pointer to the parameter entry.
///
/// # Parameters
/// * `node` - input node
/// * `param` - the parameter we are looking for
/// # Returns
/// handle to the user-defined parameter entry whose name matches the given string, or NULL if not found
/// # See also
/// User-Data API
pub fn AiNodeLookUpUserParameter<T: Into<AtString>>(
    node: *mut ai_bindings::AtNode,
    param: T,
) -> *const ai_bindings::AtUserParamEntry {
    unsafe { ai_bindings::AiNodeLookUpUserParameter(node, param.into()) }
}

/// Compare the node type against a string.
///
/// This is a useful helper function that allows one to write code like:
/// ```
/// if (AiNodeIs(mynode,AtString("lambert")))
/// {
///    // do something exciting that only applies to lambert nodes
/// }
/// ```
/// # Parameters
/// * `node` - pointer to a node whose type is to be compared
/// * `string` - the name of an existing node type
/// # Returns
/// true if the node type's name matches the given string
pub fn AiNodeIs<T: Into<AtString>>(node: *mut ai_bindings::AtNode, name: T) -> bool {
    unsafe { ai_bindings::AiNodeIs(node, name.into()) }
}

/// Reset all node parameters to their default values and remove any input links.
///
/// # Note
/// If the node is a procedural, any parameter that had a explicit value was overriding the values on the children. Resetting these parameters will remove those overrides.
/// # Parameters
/// * `node` - pointer to a node to be reset (if the pointer is NULL, the function will simply return)
pub fn AiNodeReset(node: *mut ai_bindings::AtNode) {
    unsafe { ai_bindings::AiNodeReset(node) }
}

/// Reset a node parameter to its default value and remove any links to that parameter.
///
/// # Note
/// If you reset a declared user parameter, it will be removed and will not exist anymore. You will need to re-declare it later if you wish to use that particular user parameter.
/// If the node is a procedural, this parameter could have been overriding values on the nodes contained in it. This override will disappear once the parameter is reset.
///
/// # Parameters
/// * `node` - pointer to a node (if the pointer is NULL, the function will simply return)
/// * `param` - name of the parameter to be reset to its default value
pub fn AiNodeResetParameter(node: *mut ai_bindings::AtNode, param: &str) {
    let param = CString::new(param).unwrap();
    unsafe { ai_bindings::AiNodeResetParameter(node, param.as_ptr()) }
}

/// Return an exact clone of a source node.
///
/// Note that this performs a "deep" copy of all of the node's parameters, including strings and arrays.
///
/// # Note
/// the value of the "parent" parameter is the same as if the node was being created as new with the AiNode function. A node that will be contained in a procedural should be created with the proper parent, regardless of where the original node is coming from (root level, another procedural or even the same procedural).
/// # Parameters
/// * `node` - the source node to be cloned
/// * `new_name` - the name given to the cloned node
/// * `parent` - parent of the cloned node, or NULL for none (global scope)
/// # Returns
/// a clone of the source node, or NULL if the source was NULL or was non-clonable (like the options node)
pub fn AiNodeClone(
    node: *const ai_bindings::AtNode,
    new_name: &str,
    parent: Option<*const ai_bindings::AtNode>,
) -> *mut ai_bindings::AtNode {
    unsafe {
        ai_bindings::AiNodeClone(
            node,
            AtString::from(new_name),
            parent.unwrap_or(ptr::null_mut()),
        )
    }
}

/// Destroy an existing node.
///
/// This function releases all resources associated with an existing node, including all of its memory, parameters, arrays, etc. Attempts to access a destroyed node will cause undefined behaviour.
///
/// For now, this function will do nothing if called during rendering.
///
/// # Note
/// This function is intended for a single node or very few nodes, created and destroyed during an interactive session. It has an overhead due to memory release and update of data structures, so don't apply this to a large number of nodes, and never to the whole scene. All nodes in the scene are released efficiently during AiEnd().
/// # Parameters
/// * `node` - pointer to an existing node, generally coming from AiNode()
/// # Returns
/// true if the node was destroyed, false otherwise
pub fn AiNodeDestroy(node: *mut ai_bindings::AtNode) -> bool {
    unsafe { ai_bindings::AiNodeDestroy(node) }
}

/// Replace an existing node with another, updating all references to that node.
///
/// For now, this function will do nothing if called during rendering.
///
/// # Parameters
/// * `old_node` - pointer to an existing node, generally coming from AiNode()
/// * `new_node` - pointer to an existing node, generally coming from AiNode()
/// * `remove` - true if the old_node should be destroyed after replacing references
pub fn AiNodeReplace(
    node: *mut ai_bindings::AtNode,
    new_node: *mut ai_bindings::AtNode,
    remove: bool,
) {
    unsafe { ai_bindings::AiNodeReplace(node, new_node, remove) }
}

/// Creates a connection between two shader nodes.
///
/// This is just a convenience function for linking shaders when the whole output of the source shader is used. It maintains the previous API for linking. Everything else is the same as in AiNodeLinkOutput.
///
/// So, this:
/// ```
/// AiNodeLink(source, "parameter", target);
/// ```
/// is equivalent to:
/// ```
/// AiNodeLinkOutput(source, "", target, "parameter");
/// ```
/// # See also
/// AiNodeLinkOutput
/// # Parameters
/// * `src` - a non-NULL pointer to the source node, to be connected to the target node
/// * `input` - the input parameter specification in the target node, which can optionally include a component specification (e.g. "Kd.r", "dir.x"), an array element index (e.g. "colors\[1\]") or both (e.g. "colors\[1\].r").
/// * `target` - a non-NULL pointer to the target node whose input parameter will be connected to the output of the source node
/// # Returns
/// true if the connection was performed successfully, false otherwise
pub fn AiNodeLink(
    src: *mut ai_bindings::AtNode,
    input: &str,
    target: *mut ai_bindings::AtNode,
) -> bool {
    let input = CString::new(input).unwrap();
    unsafe { ai_bindings::AiNodeLink(src, input.as_ptr(), target) }
}

/// Creates a connection between two shader nodes.
///
/// This function is used to build shader networks. It links the output of the source node to one of the input parameters in the target node.
///
/// A specific component can be selected for certain types, both in the source output and the target input, so that the connection would only affect that component (the other components can be linked independently).
///
/// For input parameters of array type, a specific element can be selected, so that the link affects only that element. Same as with components, the other array elements can be linked independently. Additionally, a specific component can be selected for the array element.
///
/// A check is made for "link-compatibility" between the source and the target of the connection, so it cannot be performed if they are not compatible. This includes resolution of array types (when using array linking) and takes into account separate component linking.
///
/// Node parameters can be explicitly flagged as non-linkable by setting the following boolean metadata:
/// ```
/// node_parameters
/// {
///    ...
///    AiMetaDataSetBool(nentry, "parameter_name", "linkable", false);
/// }
/// ```
/// Attempts to use AiNodeLink() / AiNodeLinkOutput() on a parameter that has been flagged as non-linkable will produce a warning but otherwise have no effect.
///
/// # See also
/// AiNodeLink
/// # Parameters
/// * `src` - a non-NULL pointer to the source node, to be connected to the target node
/// * `output` - selects one of the components (e.g "r", "g", ...), or the whole output of the source node (using "")
/// * `target` - a non-NULL pointer to the target node whose input parameter will be connected to the output of the source node
/// * `input` - the input parameter specification in the target node, which can optionally include a component specification (e.g. "Kd.r", "dir.x"), an array element index (e.g. "colors\[1\]") or both (e.g. "colors\[1\].r").
/// # Returns
/// true if the connection was performed successfully, false otherwise
pub fn AiNodeLinkOutput(
    node: *mut ai_bindings::AtNode,
    output: &str,
    target: *mut ai_bindings::AtNode,
    input: &str,
) -> bool {
    let output = CString::new(output).unwrap();
    let input = CString::new(input).unwrap();
    unsafe { ai_bindings::AiNodeLinkOutput(node, output.as_ptr(), target, input.as_ptr()) }
}

/// Removes a connection from a node input parameter.
///
/// Using this function you can disconnect, or unlink, a shader input that has been previously linked. If the specified parameter wasn't already linked, this function won't do anything.
///
/// # Parameters
/// * `node` - a non-NULL pointer to a node
/// * `input` - the input parameter specification in the target node, which can optionally include a component specification (e.g. "Kd.r", "dir.x"), an array element index (e.g. "colors\[1\]") or both (e.g. "colors\[1\].r").
/// # Returns
/// true if the disconnection was performed successfully, false otherwise
pub fn AiNodeUnlink(node: *mut ai_bindings::AtNode, input: &str) -> bool {
    let input = CString::new(input).unwrap();
    unsafe { ai_bindings::AiNodeUnlink(node, input.as_ptr()) }
}

/// Returns true if the input parameter is linked.
///
/// It will also check for the parameter to be an array, and check if there is any array element connection, and also check for any separate component linking.
///
/// # Parameters
/// * `node` - node to be queried
/// * `input` - the input parameter specification in the given node, which can optionally include a component specification (e.g. "Kd_color.r", "dir.x"), an array element index (e.g. "colors\[1\]") or both (e.g. "colors\[1\].r").
/// # Returns
/// true if the input specification (or any of its components) is explicitly linked, e.g. querying for "Kd_color" will return true if either "Kd_color" or any of its R, G, or B components are linked, but querying for "Kd_color.r" will return true only if the R component has been explicitly linked)
pub fn AiNodeIsLinked(node: *mut ai_bindings::AtNode, input: &str) -> bool {
    let input = CString::new(input).unwrap();
    unsafe { ai_bindings::AiNodeIsLinked(node, input.as_ptr()) }
}

/// Returns the node connected to a given node input parameter.
///
/// # Parameters
/// * `node` - node to be queried
/// * `input` - the input parameter specification in the target node, which can optionally include a component specification (e.g. "Kd_color.r", "dir.x"), an array element index (e.g. "colors\[1\]") or both (e.g. "colors\[1\].r").
/// * `comp` - if not NULL, the specific component selected on the source node output (-1 for the whole output, [0..3] for a single component) will be returned here
/// # Returns
/// pointer to the connected node, or NULL if the parameter or component is not linked
pub fn AiNodeGetLink(
    node: *mut ai_bindings::AtNode,
    input: &str,
    comp: *mut i32,
) -> *mut ai_bindings::AtNode {
    let input = CString::new(input).unwrap();
    unsafe { ai_bindings::AiNodeGetLink(node, input.as_ptr(), comp) }
}

/// Return the node's name.
///
/// This is both faster and easier to use than the equivalent call:
/// ```
/// AiNodeGetStr(node, "name");
/// ```
/// # Parameters
/// * `node` - input node
/// # Returns
/// the node name string, or NULL if the input node was NULL
pub fn AiNodeGetName(node: *mut ai_bindings::AtNode) -> &'static str {
    let slice = unsafe {
        let data = ai_bindings::AiNodeGetName(node);
        CStr::from_ptr(data)
    };
    str::from_utf8(slice.to_bytes()).unwrap()
}

/// Return the node entry for this node.
///
/// # Parameters
/// * `node` - input node
/// # Returns
/// the node entry for this node, or NULL if the input node was NULL
pub fn AiNodeGetNodeEntry(node: *mut ai_bindings::AtNode) -> *const ai_bindings::AtNodeEntry {
    unsafe { ai_bindings::AiNodeGetNodeEntry(node) }
}

/// Returns a pointer to the local data in the node.
///
/// # Parameters
/// * `node` - input node
/// # Returns
/// pointer to local data
pub fn AiNodeGetLocalData(node: *const ai_bindings::AtNode) -> *mut c_void {
    unsafe { ai_bindings::AiNodeGetLocalData(node) }
}

/// Sets local data pointer in the node.
///
/// This can be used to store custom data managed by the user.
///
/// # Parameters
/// * `node` - input node
/// * `data` - pointer to new local data
pub fn AiNodeSetLocalData(node: *mut ai_bindings::AtNode, data: *mut c_void) {
    unsafe { ai_bindings::AiNodeSetLocalData(node, data) }
}

/// Returns a pointer to the per plugin data for the node type, as created in the node_plugin_initialize method.
///
/// #Parameters
/// * `node` - input node
/// # Returns
/// pointer to plugin data
pub fn AiNodeGetPluginData(node: *const ai_bindings::AtNode) -> *mut c_void {
    unsafe { ai_bindings::AiNodeGetPluginData(node) }
}

/// Disable or enable any node in the scene.
///
/// Disabled nodes won't take part in the rendering process and the resulting effect will depend on the type of node:
///
/// * shape : it is simply excluded from the render
/// * procedural : all its contained objects are excluded from render
/// * shader : a disabled shader is replaced with the default shader
/// * light : disabled lights are not used during rendering
/// # Parameters
/// * `node` - pointer to an existing node, generally coming from AiNode()
/// * `disabled` - true to disable this node, false otherwise
pub fn AiNodeSetDisabled(node: *mut ai_bindings::AtNode, disabled: bool) {
    unsafe { ai_bindings::AiNodeSetDisabled(node, disabled) }
}

/// Check if a node has been disabled or not.
///
/// # Parameters
/// * `node` - pointer to an existing node, generally coming from AiNode()
/// # Returns
/// true if the node is disabled, false otherwise
pub fn AiNodeIsDisabled(node: *mut ai_bindings::AtNode) -> bool {
    unsafe { ai_bindings::AiNodeIsDisabled(node) }
}

/// Returns the procedural parent of a node.
///
/// This function returns the "procedural" geometry node that originally created the given node. See Procedural API.
///
/// # Note
/// Although this will work in the majority of cases, it doesn't support a few corner cases, such as automatic procedural instancing as used by the procedural .ass cache, where the first procedural that was loaded will be returned.
/// # Parameters
/// * `node` - pointer to an existing node, generally coming from AiNode()
/// # Returns
/// procedural parent node, or NULL if the node does not come from one
pub fn AiNodeGetParent(node: *mut ai_bindings::AtNode) -> *mut ai_bindings::AtNode {
    unsafe { ai_bindings::AiNodeGetParent(node) }
}

/// Creates and returns a new AtUserParamIterator for this node.
///
/// # Parameters
/// * `node` - node whose user parameters will be iterated over
/// # Returns
/// an iterator over all user parameters on this node
pub fn AiNodeGetUserParamIterator(
    node: *mut ai_bindings::AtNode,
) -> *mut ai_bindings::AtUserParamIterator {
    unsafe { ai_bindings::AiNodeGetUserParamIterator(node) }
}

/// Destroys a user param iterator when it is no longer needed.
///
/// # Parameters
/// * `iter` - user param iterator that will be deallocated
pub fn AiUserParamIteratorDestroy(iter: *mut ai_bindings::AtUserParamIterator) {
    unsafe { ai_bindings::AiUserParamIteratorDestroy(iter) }
}

/// Returns current user param entry and points user param iterator to the next one.
///
/// This function is designed to be used inside a loop, as illustrated by the following example, which prints all the user-defined parameters of a given AtNode:
/// ```
/// AtUserParamIterator *iter = AiNodeGetUserParamIterator(node);
/// while (!AiUserParamIteratorFinished(iter))
/// {
///    AtUserParamEntry *upentry = AiUserParamIteratorGetNext(iter);
///    printf("%s\n", AiUserParamGetName(upentry));
/// }
/// AiUserParamIteratorDestroy(iter);
/// ```
/// ## Note
/// If this is an instance, the iterator will NOT include user parameters that were declared in the reference node, even though AiNodeLookUpUserParameter() would find such user parameters in the instance.
/// 
/// # Parameters
/// * `iter` - a user param iterator
/// # Returns
/// the current user param entry pointed by the iterator, or NULL if there are no more user parameters to iterate over
pub fn AiUserParamIteratorGetNext(
    iter: *mut ai_bindings::AtUserParamIterator,
) -> *const ai_bindings::AtUserParamEntry {
    unsafe { ai_bindings::AiUserParamIteratorGetNext(iter) }
}

/// Returns true if there are no more user parameters to iterate over.
///
/// # Parameters
/// * `iter` - a user param iterator
/// # Returns
/// true if the user param iterator has moved past the last user parameter
pub fn AiUserParamIteratorFinished(iter: *const ai_bindings::AtUserParamIterator) -> bool {
    unsafe { ai_bindings::AiUserParamIteratorFinished(iter) }
}

pub fn AiNodeSetByte(node: *mut ai_bindings::AtNode, param: AtString, val: u8) {
    unsafe { ai_bindings::AiNodeSetByte(node, param, val) }
}
pub fn AiNodeSetInt(node: *mut ai_bindings::AtNode, param: AtString, val: i32) {
    unsafe { ai_bindings::AiNodeSetInt(node, param, val) }
}
pub fn AiNodeSetUInt(node: *mut ai_bindings::AtNode, param: AtString, val: u32) {
    unsafe { ai_bindings::AiNodeSetUInt(node, param, val) }
}
pub fn AiNodeSetBool(node: *mut ai_bindings::AtNode, param: AtString, val: bool) {
    unsafe { ai_bindings::AiNodeSetBool(node, param, val) }
}
pub fn AiNodeSetFlt(node: *mut ai_bindings::AtNode, param: AtString, val: f32) {
    unsafe { ai_bindings::AiNodeSetFlt(node, param, val) }
}
pub fn AiNodeSetPtr(
    node: *mut ai_bindings::AtNode,
    param: AtString,
    val: *mut ::std::os::raw::c_void,
) {
    unsafe { ai_bindings::AiNodeSetPtr(node, param, val) }
}
pub fn AiNodeSetArray(
    node: *mut ai_bindings::AtNode,
    param: AtString,
    val: *mut ai_bindings::AtArray,
) {
    unsafe { ai_bindings::AiNodeSetArray(node, param, val) }
}
pub fn AiNodeSetMatrix(
    node: *mut ai_bindings::AtNode,
    param: AtString,
    val: ai_bindings::AtMatrix,
) {
    unsafe { ai_bindings::AiNodeSetMatrix(node, param, val) }
}
pub fn AiNodeSetStr(node: *mut ai_bindings::AtNode, param: AtString, str: AtString) {
    unsafe { ai_bindings::AiNodeSetStr(node, param, str) }
}
pub fn AiNodeSetRGB(node: *mut ai_bindings::AtNode, param: AtString, r: f32, g: f32, b: f32) {
    unsafe { ai_bindings::AiNodeSetRGB(node, param, r, g, b) }
}
pub fn AiNodeSetRGBA(
    node: *mut ai_bindings::AtNode,
    param: AtString,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    unsafe { ai_bindings::AiNodeSetRGBA(node, param, r, g, b, a) }
}
pub fn AiNodeSetVec(node: *mut ai_bindings::AtNode, param: AtString, x: f32, y: f32, z: f32) {
    unsafe { ai_bindings::AiNodeSetVec(node, param, x, y, z) }
}
pub fn AiNodeSetVec2(node: *mut ai_bindings::AtNode, param: AtString, x: f32, y: f32) {
    unsafe { ai_bindings::AiNodeSetVec2(node, param, x, y) }
}
pub fn AiNodeSetAttributes(node: *mut ai_bindings::AtNode, attributes: &str) {
    let attributes = CString::new(attributes).unwrap();
    unsafe { ai_bindings::AiNodeSetAttributes(node, attributes.as_ptr()) }
}

pub fn AiNodeGetByte(node: *const ai_bindings::AtNode, param: &str) -> u8 {
    unsafe { ai_bindings::AiNodeGetByte(node, AtString::from(param)) }
}
pub fn AiNodeGetInt(node: *const ai_bindings::AtNode, param: &str) -> i32 {
    unsafe { ai_bindings::AiNodeGetInt(node, AtString::from(param)) }
}
pub fn AiNodeGetUInt(node: *const ai_bindings::AtNode, param: &str) -> u32 {
    unsafe { ai_bindings::AiNodeGetUInt(node, AtString::from(param)) }
}
pub fn AiNodeGetBool(node: *const ai_bindings::AtNode, param: &str) -> bool {
    unsafe { ai_bindings::AiNodeGetBool(node, AtString::from(param)) }
}
pub fn AiNodeGetFlt(node: *const ai_bindings::AtNode, param: &str) -> f32 {
    unsafe { ai_bindings::AiNodeGetFlt(node, AtString::from(param)) }
}
pub fn AiNodeGetRGB(node: *const ai_bindings::AtNode, param: &str) -> ai_bindings::AtRGB {
    unsafe { ai_bindings::AiNodeGetRGB(node, AtString::from(param)) }
}
pub fn AiNodeGetRGBA(node: *const ai_bindings::AtNode, param: &str) -> ai_bindings::AtRGBA {
    unsafe { ai_bindings::AiNodeGetRGBA(node, AtString::from(param)) }
}
pub fn AiNodeGetVec(node: *const ai_bindings::AtNode, param: &str) -> AtVector {
    unsafe { ai_bindings::AiNodeGetVec(node, AtString::from(param)) }
}
pub fn AiNodeGetVec2(node: *const ai_bindings::AtNode, param: &str) -> AtVector2 {
    unsafe { ai_bindings::AiNodeGetVec2(node, AtString::from(param)) }
}
pub fn AiNodeGetStr(node: *const ai_bindings::AtNode, param: &str) -> AtString {
    unsafe { ai_bindings::AiNodeGetStr(node, AtString::from(param)) }
}
pub fn AiNodeGetPtr(node: *const ai_bindings::AtNode, param: &str) -> *mut c_void {
    unsafe { ai_bindings::AiNodeGetPtr(node, AtString::from(param)) }
}
pub fn AiNodeGetArray(
    node: *const ai_bindings::AtNode,
    param: AtString,
) -> *mut ai_bindings::AtArray {
    unsafe { ai_bindings::AiNodeGetArray(node, param) }
}
pub fn AiNodeGetMatrix(node: *const ai_bindings::AtNode, param: AtString) -> ai_bindings::AtMatrix {
    unsafe { ai_bindings::AiNodeGetMatrix(node, param) }
}

//! ASS File API
//!
//! Arnold has built-in support for writing scene data to a file and later reading the file in. Although not required, the extension for these files is usually .ass, which stands for Arnold Scene Source. The file format is a straightforward mapping from Arnold AtNode's to human-readable ASCII. For example, a sphere node is written as:
//! ```
//! sphere          // this is the node class
//! {               // any number of param/value pairs enclosed in curly braces
//!  center 0 0 0   //  parameter "center" of type AtVector is set to value (0,0,0)
//!  radius 2.0     //  parameter "radius" of type float is set to value 2.0
//! }               // end of node block

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
use ai_bindings::AtMetadataStore;

use std::ffi::CString;
/// Write all nodes in the scene to an .ass file.
/// 
/// This function can selectively write all nodes of a given type to an .ass file. For example, to write light nodes and camera nodes only, use:
/// ```
/// AiASSWrite("lightsncams.ass", AI_NODE_LIGHT + AI_NODE_CAMERA, false);
/// ```
/// 
/// To write all nodes of all types, use:
/// ```
/// AiASSWrite("everything.ass", AI_NODE_ALL, false);
/// ```
/// 
/// Just like AiASSLoad(), this function has built-in gzip compression. If filename ends in ".gz", the generated file will be automatically compressed.
/// 
/// # Warning
/// Because nodes are partially-released (their array-parameters are released), we can NO LONGER RENDER after a call to AiASSWrite().
/// # Parameters
/// * `filename` - output filename
/// * `mask` - only write the desired types of nodes
/// * `open_procs` - if set, then all procedurals will be recursively expanded
/// * `binary` - allow binary encoding in .ass files
/// # Returns
/// 0 if the file was written succesfully, -1 if error
pub fn AiASSWrite(filename: &str, mask: i32, open_procs: bool, binary: bool) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        ai_bindings::AiASSWrite(filename.as_ptr(), mask, open_procs, binary)
    }
}

/// Write all nodes in the scene to an .ass file, including metadata.
/// 
/// See AiASSWrite
/// 
/// # Parameters
/// * `filename` - output filename
/// * `mask` - only write the desired types of nodes
/// * `open_procs` - if set, then all procedurals will be recursively expanded
/// * `binary` - allow binary encoding in .ass files
/// * `mds` - optional metadata store for writing metadata into the file
/// # Returns
/// 0 if the file was written succesfully, -1 if error
pub fn AiASSWriteWithMetadata(filename: &str, mask: i32, open_procs: bool, binary: bool, mds: *const AtMetadataStore) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        ai_bindings::AiASSWriteWithMetadata(filename.as_ptr(), mask, open_procs, binary, mds)
    }
}

/// Load all nodes from an .ass file into Arnold.
/// 
/// This function automatically recognizes gzip-compressed files; first, it tries to load filename and, if not found, it will add the suffix ".gz" and try again before finally giving up.
/// 
/// If the filename is "-", it reads data from stdin.
/// 
/// Any forward references due to linked nodes will be automatically resolved at the end of the file. The order in which nodes appear in the file is irrelevant.
/// 
/// # Parameters
/// * `filename` - input filename
/// * `mask` - only read nodes with types included in the mask (default is AI_NODE_ALL)
/// # Returns
/// 0 if the file was read successfully, -1 if error
pub fn AiASSLoad(filename: &str, mask: i32) -> i32{
    let filename = CString::new(filename).unwrap();
    unsafe {
        ai_bindings::AiASSLoad(filename.as_ptr(), mask)
    }
}
    

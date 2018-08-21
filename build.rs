extern crate bindgen;
extern crate regex;

use bindgen::callbacks::{ParseCallbacks, IntKind};
use regex::Regex;
use std::env;
use std::path::PathBuf;

fn main() {
    let arnold_path = PathBuf::from(env::var("ARNOLD_ROOT").unwrap());
    let arnold_include_path = arnold_path.join("include");
    let arnold_lib_path = arnold_path.join("bin");

    println!("cargo:rustc-link-search={}", arnold_lib_path.to_string_lossy());
    println!("cargo:rustc-link-lib=ai");
    
    #[derive(Debug)]
    struct ArnoldConstType;
    
    // Always set macros int type to signed never unsigned
    impl ParseCallbacks for ArnoldConstType {        
        fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
            let log_re = Regex::new(r"^AI_LOG_.+$").unwrap();
            if log_re.is_match(_name){
                Some(IntKind::I32)
            }else{
                None
            }            
        }
    }

    let node_bindings = bindgen::Builder::default()    
        .header("wrapper.hpp")
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I{}", arnold_include_path.to_string_lossy()))
        .parse_callbacks(Box::new(ArnoldConstType))
        .blacklist_type("max_align_t")
        //.generate_inline_functions(true) 
        // ai_allocate
        .whitelist_function("AiMalloc")
        .whitelist_function("AiRealloc")
        .whitelist_function("AiFree")
        .whitelist_function("AiAddMemUsage")
        // ai_array
        .whitelist_type("AtArray")
        .whitelist_function("AiArray.*")
        // ai_bbox
        .blacklist_type("AtBBox.*")
        .whitelist_function("AiBBox.*")
        // ai_color
        .whitelist_type("AtRGB.*")
        .whitelist_function("AiRGB.*")
        .whitelist_function("AiColor.*")
        // ai_constants
        .blacklist_type("AI_PI")
        .blacklist_type("AI_PITIMES2")
        .blacklist_type("AI_PIOVER2")
        .blacklist_type("AI_ONEOVERPI")
        .blacklist_type("AI_ONEOVER2PI")
        .blacklist_type("AI_E")
        .blacklist_type("AI_LOG2E")
        .blacklist_type("AI_LN2")
        .blacklist_type("AI_SQRT2")
        .blacklist_type("AI_SQRT3")
        .blacklist_type("AI_GOLDEN")
        .blacklist_type("AI_DTOR")
        .blacklist_type("AI_RTOD")
        .blacklist_type("AI_EPSILON")
        .blacklist_type("AI_OPACITY_EPSILON")
        .blacklist_type("AI_BIG")
        .blacklist_type("AI_INFINITE")
        .blacklist_type("AI_ALMOST_ONE")
        // ai_dotass
        .whitelist_function("AiASS.*")
        // ai_drivers
        .whitelist_function("AiFindDriverType")
        .whitelist_function("AiDriverExtension")
        // ai_enum
        .whitelist_type("AtEnum")
        .whitelist_function("AiEnum.*")
        // ai_license
        .whitelist_type("AtLicenseInfo")
        .whitelist_function("AiLicenseGetInfo")
        .whitelist_var("AI_LIC_.+")
        // ai_matrix
        .whitelist_type("AtMatrix")
        .whitelist_function("AiMatrix.*")
        .whitelist_var("AI_M4_IDENTITY")
        .whitelist_var("AI_M4_ZERO")
        // ai_metadata
        .whitelist_type("AtMetadataStore")
        .whitelist_function("AiMetaData.*")
        // ai_msg
        .whitelist_type("AtMsg")
        .whitelist_function("AiMsg.*")
        .whitelist_var("AI_LOG_.+")
        .whitelist_var("AI_SEVERITY_.+")
        // ai_node_entry
        .whitelist_type("AtNodeEntry")
        .whitelist_function("AiNodeEntry.*")
        .whitelist_var("AI_NODE_.+")
        // ai_nodes
        .whitelist_type("AtNode")
        .whitelist_function("AiNode.*")
        // ai_operator
        .whitelist_function("AiOp.*")
        // ai_params
        .whitelist_type("AtParam.+")
        .whitelist_function("AiParam.*")
        .whitelist_function("AiNodeParam.*")
        .whitelist_function("AiUserParam.*")
        .whitelist_var("AI_TYPE_.+")
        .whitelist_var("AI_USERDEF_.+")
        // ai_plugins
        .whitelist_type("AtNodeLib")
        .whitelist_function("AiLoadPlugins")
        // ai_ray
        .whitelist_type("AtRay")
        .whitelist_function("AiMakeRay")
        .whitelist_function("AiReflectRay")
        .whitelist_function("AiRefractRay")
        .whitelist_function("AiTrace.*")
        .whitelist_var("AI_RAY_.+")
        // ai_render
        .rustified_enum("AtBlockingCall")
        .rustified_enum("AtSessionMode")
        .rustified_enum("AtRenderMode")
        .whitelist_type("AtRenderStatus")
        .whitelist_function("AiBegin")
        .whitelist_function("AiEnd")
        .whitelist_function("AiGetSessionMode")
        .whitelist_function("AiRender.*")
        // ai_shader_radiance
        .whitelist_function("AiIrradiance")
        .whitelist_function("AiRadiance")
        // ai_stats
        .whitelist_function("AiStats.+")
        .whitelist_function("AiProfile.+")
        // ai_string
        .whitelist_type("AtString.*")
        .whitelist_function("AiCreateAtStringData_private")
        .whitelist_function("AiAtStringLength")
        .whitelist_function("AiAtStringHash")
        // ai_texture
        .whitelist_function("AiTextureGet.+")
        .whitelist_function("AiTextureInvalidate")
        // ai_unit_test
        .whitelist_function("AiTest")
        // ai_universe
        .whitelist_type("AtAOVEntry")
        .whitelist_function("AiUniverse.+")
        .whitelist_function("AiNodeIterator.+")
        .whitelist_function("AiNodeEntryIterator.+")
        .whitelist_function("AiAOVIterator.+")
        .whitelist_var("AI_CACHE_.+")
        // ai_vector
        .blacklist_type("AtVector.*") // reimplemented in ai_vector.rs
        .whitelist_type("AtHPoint.*")
        .whitelist_function("AiV[234].+")
        .whitelist_var("AI_[XYZ]")
        .whitelist_var("AI_[VP][234]_.+")
        // ai_version
        .whitelist_function("AiGetVersion.*")
        .whitelist_function("AiGetCompileOptions")
        .whitelist_function("AiCheckAPIVersion")
        .whitelist_function("AiSetAppString")
        .whitelist_var("AI_VERSION.*")
        // ai_volume
        .whitelist_function("AiVolumeFileGetChannels")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());    
    node_bindings
        .write_to_file(out_path.join("arnold_bindings.rs"))
        .expect("Couldn't write bindings!");
}
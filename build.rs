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
        .header(arnold_include_path.join("ai.h").to_str().expect("Unable to create header path"))
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I{}", arnold_include_path.to_string_lossy()))
        .parse_callbacks(Box::new(ArnoldConstType))
        .hide_type("max_align_t")
        // ai_allocate
        .whitelisted_function("AiMalloc")
        .whitelisted_function("AiRealloc")
        .whitelisted_function("AiFree")
        .whitelisted_function("AiAddMemUsage")
        // ai_array
        .whitelisted_type("AtArray")
        .whitelisted_function("AiArray.*")
        // ai_bbox
        .whitelisted_type("AtBBox")
        .whitelisted_function("AiBBox.*")
        // ai_color
        .whitelisted_type("AtRGB.*")
        .whitelisted_function("AiRGB.*")
        .whitelisted_function("AiColor.*")
        // ai_constants
        .whitelisted_var("AI_PI")
        .whitelisted_var("AI_PITIMES2")
        .whitelisted_var("AI_PIOVER2")
        .whitelisted_var("AI_ONEOVERPI")
        .whitelisted_var("AI_ONEOVER2PI")
        .whitelisted_var("AI_E")
        .whitelisted_var("AI_LOG2E")
        .whitelisted_var("AI_LN2")
        .whitelisted_var("AI_SQRT2")
        .whitelisted_var("AI_SQRT3")
        .whitelisted_var("AI_GOLDEN")
        .whitelisted_var("AI_DTOR")
        .whitelisted_var("AI_RTOD")
        .whitelisted_var("AI_EPSILON")
        .whitelisted_var("AI_OPACITY_EPSILON")
        .whitelisted_var("AI_BIG")
        .whitelisted_var("AI_INFINITE")
        .whitelisted_var("AI_ALMOST_ONE")
        // ai_dotass
        .whitelisted_function("AiASS.*")
        // ai_drivers
        .whitelisted_function("AiFindDriverType")
        .whitelisted_function("AiDriverExtension")
        // ai_enum
        .whitelisted_type("AtEnum")
        .whitelisted_function("AiEnum.*")
        // ai_license
        .whitelisted_type("AtLicenseInfo")
        .whitelisted_function("AiLicenseGetInfo")
        .whitelisted_var("AI_LIC_.+")
        // ai_matrix
        .whitelisted_type("AtMatrix")
        .whitelisted_function("AiMatrix.*")
        .whitelisted_var("AI_M4_IDENTITY")
        .whitelisted_var("AI_M4_ZERO")
        // ai_metadata
        .whitelisted_type("AtMetadataStore")
        .whitelisted_function("AiMetaData.*")
        // ai_msg
        .whitelisted_type("AtMsg")
        .whitelisted_function("AiMsg.*")
        .whitelisted_var("AI_LOG_.+")
        .whitelisted_var("AI_SEVERITY_.+")
        // ai_node_entry
        .whitelisted_type("AtNodeEntry")
        .whitelisted_function("AiNodeEntry.*")
        .whitelisted_var("AI_NODE_.+")
        // ai_nodes
        .whitelisted_type("AtNode")
        .whitelisted_function("AiNode.*")
        // ai_operator
        .whitelisted_function("AiOp.*")
        // ai_params
        .whitelisted_type("AtParam.+")
        .whitelisted_function("AiParam.*")
        .whitelisted_function("AiNodeParam.*")
        .whitelisted_function("AiUserParam.*")
        .whitelisted_var("AI_TYPE_.+")
        .whitelisted_var("AI_USERDEF_.+")
        // ai_plugins
        .whitelisted_type("AtNodeLib")
        .whitelisted_function("AiLoadPlugins")
        // ai_ray
        .whitelisted_type("AtRay")
        .whitelisted_function("AiMakeRay")
        .whitelisted_function("AiReflectRay")
        .whitelisted_function("AiRefractRay")
        .whitelisted_function("AiTrace.*")
        .whitelisted_var("AI_RAY_.+")
        // ai_render
        .whitelisted_type("AtRenderStatus")
        .whitelisted_function("AiBegin")
        .whitelisted_function("AiEnd")
        .whitelisted_function("AiGetSessionMode")
        .whitelisted_function("AiRender.*")
        // ai_shader_radiance
        .whitelisted_function("AiIrradiance")
        .whitelisted_function("AiRadiance")
        // ai_stats
        .whitelisted_function("AiStats.+")
        .whitelisted_function("AiProfile.+")
        // ai_string
        .whitelisted_type("AtString.*")
        .whitelisted_function("AiCreateAtStringData_private")
        .whitelisted_function("AiAtStringLength")
        .whitelisted_function("AiAtStringHash")
        // ai_texture
        .whitelisted_function("AiTextureGet.+")
        .whitelisted_function("AiTextureInvalidate")
        // ai_unit_test
        .whitelisted_function("AiTest")
        // ai_universe
        .whitelisted_type("AtAOVEntry")
        .whitelisted_function("AiUniverse.+")
        .whitelisted_function("AiNodeIterator.+")
        .whitelisted_function("AiNodeEntryIterator.+")
        .whitelisted_function("AiAOVIterator.+")
        .whitelisted_var("AI_CACHE_.+")
        // ai_vector
        .whitelisted_type("AtVector.*")
        .whitelisted_type("AtHPoint.*")
        .whitelisted_function("AiV[234].+")
        .whitelisted_var("AI_[XYZ]")
        .whitelisted_var("AI_[VP][234]_.+")
        // ai_version
        .whitelisted_function("AiGetVersion.*")
        .whitelisted_function("AiGetCompileOptions")
        .whitelisted_function("AiCheckAPIVersion")
        .whitelisted_function("AiSetAppString")
        .whitelisted_var("AI_VERSION.*")
        // ai_volume
        .whitelisted_function("AiVolumeFileGetChannels")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());    
    node_bindings
        .write_to_file(out_path.join("arnold_bindings.rs"))
        .expect("Couldn't write bindings!");
}
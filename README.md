# Rust bindings for the Arnold rendering API

## Modules
- [ ] ai_allocate
- [ ] ai_api
- [x] ai_array
- [ ] ai_bbox
- [ ] ai_cameras
- [ ] ai_closure
- [x] ai_color
- [ ] ai_color_managers
- [ ] ai_comparison
- [x] ai_constants
- [ ] ai_critsec
- [ ] ai_deprecated
- [ ] ai_device
- [x] ai_dotass
- [ ] ai_driver_utils
- [ ] ai_drivers
- [ ] ai_enum
- [ ] ai_filters
- [ ] ai_license
- [ ] ai_math
- [ ] ai_matrix
- [ ] ai_matrix_private
- [ ] ai_metadata
- [ ] ai_msg
- [ ] ai_node_entry
- [x] ai_nodes
- [ ] ai_noise
- [ ] ai_operator
- [ ] ai_params
- [ ] ai_plugins
- [ ] ai_pointcloud
- [ ] ai_procedural
- [ ] ai_ray
- [ ] ai_render
- [ ] ai_sampler
- [ ] ai_shader_aovs
- [ ] ai_shader_bsdf
- [ ] ai_shader_closure
- [ ] ai_shader_lights
- [ ] ai_shader_message
- [ ] ai_shader_parameval
- [ ] ai_shader_radiance
- [ ] ai_shader_sample
- [ ] ai_shader_sss
- [ ] ai_shader_userdef
- [ ] ai_shader_util
- [ ] ai_shader_volume
- [ ] ai_shaderglobals
- [ ] ai_shaders
- [ ] ai_stats
- [x] ai_string
- [ ] ai_texture
- [ ] ai_threads
- [ ] ai_unit_test
- [ ] ai_universe
- [ ] ai_vector
- [ ] ai_version
- [ ] ai_volume

## Build
### Environment Setup
Before you build and run you need to make sure arnold can be found.
```bash
export ARNOLD_ROOT=/path/to/arnold/root;
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$ARNOLD_ROOT/bin;
```
### Build
```bash
cargo build
```

### Test
```bash
cargo test string_cmp -- --test-threads=1
```
This will run the test *string_cmp* see [Attention Tests](#Tests) why we need to run them individually.
You can run the following to list the available tests.
```bash
cargo test -- --list
...
ai_render::tests::render_testing: test
ai_string::tests::string_cmp: test
ai_string::tests::string_empty: test
ai_string::tests::string_hash: test
ai_string::tests::string_length: test
```
# Documentation
To build documentation, run:
```cargo rustdoc --lib -- --sort-modules-by-appearance -Z unstable-options```
## !!Attention!!
### WIP
This is an extremely early test to learn rust. Don't expect much.

### Tests
You need to run the tests individually.
Even if we run tests with ```cargo test -- --test-threads=1``` we get panics some times.

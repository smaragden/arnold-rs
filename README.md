# Rust bindings for the Arnold rendering API

## Modules
|                            |                            |
|----------------------------|----------------------------|
| <ul><li>[ ] ai_allocate</li></ul>          | - [ ] ai_operator          |
| <ul><li>[ ] ai_api</li></ul>               | - [ ] ai_params            |
| <ul><li>[x] ai_array</li></ul>             | - [ ] ai_plugins           |
| <ul><li>[ ] ai_bbox</li></ul>              | - [ ] ai_pointcloud        |
| <ul><li>[ ] ai_cameras</li></ul>           | - [ ] ai_procedural        |
| <ul><li>[ ] ai_closure</li></ul>           | - [ ] ai_ray               |
| <ul><li>[x] ai_color</li></ul>             | - [ ] ai_render            |
| <ul><li>[ ] ai_color_managers</li></ul>    | - [ ] ai_sampler           |
| <ul><li>[ ] ai_comparison</li></ul>        | - [ ] ai_shader_aovs       |
| <ul><li>[x] ai_constants</li></ul>         | - [ ] ai_shader_bsdf       |
| <ul><li>[ ] ai_critsec</li></ul>           | - [ ] ai_shader_closure    |
| <ul><li>[ ] ai_deprecated</li></ul>        | - [ ] ai_shader_lights     |
| <ul><li>[ ] ai_device</li></ul>            | - [ ] ai_shader_message    |
| <ul><li>[x] ai_dotass</li></ul>            | - [ ] ai_shader_parameval  |
| <ul><li>[ ] ai_driver_utils</li></ul>      | - [ ] ai_shader_radiance   |
| <ul><li>[ ] ai_drivers</li></ul>           | - [ ] ai_shader_sample     |
| <ul><li>[ ] ai_enum</li></ul>              | - [ ] ai_shader_sss        |
| <ul><li>[ ] ai_filters</li></ul>           | - [ ] ai_shader_userdef    |
| <ul><li>[ ] ai_license</li></ul>           | - [ ] ai_shader_util       |
| <ul><li>[ ] ai_math</li></ul>              | - [ ] ai_shader_volume     |
| <ul><li>[ ] ai_matrix</li></ul>            | - [ ] ai_shaderglobals     |
| <ul><li>[ ] ai_matrix_private</li></ul>    | - [ ] ai_shaders           |
| <ul><li>[ ] ai_metadata</li></ul>          | - [ ] ai_stats             |
| <ul><li>[ ] ai_msg</li></ul>               | - [x] ai_string            |
| <ul><li>[ ] ai_node_entry</li></ul>        | - [ ] ai_texture           |
| <ul><li>[x] ai_nodes</li></ul>             | - [ ] ai_threads           |
| <ul><li>[ ] ai_noise</li></ul>             | - [ ] ai_unit_test         |
| <ul><li>[ ] ai_universe</li></ul>          |                            |
| <ul><li>[ ] ai_vector</li></ul>            |                            |
| <ul><li>[ ] ai_version</li></ul>           |                            |
| <ul><li>[ ] ai_volume</li></ul>            |                            |




























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

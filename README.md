# Rust bindings for the Arnold rendering API

## Modules
|                                        |                                        |                                          |
|----------------------------------------|----------------------------------------|------------------------------------------|
|<ul><li>[ ] ai_allocate</li></ul>       |<ul><li>[ ] ai_matrix</li></ul>         |<ul><li>[ ] ai_shader_aovs</li></ul>      |
|<ul><li>[ ] ai_api</li></ul>            |<ul><li>[ ] ai_matrix_private</li></ul> |<ul><li>[ ] ai_shader_bsdf</li></ul>      |
|<ul><li>[x] ai_array</li></ul>          |<ul><li>[ ] ai_metadata</li></ul>       |<ul><li>[ ] ai_shader_closure</li></ul>   |
|<ul><li>[ ] ai_bbox</li></ul>           |<ul><li>[ ] ai_msg</li></ul>            |<ul><li>[ ] ai_shader_lights</li></ul>    |
|<ul><li>[ ] ai_cameras</li></ul>        |<ul><li>[ ] ai_node_entry</li></ul>     |<ul><li>[ ] ai_shader_message</li></ul>   |
|<ul><li>[ ] ai_closure</li></ul>        |<ul><li>[x] ai_nodes</li></ul>          |<ul><li>[ ] ai_shader_parameval</li></ul> |
|<ul><li>[x] ai_color</li></ul>          |<ul><li>[ ] ai_noise</li></ul>          |<ul><li>[ ] ai_shader_radiance</li></ul>  |
|<ul><li>[ ] ai_color_managers</li></ul> |<ul><li>[ ] ai_universe</li></ul>       |<ul><li>[ ] ai_shader_sample</li></ul>    |
|<ul><li>[ ] ai_comparison</li></ul>     |<ul><li>[ ] ai_vector</li></ul>         |<ul><li>[ ] ai_shader_sss</li></ul>       |
|<ul><li>[x] ai_constants</li></ul>      |<ul><li>[ ] ai_version</li></ul>        |<ul><li>[ ] ai_shader_userdef</li></ul>   |
|<ul><li>[ ] ai_critsec</li></ul>        |<ul><li>[ ] ai_volume</li></ul>         |<ul><li>[ ] ai_shader_util</li></ul>      |
|<ul><li>[ ] ai_deprecated</li></ul>     |<ul><li>[ ] ai_operator</li></ul>       |<ul><li>[ ] ai_shader_volume</li></ul>    |
|<ul><li>[ ] ai_device</li></ul>         |<ul><li>[ ] ai_params</li></ul>         |<ul><li>~~[ ] ai_shaderglobals~~</li></ul>|
|<ul><li>[x] ai_dotass</li></ul>         |<ul><li>[ ] ai_plugins</li></ul>        |<ul><li>[ ] ai_shaders</li></ul>          |
|<ul><li>[ ] ai_driver_utils</li></ul>   |<ul><li>[ ] ai_pointcloud</li></ul>     |<ul><li>[ ] ai_stats</li></ul>            |
|<ul><li>[ ] ai_drivers</li></ul>        |<ul><li>[ ] ai_procedural</li></ul>     |<ul><li>[x] ai_string</li></ul>           |
|<ul><li>[ ] ai_enum</li></ul>           |<ul><li>[ ] ai_ray</li></ul>            |<ul><li>[ ] ai_texture</li></ul>          |
|<ul><li>[ ] ai_filters</li></ul>        |<ul><li>[ ] ai_render</li></ul>         |<ul><li>[ ] ai_unit_test</li></ul>        |
|<ul><li>[ ] ai_license</li></ul>        |<ul><li>[ ] ai_threads</li></ul>        |                                          |
|<ul><li>[ ] ai_math</li></ul>           |<ul><li>[ ] ai_sampler</li></ul>        |                                          |

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

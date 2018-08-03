# Rust bindings for the Arnold rendering API

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


## !!Attention!!
### WIP
This is an extremely early test to learn rust. Don't expect much.

### Tests
You need to run the tests individually.
Even if we run tests with ```cargo test -- --test-threads=1``` we get panics some times.

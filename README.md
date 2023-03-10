# Introduction

Embed a rust static library alongside C code on a nRF52-DK (nRF52832).

Project template is based on "ble app blinky" example from nRF5 SDK.

# Pre build script

At each compile from Segger Embedded Studio, pre-build.sh script is run. It will:
- compile rust library, which will:
    - use rust-bindgen to generate a Rust bindings to C functions (bindings.rs)
    - compile a rust static library (.a)
- use cbindgen to generate C bindings to Rust functions (rust_embedded_lib.h file)

# Capability

- debugger seems to work for rust code, but we have to specify where to find .rs files, not found automatically by the IDE.
- can call Rust function from C code, passing and returning basic type.
- can call C function from Rust code.

# Improvement

- test interacting with peripheral
- try unofficial nrf HAL from https://github.com/nrf-rs/nrf-hal

# Dependencies

- cargo
- cbindgen
- rust-bindgen
- segger embedded studio

# References

- https://docs.rust-embedded.org/book/interoperability/rust-with-c.html
- https://docs.rust-embedded.org/book/interoperability/c-with-rust.html
- https://dev.to/apollolabsbin/rust-ffi-and-cbindgen-integrating-embedded-rust-code-in-c-59f8

# Introduction

Embed a rust static library alongside C code on a nRF52-DK (nRF52832).

Project template is based on "ble app blinky" example from nRF5 SDK.

# Pre build script

pre-build.sh will compile rust library and generate C header using cbindgen at each compile from Segger Embedded Studio.

# Capability

- debugger seems to work for rust code, but we have to specify where to find .rs files, not found automatically by the IDE.
- can pass and return basic type to and from rust function.

# Improvement

- test interacting with peripheral
- test the other way around : C static library alongside Rust code
- try unofficial nrf HAL from https://github.com/nrf-rs/nrf-hal

# Dependencies

- cargo
- cbindgen
- segger embedded studio

# References

- https://docs.rust-embedded.org/book/interoperability/rust-with-c.html
- https://dev.to/apollolabsbin/rust-ffi-and-cbindgen-integrating-embedded-rust-code-in-c-59f8

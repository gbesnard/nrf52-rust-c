set -e
set -x

echo "pre-build script"

cd rust_embedded_lib
cargo build --target thumbv7em-none-eabihf
cbindgen --config cbindgen.toml --crate rust_embedded_lib --lang c --output rust_embedded_lib.h

cd -

exit 0

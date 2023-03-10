set -e
set -x

echo "pre-build script"

cd rust_embedded_lib

if [ $1 = "debug" ]; then
    cargo build --target thumbv7em-none-eabihf
else
    cargo build --target thumbv7em-none-eabihf --release
fi

cbindgen --config cbindgen.toml --crate rust_embedded_lib --lang c --output ../rust_embedded_lib.h

exit 0

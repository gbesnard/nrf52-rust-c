set -e
set -x

echo "pre-build script"

cd rust_embedded_lib

if [ $1 = "debug" ]; then
    cargo build
else
    cargo build --release
fi

cbindgen --config cbindgen.toml --crate rust_embedded_lib --lang c --output ../rust_embedded_lib.h

exit 0

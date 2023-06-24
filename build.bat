cargo build --package exposed-gl --profile dev_release
cargo build --package exposed --profile dev_release
cargo build --package exposed-gl --profile dev_release --target i686-pc-windows-msvc
cargo build --package exposed --profile dev_release --target i686-pc-windows-msvc

cbindgen --config cbindgen.toml --crate exposed --output target/exposed.h 
cbindgen --config cbindgen.toml --crate exposed-gl --output target/exposed_gl.h 
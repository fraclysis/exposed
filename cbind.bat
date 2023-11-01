cbindgen --config cbindgen.toml --crate exposed --output includes/exposed.h --lang c
cbindgen --config cbindgengl.toml --crate exposed-gl --output includes/exposed_gl.h --lang c 
use std::{fs::read_to_string, path::Path};

use exposed_key_gen::{c, rs};

fn main() {
    const KEY_FILE: &str = "keys.txt";

    let key_file = read_to_string(KEY_FILE).unwrap();

    {
        let contents = exposed_key_gen::bindings(&key_file, 0, rs::parse).unwrap();
        std::fs::write("exposed/src/window/keys.rs", contents).unwrap();
    }

    {
        if Path::new("includes").exists() {
            let contents = exposed_key_gen::bindings(&key_file, 1, c::parse).unwrap();
            std::fs::write("includes/exposed_keys.h", contents).unwrap();
        }
    }
}

pub use std::{
    fs::read_to_string,
    io::{Error, Write},
};

pub fn bindings(
    file: &str, variant: usize, proc: fn(f: &mut Vec<u8>, keys: &Vec<KeyDef>) -> Result<(), Error>,
) -> Result<Vec<u8>, Error> {
    let keys = parse_keys(&file, variant);

    let mut buffer = Vec::new();
    writeln!(&mut buffer, "// This file is automatically @generated.\n// It is not intended for manual editing.")?;
    proc(&mut buffer, &keys)?;
    Ok(buffer)
}

#[derive(Debug, Clone, Copy)]
pub enum Os {
    Windows,
    Linux,
    Android,
}

#[derive(Debug)]
pub struct KeyDef<'a> {
    name: &'a str,
    windows: &'a str,
    linux: &'a str,
    android: &'a str,
}

fn parse_keys(text: &str, variant: usize) -> Vec<KeyDef> {
    let mut keys: Vec<KeyDef> = Vec::new();

    for non_trimmed_line in text.lines() {
        let line = non_trimmed_line.trim();
        if line == "" {
            println!("Skipping Line, {:?}", non_trimmed_line)
        }

        if line.starts_with("//") {
            println!("Skipping Line, {:?}", non_trimmed_line)
        }

        let mut val_key_buffer = [""; 4];

        for (i, non_trimmed_val_keys) in line.split("=").enumerate() {
            let val_keys = non_trimmed_val_keys.trim();
            if val_keys == "" {
                eprintln!("Unexpected syntax: {:?}", line);
            }

            if i == 0 {
                val_key_buffer[0] = val_keys;
            } else if i == 1 {
                for (i, non_trimmed_val) in val_keys.split('|').enumerate() {
                    let val = non_trimmed_val.trim();
                    let mut variant_buffer = [""; 10];

                    for (i, v) in val.split('\\').enumerate() {
                        variant_buffer[i] = v;
                    }

                    let mut selected = variant_buffer[0];
                    if variant_buffer[variant] != "" {
                        selected = variant_buffer[variant];
                    }

                    val_key_buffer[i + 1] = selected;
                }
            } else {
                eprintln!("Unexpected syntax: {:?}", line);
            }
        }

        keys.push(KeyDef {
            name: val_key_buffer[0],
            windows: val_key_buffer[1],
            linux: val_key_buffer[2],
            android: val_key_buffer[3],
        })
    }
    keys
}

pub mod rs {
    use super::*;
    fn platform_parse(f: &mut Vec<u8>, keys: &Vec<KeyDef>, os: Os, cfg_: &str) -> Result<(), Error> {
        writeln!(f, "{cfg_}")?;
        writeln!(f, "#[repr(C)]")?;
        writeln!(f, "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]")?;
        writeln!(f, "/// Wraps inner os specific virtual key to make matching more easier.")?;
        writeln!(f, "/// ```no_run")?;
        writeln!(f, "/// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;")?;
        writeln!(f, "/// let key = Key(VK_ESCAPE as u64);")?;
        writeln!(f, "/// assert!(key == KEY::ESCAPE);")?;
        writeln!(f, "///")?;
        writeln!(f, "/// use ndk_sys::AKEYCODE_ESCAPE;")?;
        writeln!(f, "/// let key = Key(AKEYCODE_ESCAPE as u64);")?;
        writeln!(f, "/// assert!(key == KEY::ESCAPE);")?;
        writeln!(f, "/// ```")?;
        writeln!(f, "pub struct Key(pub u64);")?;
        writeln!(f, "")?;

        writeln!(f, "{cfg_}")?;
        writeln!(f, "impl Key {{")?;

        for key in keys {
            writeln!(f, "    pub const {name}: Self = Self({value} as u64);", name = key.name, value = key.get_value(os))?;
        }

        writeln!(f, "}}")?;
        writeln!(f, "")?;

        Ok(())
    }

    pub fn parse_debug(f: &mut Vec<u8>, keys: &Vec<KeyDef>) -> Result<(), Error> {
        writeln!(f, "impl Debug for Key {{")?;
        writeln!(f, "    #[allow(unreachable_patterns)]")?;
        writeln!(f, "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
        writeln!(f, "        match *self {{")?;

        for key in keys {
            let name = key.name;
            writeln!(f, "            Key::{name} => f.write_str(\"Key::{name}\"),",)?
        }

        writeln!(f, "            any => write!(f, \"Key({{}})\", any.0),")?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
        // writeln!(f, "")?;

        Ok(())
    }

    pub fn parse(f: &mut Vec<u8>, keys: &Vec<KeyDef>) -> Result<(), Error> {
        writeln!(f, "use std::fmt::Debug;")?;
        writeln!(f, "")?;

        let cfg_windows = "#[cfg(target_os = \"windows\")]";
        writeln!(f, "{cfg_windows}")?;
        writeln!(f, "use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;")?;
        platform_parse(f, keys, Os::Windows, cfg_windows)?;

        let cfg_linux = "#[cfg(target_os = \"linux\")]";
        writeln!(f, "{cfg_linux}")?;
        writeln!(f, "use x11::keysym::*;")?;
        platform_parse(f, keys, Os::Linux, cfg_linux)?;

        let cfg_android = "#[cfg(target_os = \"android\")]";
        writeln!(f, "{cfg_android}")?;
        writeln!(f, "use ndk_sys::*;")?;
        platform_parse(f, keys, Os::Android, cfg_android)?;

        parse_debug(f, keys)?;

        Ok(())
    }
}

pub mod c {
    use super::*;
    fn platform_parse(f: &mut Vec<u8>, keys: &Vec<KeyDef>, os: Os) -> Result<(), Error> {
        for key in keys {
            writeln!(f, "#define {name} {value}", name = key.name, value = key.get_value(os))?;
        }

        Ok(())
    }

    pub fn parse_debug(f: &mut Vec<u8>, keys: &Vec<KeyDef>) -> Result<(), Error> {
        writeln!(f, "#ifdef EXPOSED_KEY_DEBUG")?;

        writeln!(
            f,
            "    const char * dbgKey(int key) {{
        switch (key) {{"
        )?;

        for key in keys {
            let name = key.name;
            writeln!(
                f,
                "        case {name}:
            return \"{name}\";"
            )?;
        }

        writeln!(
            f,
            "        default:
            return \"Unused\";
        }}
    }}
#endif"
        )?;

        Ok(())
    }

    pub fn parse(f: &mut Vec<u8>, keys: &Vec<KeyDef>) -> Result<(), Error> {
        writeln!(f, "#ifndef EXPOSED_KEYS_H")?;
        writeln!(f, "#define EXPOSED_KEYS_H")?;

        {
            writeln!(f, "#ifdef USE_WINDOWS")?;
            writeln!(f, "#include <Windows.h>")?;
            platform_parse(f, keys, Os::Windows)?;
            writeln!(f, "#endif")?;
            writeln!(f, "")?;
        }

        {
            writeln!(f, "#ifdef USE_X11")?;
            writeln!(f, "#include <keysymdef.h>")?;
            platform_parse(f, keys, Os::Linux)?;
            writeln!(f, "#endif")?;
            writeln!(f, "")?;
        }

        {
            writeln!(f, "#ifdef USE_ANDROID")?;
            writeln!(f, "#include <keycodes.h>")?;
            platform_parse(f, keys, Os::Android)?;
            writeln!(f, "#endif")?;
            writeln!(f, "")?;
        }

        parse_debug(f, keys)?;

        writeln!(f, "#endif")?;
        Ok(())
    }
}

impl<'a> KeyDef<'a> {
    fn get_value(&self, os: Os) -> &str {
        match os {
            Os::Windows => self.windows,
            Os::Linux => self.linux,
            Os::Android => self.android,
        }
    }
}

use proc_macro::TokenStream;

#[proc_macro]
pub fn log_impl(tokens: TokenStream) -> TokenStream {
    let tokens = tokens.to_string();

    let tokens: Vec<&str> = tokens.split(',').map(|s| s.trim()).collect();
    if tokens.len() != 2 {
        panic!("Bad args");
    }

    return log_impl_(&tokens[0], &tokens[1]).parse().unwrap();
}

const TARGET_ANDROID: &str = r#"#[cfg(target_os = "android")]"#;
const NOT_TARGET_ANDROID: &str = r#"#[cfg(not(target_os = "android"))]"#;

fn log_impl_(name: &str, level: &str) -> String {
    let upper_name = name.to_uppercase();

    format!(
        r#"
        
#[macro_export]
{TARGET_ANDROID}
macro_rules! log_{name}_ {{
    ($tag:literal, $fmt:literal) => {{
        $crate::__android_log_print({level} as _, $crate::cstr!($tag), $crate::cstr!($fmt))
    }};

    ($tag:literal, $fmt:literal, $($args:expr),*) => {{
        $crate::__android_log_print(
            {level} as _,
            $crate::cstr!($tag),
            $crate::cstr!($fmt),
            $($args),*
        )
    }};

    ($tag:expr, $fmt:literal) => {{
        $crate::__android_log_print({level} as _, $crate::cstr!($tag), $crate::cstr!($fmt))
    }};

    ($tag:expr, $fmt:literal, $($args:expr),*) => {{
        $crate::__android_log_print(
            {level} as _,
            $crate::cstr!($tag),
            $crate::cstr!($fmt),
            $($args),*
        )
    }};
}}

#[macro_export]
{TARGET_ANDROID}
macro_rules! log_{name} {{
    ($tag:expr, $($arg:expr),*) => {{
        $crate::log_string({level},  $crate::cstr!($tag), format!($($arg),*))
    }};
    
    ($tag:literal, $($arg:expr)*) => {{
        $crate::log_string({level},  $crate::cstr!($tag), format!($($arg),*))
    }};
}}

#[macro_export]
{NOT_TARGET_ANDROID}
macro_rules! log_{name} {{
    ($tag:expr, $($arg:expr),*) => {{
        println!("{upper_name}: {{}} {{}}", $tag, format!($($arg),*));
    }};
    
    ($tag:literal, $($arg:expr)*) => {{
        println!("{upper_name}: {{}} {{}}", $tag, format!($($arg),*));
    }};
}}

    "#,
    )
}

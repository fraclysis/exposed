use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn c_str(tokens: TokenStream) -> TokenStream {
    for token in tokens {
        if let TokenTree::Literal(text) = token {
            let text = text.to_string();
            let text = &text[1..text.len() - 1];

            let out = format!("\"{text}\0\".as_ptr() as *const std::ffi::c_char");

            match out.parse::<TokenStream>() {
                Ok(o) => return o,
                Err(e) => panic!("Failed {out} with {e}!"),
            }
        } else if let TokenTree::Group(group) = token {
            return c_str(group.stream());
        } else {
            panic!("Only literals are supported: {token:?}")
        }
    }
    panic!("No attrib is found!")
}

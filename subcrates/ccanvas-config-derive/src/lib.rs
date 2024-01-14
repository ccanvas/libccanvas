use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CcanvasConfig)]
pub fn derive_ccanvas_config(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct being derived for
    let struct_name = &input.ident;

    // Generate the implementation of CcanvasConfig trait
    let expanded = quote! {
        impl CcanvasConfig for #struct_name {
            const CNAME: &'static str = env!("CARGO_PKG_NAME");
        }
    };

    // Convert the expanded tokens back into a TokenStream and return it
    TokenStream::from(expanded)
}

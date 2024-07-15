use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, parenthesized, parse_macro_input};

#[proc_macro_derive(Api, attributes(api))]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let mut endpoint: LitStr = syn::parse_str("\"\"").unwrap();

    if let Some(attr) = input.attrs.iter().find(|&attr| attr.path().is_ident("api")) {
        attr.parse_nested_meta(|meta| {
            if let Some(i) = meta.path.get_ident() {
                let content;
                parenthesized!(content in meta.input);
                match i.to_string().as_str() {
                    "endpoint" => {
                        endpoint = content.parse()?;
                    }
                    attr => return Err(meta.error(format!("unknown attribute {attr}"))),
                }
            }
            Ok(())
        })
            .unwrap();
    };
    let expanded = quote! {
        impl #struct_name {
            pub fn get_api() -> &'static str {
                #endpoint
            }
        }
    };
    TokenStream::from(expanded)
}

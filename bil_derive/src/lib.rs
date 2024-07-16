mod api;
use proc_macro::TokenStream;

#[proc_macro_derive(Api, attributes(api))]
pub fn derive_api(input: TokenStream) -> TokenStream {
    api::derive_api(input)
}

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn qjs(input: TokenStream) -> TokenStream {
    qjs_derive_support::qjs(proc_macro2::TokenStream::from(input))
        .unwrap()
        .into()
}

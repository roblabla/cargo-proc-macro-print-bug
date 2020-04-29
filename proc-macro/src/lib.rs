extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Hello");
    item
}
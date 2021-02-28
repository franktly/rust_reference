extern crate proc_macro;
use proc_macro::TokenStream;

/*
 * function-like procedural macros
 */

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 88}".parse().unwrap()
}

/*
 * derive macros
 */

#[proc_macro_derive(NewAnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn new_answer() -> u32 { 8888}".parse().unwrap()
}

/*
 * derive macro helper attributes
 */
#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/*
 * attribute macros
 */

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{:?}\"", attr.to_string());
    println!("item: \"{:?}\"", item.to_string());
    item
}

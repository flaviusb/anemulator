#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_non_items)]
#![feature(proc_macro_quote)]
#![feature(proc_macro_span)]

extern crate proc_macro;

#[proc_macro]
pub fn define_chip(input: TokenStream) -> TokenStream {
  panic!("Not implemented.");
}

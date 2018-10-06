#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_hygiene)]
#![feature(proc_macro_quote)]
#![feature(proc_macro_span)]

extern crate proc_macro;

#[proc_macro]
pub fn define_chip(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  panic!("Not implemented.");
}

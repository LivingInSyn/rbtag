extern crate proc_macro;

use quote::{quote};
use syn::{parse_macro_input,DeriveInput};
use chrono::prelude::*;

#[proc_macro_derive(BuildDateTime)]
pub fn build_dt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let utc: String = Utc::now().to_rfc3339();
    let expanded = quote! {
        impl BuildDateTime for #name {
            fn get_build_dt(&self) -> &'static str {
                //String::from("test")
                #utc
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

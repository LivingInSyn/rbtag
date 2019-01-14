extern crate proc_macro;

use quote::{quote};
use syn::{parse_macro_input,DeriveInput};
use chrono::prelude::*;
use std::process::Command;

#[proc_macro_derive(BuildDateTime)]
pub fn build_dt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let utc: String = Utc::now().to_rfc3339();
    let expanded = quote! {
        impl BuildDateTime for #name {
            fn get_build_timestamp(&self) -> &'static str {
                //String::from("test")
                #utc
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(BuildGitCommit)]
pub fn get_build_commit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //git rev-parse --short HEAD
    let git_command = "git rev-parse --short HEAD";
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", git_command])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(git_command)
                .output()
                .expect("failed to execute process")
    };
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expanded = quote! {
        impl BuildGitCommit for #name {
            fn get_build_commit(&self) -> &'static str {
                //String::from("test")
                #stdout
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

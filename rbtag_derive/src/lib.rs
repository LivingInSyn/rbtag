extern crate proc_macro;

use quote::{quote};
use syn::{parse_macro_input,DeriveInput};
use chrono::prelude::*;
use std::process::Command;


/// This function creates a utc datetime in rfc3339 format and returns it as a 
/// `&'static str`
#[proc_macro_derive(BuildDateTime)]
pub fn build_dt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("build dt called");
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let utc: String = Utc::now().to_rfc3339();
    let expanded = quote! {
        impl BuildDateTime for #name {
            fn get_build_timestamp(&self) -> &'static str {
                #utc
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn get_commit_info() -> String {
    let git_commit_command = "git show -s --format=%h-%ct";
    let git_dirty_command = "git diff-index --quiet HEAD --";
    // get the git commit/datetime info
    let commit_output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", git_commit_command])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(git_commit_command)
                .output()
                .expect("failed to execute process")
    };
    // get the dirty status
    let dirty_output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", git_dirty_command])
                .status()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(git_dirty_command)
                .status()
                .expect("failed to execute process")
    };
    let dirty_string = match dirty_output.success() {
        true => "clean",
        false => "dirty"
    };

    format!("{}-{}", String::from_utf8_lossy(&commit_output.stdout), dirty_string)
}

/// This function gets the current short git commit hash from windows
/// or *nix and returns it as a `&'static str`
#[proc_macro_derive(BuildGitCommit)]
pub fn get_build_commit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("build commit called");
    let gitoutput = get_commit_info();

    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    //let stdout = String::from_utf8_lossy(&output.stdout);
    let expanded = quote! {
        impl BuildGitCommit for #name {
            fn get_build_commit(&self) -> &'static str {
                //String::from("test")
                #gitoutput
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

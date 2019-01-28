extern crate proc_macro;

use quote::{quote};
use syn::{parse_macro_input,DeriveInput};
use std::process::Command;
use std::env;

fn get_time_info() -> String {
    //check if the environmental variable is set
    let key = "SOURCE_DATE_EPOCH";
    if let Some(sde_val) = env::var_os(key) {
        if let Some(os_str) = sde_val.to_str() {
            let mut oss = os_str.to_string();
            if oss.ends_with("\n") {
                oss.pop();
                oss
            } else {
                oss
            }
        } else {
            String::new()
        }
    } else {
        let git_commit_command = "git show -s --format=%ct";
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
        let mut gco = String::from_utf8_lossy(&commit_output.stdout).to_string();
        if gco.ends_with("\n") {
            gco.pop();
            gco
        } else {
            gco
        }
    }
}

/// This function creates a utc datetime in rfc3339 format and returns it as a 
/// `&'static str`
#[proc_macro_derive(BuildDateTime)]
pub fn build_dt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let time: String = get_time_info();
    let expanded = quote! {
        impl BuildDateTime for #name {
            fn get_build_timestamp(&self) -> &'static str {
                #time
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn get_commit_info() -> String {
    let git_commit_command = "git show -s --format=%h";
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
    let mut gitoutput = String::from_utf8_lossy(&commit_output.stdout).to_string();
    //if gitoutput ends with a newline, pop it
    gitoutput.pop();
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
    format!("{}-{}", gitoutput, dirty_string)
}

/// This function gets the current short git commit hash from windows
/// or *nix and returns it as a `&'static str`
#[proc_macro_derive(BuildInfo)]
pub fn get_build_commit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let gitoutput = get_commit_info();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    //let stdout = String::from_utf8_lossy(&output.stdout);
    let expanded = quote! {
        impl BuildInfo for #name {
            fn get_build_commit(&self) -> &'static str {
                //String::from("test")
                #gitoutput
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

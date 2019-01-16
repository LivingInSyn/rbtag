// reexport of rbtag_derive
pub use rbtag_derive::*;

/// Trait for returning the build datetime stamp in UTC
pub trait BuildDateTime {
    fn get_build_timestamp(&self) -> &'static str;
}

/// Trait for returning the build commit short hash
pub trait BuildInfo {
    fn get_build_commit(&self) -> &'static str;
}
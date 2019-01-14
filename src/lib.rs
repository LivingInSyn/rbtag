pub use rbtag_derive::*;

pub trait BuildDateTime {
    fn get_build_timestamp(&self) -> &'static str;
}
pub trait BuildGitCommit {
    fn get_build_commit(&self) -> &'static str;
}
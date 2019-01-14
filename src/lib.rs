pub use rbtag_derive::*;

pub trait BuildDateTime {
    fn get_build_dt(&self) -> &'static str;
}
use rbtag::{BuildDateTime, BuildInfo};

#[derive(BuildDateTime, BuildInfo)]
struct BuildTag;

fn main() {
    println!("{}", BuildTag{}.get_build_timestamp());
    println!("{}", BuildTag{}.get_build_commit());
}

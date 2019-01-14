use rbtag::{BuildDateTime, BuildGitCommit};

#[derive(BuildDateTime, BuildGitCommit)]
struct BuildTag;

fn main() {
    println!("{}", BuildTag{}.get_build_timestamp());
    println!("{}", BuildTag{}.get_build_commit());
}

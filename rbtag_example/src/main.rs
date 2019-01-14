use rbtag::BuildDateTime;

#[derive(BuildDateTime)]
struct BuildTag;

fn main() {
    println!("{}", BuildTag{}.get_build_dt());
}

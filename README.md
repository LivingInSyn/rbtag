# RBTAG
rbtag is a procedural macro designed to add build time information or git commit information to your crate or project. Just add `#[derive(BuildDateTime)]` and/or `#[derive(BuildInfo)]` to a struct and call `get_build_timestamp()` and/or `.get_build_commit()` on that struct to get a static datetime string for the UTC date when it was built

## Example
```rust
use rbtag::{BuildDateTime, BuildInfo};

#[derive(BuildDateTime, BuildInfo)]
struct BuildTag;

fn main() {
    println!("{}", BuildTag{}.get_build_timestamp());
    println!("{}", BuildTag{}.get_build_commit());
}

```
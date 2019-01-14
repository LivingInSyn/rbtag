# RBTAG
rbtag is a procedural macro designed to add build time information to your crate or project. Just add `#[derive(BuildDateTime)]` to a struct and call `get_build_dt()` on that struct to get a static datetime string for the UTC date when it was built

## Example
```rust
use rbtag::BuildDateTime;

#[derive(BuildDateTime)]
struct BuildTag;

fn main() {
    println!("{}", BuildTag{}.get_build_dt());
}

```
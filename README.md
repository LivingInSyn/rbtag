# RBTAG
rbtag is a procedural macro designed to add build time information or git commit information to your crate or project. 

## Git Commit Info 
To use the Git commit Info functionality just add `#[derive(BuildInfo)]` to a struct and call `.get_build_commit()` on it. The output looks like the following:

```shell
eaba6e2-dirty
```

OR

```shell
eaba6e2-clean
```

Where clean vs dirty indicates the presence of uncommited changes to tracked files in the repo.

**NOTE** If you have this code continue to return 'dirty', run `git diff` to see what files are causing the issue.

## Build Time Info

To use the Git commit Info functionality just add `#[derive(BuildDateTime)]` to a struct and call `.get_build_timestamp()` on it. In order to comply with https://reproducible-builds.org/, two sources of time are possibly used the following precedence

1) If the environmental variable `SOURCE_DATE_EPOCH` is set, the value in this variable will be used
2) If the environmental variable is **NOT** set, the timestamp of the current git commit is used and displayed as a UNIX timestamp with no fractional component

### Sample output:
The following is an example of running the below 'example' code with and without an environmental variable set
```shell
#$ cargo clean && env SOURCE_DATE_EPOCH='12345678909' cargo run
12345678901
90c2266-dirty
#? cargo clean && cargo run
1547647585
90c2266-dirty
```

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
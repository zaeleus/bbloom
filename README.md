# bbloom

**bbloom** is a Rust library that provides an implementation of a Bloom filter.

## Install

bbloom is not published to [crates.io]. Add the dependency to `Cargo.toml`
using the git repository.

```toml
bbloom = { git = "https://github.com/zaeleus/bbloom.git" }
```

[crates.io]: https://crates.io/

## Examples

```rust
use bbloom::BloomFilter;

// false positive probability
const P: f64 = 0.0001;

// expected number of inserted values
const N: usize = 64;

fn main() {
    let mut filter = BloomFilter::from_fpp(P, N);

    filter.insert("a");
    filter.insert("b");

    assert!(filter.contains("a"));
    assert!(filter.contains("b"));
    assert!(!filter.contains("c"));
}
```

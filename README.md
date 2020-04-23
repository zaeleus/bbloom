# bbloom

[![GitHub Actions status](https://github.com/zaeleus/bbloom/workflows/CI/badge.svg)](https://github.com/zaeleus/bbloom/actions)

**bbloom** is a Rust library that provides an implementation of a Bloom filter.

A scalable Bloom filter is also included, which is a variant that dynamically
grows to ensure a given false positive probability.

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

## References

  * Paulo Sérgio Almeida, Carlos Baquero, Nuno Preguiça, and David Hutchison.
    2007. Scalable Bloom Filters. Inf. Process. Lett. 101, 6 (March 2007),
    255-261. DOI=http://dx.doi.org/10.1016/j.ipl.2006.10.007

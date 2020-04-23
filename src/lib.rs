//! # bbloom
//!
//! **bbloom** is a Rust library that provides an implementation of a Bloom filter.
//!
//! A scalable Bloom filter is also included, which is a variant that dynamically grows to ensure a
//! given false positive probability.
//!
//! ## Examples
//!
//! ```
//! use bbloom::BloomFilter;
//!
//! // false positive probability
//! const P: f64 = 0.0001;
//! // expected number of inserted values
//! const N: usize = 64;
//!
//! let mut filter = BloomFilter::from_fpp(P, N);
//!
//! filter.insert("a");
//! filter.insert("b");
//!
//! assert!(filter.contains("a"));
//! assert!(filter.contains("b"));
//! assert!(!filter.contains("c"));
//! ```

mod bloom_filter;
mod double_hasher;
mod scalable_bloom_filter;

pub use self::{bloom_filter::BloomFilter, scalable_bloom_filter::ScalableBloomFilter};

type DefaultHashBuilder = std::collections::hash_map::RandomState;

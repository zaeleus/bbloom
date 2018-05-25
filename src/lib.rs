extern crate bit_vec;

pub use self::bloom_filter::BloomFilter;
pub use self::scalable_bloom_filter::ScalableBloomFilter;

pub mod bloom_filter;
pub mod double_hasher;
pub mod scalable_bloom_filter;

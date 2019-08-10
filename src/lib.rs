pub use self::bloom_filter::BloomFilter;
pub use self::scalable_bloom_filter::ScalableBloomFilter;

mod bloom_filter;
mod double_hasher;
mod scalable_bloom_filter;

type DefaultHashBuilder = std::collections::hash_map::RandomState;

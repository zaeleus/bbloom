use std::collections::hash_map::RandomState;
use std::f64;
use std::hash::Hash;

use bit_vec::BitVec;

use crate::double_hasher::DoubleHasher;

/// A Bloom filter is a probabilistic data structure to test whether an element may be in a set or
/// definitely not in a set.
pub struct BloomFilter {
    bits: BitVec,

    // bit array length
    m: usize,
    // number of inserted elements
    n: usize,
    // number of hash functions
    k: usize,

    builder_1: RandomState,
    builder_2: RandomState,
}

impl BloomFilter {
    /// Creates a new bloom filter that targets a false positive probability `p` ([0.0, 1.0]) with
    /// an expected number of inserted elements `n`.
    ///
    /// The optimal size of the bit array `m` and number of hash functions `k` are automatically
    /// calculated. See "[Optimal number of hash functions][1]".
    ///
    /// [1]: https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let _filter = BloomFilter::from_fpp(0.0001, 64);
    /// ```
    pub fn from_fpp(p: f64, n: usize) -> BloomFilter {
        let m = optimal_required_bits(p, n);
        let k = optimal_number_of_hash_functions(m, n);
        BloomFilter::new(m, k)
    }

    /// Creates a new bloom filter with a predetermined bit array size `m` and number of hash
    /// functions `k`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let _filter = BloomFilter::new(1227, 14);
    /// ```
    pub fn new(m: usize, k: usize) -> BloomFilter {
        BloomFilter {
            bits: BitVec::from_elem(m, false),
            m,
            n: 0,
            k,
            builder_1: RandomState::new(),
            builder_2: RandomState::new(),
        }
    }

    /// Returns the size of the bit array `m`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let filter = BloomFilter::new(1227, 14);
    /// assert_eq!(filter.capacity(), 1227);
    /// ```
    pub fn capacity(&self) -> usize {
        self.m
    }

    /// Tests whether an element may be in the filter or definitely not in the filter.
    ///
    /// Remember that false positives can occur, meaning that if this returns `true`, there is only
    /// a possibility that the element is in the filter. If this returns `false`, the element is
    /// definitely not in the filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::from_fpp(0.0001, 64);
    /// filter.insert("a");
    /// filter.insert("b");
    ///
    /// assert!(filter.contains("a"));
    /// assert!(filter.contains("b"));
    /// assert!(!filter.contains("c"));
    /// ```
    pub fn contains<H: Hash + ?Sized>(&self, key: &H) -> bool {
        let hasher = self.build_hasher(key);

        for hash in hasher.take(self.k) {
            let i = (hash as usize) % self.m;

            if !self.bits[i] {
                return false;
            }
        }

        true
    }

    /// Adds a value to the bloom filter.
    ///
    /// Returns whether the value is already (maybe) in the filter or not. Duplicate values do not
    /// affect the load factor.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::from_fpp(0.0001, 64);
    /// assert!(filter.insert("a"));
    /// assert!(filter.insert("b"));
    /// assert!(!filter.insert("b"));
    /// ```
    pub fn insert<H: Hash + ?Sized>(&mut self, key: &H) -> bool {
        let mut present = true;

        let hasher = self.build_hasher(key);

        for hash in hasher.take(self.k) {
            let i = (hash as usize) % self.m;

            if !self.bits[i] {
                present = false;
                self.bits.set(i, true);
            }
        }

        if !present {
            self.n += 1;
        }

        !present
    }

    /// Returns the number of elements `n` in the filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::from_fpp(0.0001, 64);
    /// assert_eq!(filter.len(), 0);
    ///
    /// filter.insert("a");
    /// assert_eq!(filter.len(), 1);
    ///
    /// filter.insert("b");
    /// assert_eq!(filter.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns `true` if the bloom filter contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let mut filter = BloomFilter::from_fpp(0.0001, 64);
    /// assert!(filter.is_empty());
    ///
    /// filter.insert("a");
    /// assert!(!filter.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn build_hasher<H>(&self, key: &H) -> DoubleHasher
    where
        H: Hash + ?Sized,
    {
        DoubleHasher::new(key, &self.builder_1, &self.builder_2)
    }
}

// Calculates the optimal size of the bit array given a target false positive probability `p`
// ([0.0, 1.0]) and the expected number of inserted elements `n`.
fn optimal_required_bits(p: f64, n: usize) -> usize {
    let ln_2 = f64::consts::LN_2;
    let n = n as f64;
    let m = -(n * p.ln()) / (ln_2 * ln_2);
    m.ceil() as usize
}

#[test]
fn test_optimal_required_bits() {
    let p = 0.01;
    let n = 128;
    let m = optimal_required_bits(p, n);
    assert_eq!(m, 1227);
}

// Calculates the optimal number of hash functions given the size of the bit array `m` and the
// expected number of inserted elements `n`.
fn optimal_number_of_hash_functions(m: usize, n: usize) -> usize {
    let m = m as f64;
    let n = n as f64;
    let k = m / n * f64::consts::LN_2;
    k.ceil() as usize
}

#[test]
fn test_optimal_number_of_hash_functions() {
    let m = 1227;
    let n = 128;
    let k = optimal_number_of_hash_functions(m, n);
    assert_eq!(k, 7);
}

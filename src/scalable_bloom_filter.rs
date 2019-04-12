use std::hash::Hash;

use crate::BloomFilter;

// growth factor `s`
const GROWTH_FACTOR: usize = 2;
// tightening ratio `r`
const TIGHTENING_RATIO: f64 = 0.85;

/// A scalable Bloom filter is a variant of a Bloom filter that can adapt to to the number of
/// elements inserted into the filter, targetting a given false positive probability. This is
/// effectively done by layering bloom filters with larger capacities.
///
/// This implements the ideas described in "[Scalable Bloom Filters]" (2007) by Almeida, Paulo
/// Sérgio, et al.
///
/// [Scalable Bloom Filters]: https://dl.acm.org/citation.cfm?id=1224501
pub struct ScalableBloomFilter {
    // total number of elements inserted
    n: usize,
    // total capacity of all filters
    total_capacity: usize,
    // a list of all filters in order they were created
    filters: Vec<BloomFilter>,
    // the (tightened) false positive probably of the last created filter
    last_fpp: f64,
}

impl ScalableBloomFilter {
    /// Creates a new scalable Bloom filter that targets a false positive probability `p` ([0.0,
    /// 1.0]) with an initial expected number of inserted elements `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::ScalableBloomFilter;
    /// let _filter = ScalableBloomFilter::new(0.0001, 64);
    /// ```
    pub fn new(p: f64, n: usize) -> ScalableBloomFilter {
        ScalableBloomFilter {
            n: 0,
            total_capacity: n,
            filters: vec![BloomFilter::from_fpp(p, n)],
            last_fpp: p,
        }
    }

    /// Tests all filters for whether an element may be in the filter or definitely not in the filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::ScalableBloomFilter;
    ///
    /// let mut filter = ScalableBloomFilter::new(0.0001, 64);
    ///
    /// filter.insert("a");
    ///
    /// assert!(filter.contains("a"));
    /// assert!(!filter.contains("b"));
    /// ```
    pub fn contains<H: Hash + ?Sized>(&self, key: &H) -> bool {
        self.filters.iter().any(|f| f.contains(key))
    }

    /// Adds a value to the bloom filter.
    ///
    /// Returns whether the value is already (maybe) in the _last_ filter or not. Duplicate values
    /// may be present in the scalable Bloom filter but not in the last filter. When a duplicate
    /// value is in the last filter, it does not affect the load factor.
    pub fn insert<H: Hash + ?Sized>(&mut self, key: &H) -> bool {
        if self.n >= self.total_capacity {
            self.grow();
        }

        let i = self.filters.len() - 1;
        let filter = &mut self.filters[i];
        let inserted = filter.insert(key);

        if inserted {
            self.n += 1;
        }

        inserted
    }

    /// Adds a value to a Bloom filter if it is not already present.
    ///
    /// When there are > 1 filters, this is only slightly faster than calling both `contains` and
    /// `insert`, as the last filter does not have to be checked twice.
    ///
    /// Returns whether the value is (maybe) in the filter or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::ScalableBloomFilter;
    ///
    /// let mut filter = ScalableBloomFilter::new(0.0001, 64);
    ///
    /// assert!(!filter.contains_or_insert("a"));
    /// assert!(!filter.contains_or_insert("b"));
    /// assert!(filter.contains_or_insert("b"));
    /// ```
    pub fn contains_or_insert<H: Hash + ?Sized>(&mut self, key: &H) -> bool {
        let n = if self.filters.len() == 1 {
            1
        } else {
            self.filters.len() - 1
        };

        if self.filters.iter().take(n).any(|f| f.contains(key)) {
            true
        } else {
            !self.insert(key)
        }
    }

    fn grow(&mut self) {
        let p = self.last_fpp * TIGHTENING_RATIO;
        let n = self.total_capacity * GROWTH_FACTOR;

        let filter = BloomFilter::from_fpp(p, n);
        self.filters.push(filter);

        self.total_capacity += n;
        self.last_fpp = p;
    }
}

use BloomFilter;

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
    pub fn new(p: f64, n: usize) -> ScalableBloomFilter {
        ScalableBloomFilter {
            n: 0,
            total_capacity: n,
            filters: vec![BloomFilter::from_fpp(p, n)],
            last_fpp: p,
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.filters.iter().any(|f| f.contains(key))
    }

    pub fn insert(&mut self, key: &str) {
        if self.n >= self.total_capacity {
            self.grow();
        }

        let i = self.filters.len() - 1;
        let filter = &mut self.filters[i];
        filter.insert(key);

        self.n += 1;
    }

    pub fn contains_or_insert(&mut self, key: &str) -> bool {
        let n = if self.filters.len() == 1 { 1 } else { self.filters.len() - 1 };

        if self.filters.iter().take(n).any(|f| f.contains(key)) {
            true
        } else {
            self.insert(key);
            false
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

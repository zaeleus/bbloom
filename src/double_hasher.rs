use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

// https://en.wikipedia.org/wiki/Double_hashing
pub struct DoubleHasher {
    h1: u64,
    h2: u64,
    i: usize,
}

impl DoubleHasher {
    pub fn new<H: Hash + ?Sized>(
        key: &H,
        state_1: &RandomState,
        state_2: &RandomState,
    ) -> DoubleHasher {
        let mut hasher = state_1.build_hasher();
        key.hash(&mut hasher);
        let h1 = hasher.finish();

        let mut hasher = state_2.build_hasher();
        key.hash(&mut hasher);
        let h2 = hasher.finish();

        DoubleHasher { h1, h2, i: 0 }
    }
}

impl Iterator for DoubleHasher {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let hash = match self.i {
            0 => self.h1,
            1 => self.h2,
            _ => {
                let i = self.i as u64;
                self.h1.wrapping_add(i.wrapping_mul(self.h2))
            }
        };

        self.i += 1;

        Some(hash)
    }
}

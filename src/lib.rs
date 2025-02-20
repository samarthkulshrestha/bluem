extern crate bit_vec;

use bit_vec::BitVec;
use core::f64;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;


pub struct BloomFilter<T: ?Sized> {
    bitmap: BitVec,
    optimal_m: usize,
    optimal_k: u32,
    hashers: [DefaultHasher; 2],
    _marker: PhantomData<T>
}

impl<T: ?Sized> BloomFilter<T> {
    // create a new BloomFilter that expects to store `items_count`
    // membership with a false positive rate of the value specified in `fp_rate`.
    pub fn new(items_count: usize, fp_rate: f64) -> Self {
        let optimal_m = Self::bitmap_size(items_count, fp_rate);
        let optimal_k = Self::optimal_k(fp_rate);
        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        BloomFilter {
            bitmap: BitVec::from_elem(optimal_m as usize, false),
            optimal_m,
            optimal_k,
            hashers,
            _marker: PhantomData
        }
    }

    // insert items into the set.
    pub fn insert(&mut self, item: &T)
    where
        T: Hash,
    {
        let (h1, h2) = self.hash_kernel(item);

        for k_i in 0..self.optimal_k {
            let index = self.get_index(h1, h2, k_i as u64);
            self.bitmap.set(index, true)
        }
    }

    // check if an item is present in the set.
    // false positives are possible, but not false negatives.
    pub fn contains(&mut self, item: &T) -> bool
    where
        T: Hash,
    {
        let (h1, h2) = self.hash_kernel(item);

        for k_i in 0..self.optimal_k {
            let index = self.get_index(h1, h2, k_i as u64);

            if !self.bitmap.get(index).unwrap() {
                return false;
            }
        }

        true
    }

    // get the index from the hash value of `k_i`.
    fn get_index(&self, h1: u64, h2: u64, k_i: u64) -> usize {
        h1.wrapping_add((k_i).wrapping_mul(h2)) as usize % self.optimal_m
    }

    // calculate the size of `bitmap`.
    // the size of bitmap depends on the target false positive probability
    // and the number of items in the set.
    fn bitmap_size(items_count: usize, fp_rate: f64) -> usize {
        let ln2_2 = core::f64::consts::LN_2 * core::f64::consts::LN_2;
        ((-1.0f64 * items_count as f64 * fp_rate.ln()) / ln2_2).ceil() as usize
    }

    // calculate the number of hash functions.
    // the required number of hash functions only depends on the target
    // false positive probability.
    fn optimal_k(fp_rate: f64) -> u32 {
        ((-1.0f64 * fp_rate.ln()) / core::f64::consts::LN_2).ceil() as u32
    }

    // calculate two hash values from which the k hashes are derived.
    fn hash_kernel(&self, item: &T) -> (u64, u64)
    where
        T: Hash,
    {
        let hasher1 = &mut self.hashers[0].clone();
        let hasher2 = &mut self.hashers[1].clone();

        item.hash(hasher1);
        item.hash(hasher2);

        let hash1 = hasher1.finish();
        let hash2 = hasher2.finish();

        (hash1, hash2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut bloom = BloomFilter::new(100, 0.01);
        bloom.insert("item");
        assert!(bloom.contains("item"));
    }

    #[test]
    fn check_and_insert() {
        let mut bloom = BloomFilter::new(100, 0.01);
        assert!(!bloom.contains("item_1"));
        assert!(!bloom.contains("item_2"));
        bloom.insert("item_1");
        assert!(bloom.contains("item_1"));
        assert!(!bloom.contains("item_2"));
    }
}

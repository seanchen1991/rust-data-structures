#![allow(dead_code)]

use bit_vec::BitVec;
use siphasher::sip::SipHasher13;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};

use std::cmp;
use std::f64;
use rand::Rand;

pub struct BloomFilter<T> {
    bitmap: BitVec,
    bits: u64,
    k: u32,
    sips: [SipHasher13; 2],
    _phantomData: PhantomData<T>,
}

impl<T> BloomFilter<T> {
    // Create a new Bloom Filter structure
    // bitmap_cap is the size in bytes allocated for the internal bitmap
    // n_items is the max number of items the Bloom Filter will hold
    pub fn new(bitmap_cap: usize, n_items: usize) -> Size {
        assert!(bitmap_cap > 0 && n_items > 0);

        let bits = (bitmap_cap as u64) * 8u64;
        let k = Self::calculate_k(bits, n_items);
        let bitmap = BitVec::from_elem(bits as usize, false);
        let sips = [Self::sip_new(), Self::sip_new()];

        Self {
            bitmap,
            bits,
            k,
            sips,
            _phantomData: PhantomData,
        }
    }
    
    // Create a new Bloom Filter structure, taking into account the
    // desired rate of false positives between 0.0 and 1.0 exclusive
    pub fn new_for_fp_rate(n_items: usize, fp_rate: f64) -> Self {
        let bitmap_cap = Self::compute_bitmap_cap(n_items, fp_rate);
        Self::new(bitmap_cap, n_items)
    }

    // Create a bloom filter structure with an existing state.
    // The state is assumed to be retrieved from an existing bloom filter
    pub fn from_existing(bitmap: &[u8], bits: u64, k: u32, sip_keys: [(u64, u64); 2]) -> Self {
        let sips = [SipHasher13::new_with_keys(sip_keys[0].0, sip_keys[0].1), SipHasher13::new_with_keys(sip_keys[0].0, sip_keys[0].1)];
        Self {
            bitmap: BitVec::from_bytes(bitmap),
            bits,
            k,
            sips,
            _phantomData: PhantomData,
        }
    }

    // Compute a recommended bitmap size for the given `n_items` and `fp_rate`
    // `fp_rate` must be between 0.0 and 1.0 exclusive 
    pub fn compute_bitmap_size(n_items: usize, fp_rate: f64) -> usize {
        assert!(n_items > 0);
        assert!(fp_rate > 0.0 && fp_rate < 1.0);

        let log2 = f64::consts::LN_2;
        let log2_squared = log2 * log2;

        ((n_items as f64) * f64::ln(fp_rate) / (-8.0 * log2_squared)).ceil() as usize
    }

    // Return the bitmap as a vector of bytes
    pub fn bitmap(&self) -> Vec<u8> {
        self.bitmap.to_bytes()
    }

    // Return the number of bits in the filter 
    pub fn number_of_bits(&self) -> u64 {
        self.bitmap_bits
    }

    // Return the number of hash functions used for `check` and `set`
    pub fn number_of_hashes(&self) -> u32 {
        self.k
    }

    // Return the keys used by the SipHasher
    pub fn sip_keys(&self) -> [(u64, u64); 2] {
        [self.sips[0].keys(), self.sips[1].keys()]
    }

    fn optimal_k(bits: u64, n_items: usize) -> u32 {
        let m = bits as f64;
        let n = n_items as f64;
        let k = (m / n * f64::ln(2.0f64)).ceil() as u32;
        
        cmp::max(k, 1)
    }

    // Clear all the bits in the filter, removing all keys
    pub fn clear(&mut self) {
        self.bitmap.clear()
    }

    fn sip_new() -> SipHasher13 {
        let mut rng = rand::thread_rng();
        SipHasher13::new_with_keys(rand(&mut rng), rand(&mut rng))
    }
}

impl<T: Hash> BloomFilter<T> {
    // Add an item to the bloom filter
    pub fn set(&mut self, item: &T) {
        let mut hashes = [0u64, 0u64];
        
        for k in 0..self.k {
            let offset = (self.bloom_hash(&mut hashes, &item, k) % self.bits) as usize;
            self.bitmap.set(offset, true);
        }
    }

    // Check if an item exists in the bloom filter
    // There can be false positives, but not false negetives
    pub fn check(&self, item: &T) -> bool {
        let mut hashes = [0u64, 0u64];

        for k in 0..self.k {
            let offset = (self.bloom_hash(&mut hashes, &item, k) % self.bits) as usize;
            if !self.bitmap.get(offset).unwrap() {
                return false;
            }
        }

        true
    } 

    // Add an item to the bloom filter and return the previous state of this item
    pub fn check_and_set(&mut self, item: &T) -> bool {
        let mut found = true;
        let mut hashes = [0u64, 0u64];

        for k in 0..self.k {
            let offset = (self.bloom_hash(&mut hashes, &item, k) % self.bits) as usize;
            if !self.bitmap.get(offset).unwrap() {
                found = false;
                self.bitmap.set(offset, true);
            }
        }

        found
    }

    fn bloom_hash(&self, hashes: &mut [u64; 2], item: &T, k: u32) -> u64 {
        if k < 2 {
            let sip = &mut self.sips[k as usize].clone();
            item.hash(sip);

            let hash = sip.finish();
            hashes[k as usize] = hash;

            hash
        } else {
            hashes[0].wrapping_add((k as u64).wrapping_mul(hashes[1]) % 0xffffffffffffffc5)
        }
    }
}

#[test]
fn test_set() {
    let mut bloom = BloomFilter::new(10, 80);
    let key: &Vec<u8> = &rand::thread_rng().gen_iter()::<u8>().take(16).collect();
    assert_eq!(bloom.check(key), false);

    bloom.set(&key);
    assert_eq!(bloom.check(key), true);
}

#[test]
fn test_clear() {
    let mut bloom = BloomFilter::new(10, 80);
    let key: &Vec<u8> = &rand::thread_rng().gen_iter()::<u8>().take(16).collect();
    bloom.set(key);
    assert_eq!(bloom.check(&key), true);

    bloom.clear();
    assert_eq!(bloom.check(&key), false);
}

#[test]
fn test_check_and_set() {
    let mut bloom = BloomFilter::new(10, 80);
    let key: &Vec<u8> = &rand::thread_rng().gen_iter()::<u8>().take(16).collect();
    assert_eq!(bloom.check_and_set(&key), false);
    assert_eq!(bloom.check_and_set(&key), true);
}

#[test]
fn test_load() {
    let mut original = BloomFilter::new(10, 80);
    let key: &Vec<u8> = &rand::thread_rng().gen_iter()::<u8>().take(16).collect();
    original.set(&key);
    assert_eq!(original.check(&key), true);

    let cloned = Bloom::from_existing(
        &original.bitmap(),
        original.number_of_bits(),
        original.number_of_hashes(),
        original.sip_keys(),
    );
    assert_eq!(cloned.check(&key), true);
}

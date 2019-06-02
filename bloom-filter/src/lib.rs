#![allow(dead_code)]

use bit_vec::BitVec;
use siphasher::sip::SipHasher13;
use std::marker::PhantomData;

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
}


#![allow(dead_code)]
#![feature(box_into_raw_non_null)]

use std::mem;
use std::ptr::NonNull;
use pennant::Pennant;

/// A Bag is a unordered set data structure that exhibits logarithmic
/// insertions, unions, and splits. It achieves this by leveraging a
/// binary tree data structure called a Pennant, where each Pennant has
/// a degree k such that the number of elements in the Pennant is 2^(k+1).
/// In order to hold any arbitrary number of elements, the Bag has an 
/// array of Pennants called the spine. The Bag maintains the invariant
/// that for some Pennant in one of the slots of the spine, the Pennant's
/// k value matches the array index it resides in.
pub struct Bag<T> {
    spine: Vec<Option<NonNull<Pennant<T>>>>,
    capacity: usize,
    count: usize,
}

impl<T> Bag<T> {
    /// Initializes a new empty bag whose spine defaults to a max degree of 10
    pub fn new() -> Self {
        Bag {
            spine: vec![None; 8],
            capacity: 255,
            count: 0,
        }
    }

    /// Initializes a new empty bag whose spine has the specified max k value
    pub fn with_degree(k: usize) -> Self {
        let degree: u32 = (k + 1) as u32;
        Bag {
            spine: vec![None; k],
            capacity: (i32::pow(2, degree) - 1) as usize,
            count: 0,
        }
    }

    /// Returns the maximum number of elements that the Bag can
    /// hold without re-allocating 
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.count
    }

    /// Inserts the given element into the Bag
    pub fn insert(&mut self, element: T) {
        let new_pennant = Box::new(Pennant::new(element));
        self.insert_pennant(new_pennant, 0);
    }

    /// Inserts a Pennant at the given array index of the spine.
    /// If there already exists another Pennant at the index where the
    /// input Pennant should go, the two Pennants are combined and then
    /// we attempt to insert the combined Pennant at the next slot in
    /// the spine.
    fn insert_pennant(&mut self, mut pennant: Box<Pennant<T>>, index: usize) {
        if index == self.spine.len() {
            let new_len: u32 = (index * 2 + 1) as u32;
            self.spine.resize_with(new_len as usize, || { None });
            self.capacity = (i32::pow(2, new_len) - 1) as usize;
        }

        match self.spine[index] {
            None => {
                self.count += pennant.len();
                self.spine[index].replace(Box::into_raw_non_null(pennant));
                return;
            },
            Some(p) => {
                let other;
                unsafe {
                    other = Box::from_raw(p.as_ptr());
                }
                self.count -= other.len();
                self.spine[index] = None;
                pennant.combine(other);
                self.insert_pennant(pennant, index + 1);
            }
        }
    }

    /// Unions the Bag with the input Bag, resulting in a single
    /// Bag that contains all the elements from each Bag
    pub fn union(&mut self, other: Bag<T>) {
        let len = other.len();

        for option in other.spine {
            match option {
                None => continue,
                Some(p) => {
                    let to_insert;
                    unsafe {
                        to_insert = Box::from_raw(p.as_ptr());
                    }
                    let k: usize = to_insert.k as usize;
                    self.insert_pennant(to_insert, k);
                }
            }
        }

        self.count += len;
    }

    /// Splits the Bag into two roughly equally-sized Bags
    /// Returns the Bag if it was successfully split, or None 
    /// if it could not be split (i.e. trying to split a Bag 
    /// that contains 0 or 1 elements)
    /// Note that when splitting a Bag with an odd number of 
    /// elements, the original Bag holds the remainder element
    pub fn split(&mut self) -> Bag<T> {
        let count = self.count;
        if count <= 1 {
            return mem::replace(self, Self::new());
        }

        let len = self.spine.len();
        let mut spare = Bag::with_degree(len);

        for i in 0..len {
            let current = mem::replace(&mut self.spine[i], None);

            match current {
                None => continue,
                Some(p) => {
                    let mut pennant;
                    unsafe {
                        pennant = Box::from_raw(p.as_ptr());
                    }
                    self.count -= pennant.len();

                    if let Some(other_pennant) = pennant.split() {
                        let other_pennant_k: usize = other_pennant.degree() as usize;
                        spare.insert_pennant(other_pennant, other_pennant_k);
                    }

                    let self_k: usize = pennant.degree() as usize;
                    self.insert_pennant(pennant, self_k);
                }
            }
        }

        spare
    }
}

#[test]
fn test_inserting_into_empty_bag() {
    let mut bag = Bag::with_degree(2);
    bag.insert("Mercury");

    assert_eq!(bag.len(), 1);
    assert!(bag.spine[0].is_some());
}

#[test]
fn test_inserting_into_nonempty_bag() {
    let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");

    assert_eq!(bag.len(), 2);
    assert!(bag.spine[0].is_none());
    assert!(bag.spine[1].is_some());

    bag.insert("Earth");

    assert_eq!(bag.len(), 3);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[1].is_some());

    bag.insert("Mars");

    assert_eq!(bag.len(), 4);
    assert!(bag.spine[0].is_none());
    assert!(bag.spine[1].is_none());
    assert!(bag.spine[2].is_some());

    bag.insert("Jupiter");

    assert_eq!(bag.len(), 5);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[1].is_none());
    assert!(bag.spine[2].is_some());
}

#[test]
fn test_union_with_empty_bags() {
   let mut bag: Bag<i32> = Bag::new();
   let empty = Bag::new();
   bag.union(empty);

   assert_eq!(bag.count, 0);
   assert!(bag.spine.iter().all(|x| x.is_none()));
}

#[test]
fn test_union_with_one_nonempty_bag_and_one_empty_bag() {
    let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");

    let empty = Bag::with_degree(2);

    bag.union(empty);

    assert_eq!(bag.len(), 2);
    assert!(bag.spine[1].is_some());
}

fn test_union_with_nonempty_bags() {
    let mut bag = Bag::new();
    bag.insert("Mercury");
    bag.insert("Venus");

    let mut other = Bag::new();
    other.insert("Earth");
    other.insert("Mars");
    other.insert("Jupiter");
    other.insert("Saturn");
    other.insert("Uranus");
    other.insert("Neptune");
    other.insert("Pluto");

    bag.union(other);

    assert_eq!(bag.len(), 9);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[3].is_some());
}

#[test]
fn test_splitting_empty_bag() {
    let mut bag: Bag<i32> = Bag::with_degree(2);
    let other_bag = bag.split();

    assert_eq!(other_bag.len(), 0);
}

#[test]
fn test_splitting_bag_with_one_element() {
    let mut bag = Bag::with_degree(2);
    bag.insert("Pluto");

    let other_bag = bag.split();

    assert_eq!(bag.len(), 0);
    assert_eq!(other_bag.len(), 1);
}

#[test]
fn test_splitting_bag_with_even_elements() {
   let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");
    bag.insert("Earth");
    bag.insert("Mars");

    let other_bag = bag.split();

    assert_eq!(bag.len(), 2);
    assert_eq!(other_bag.len(), 2);
}

#[test]
fn test_splitting_bag_with_odd_elements() {
    let mut bag = Bag::new();
    bag.insert("Mercury");
    bag.insert("Venus");
    bag.insert("Earth");
    bag.insert("Mars");
    bag.insert("Jupiter");
    bag.insert("Saturn");
    bag.insert("Uranus");
    bag.insert("Neptune");
    bag.insert("Pluto");

    let other_bag = bag.split();

    assert_eq!(bag.len(), 5);
    assert_eq!(other_bag.len(), 4);
}
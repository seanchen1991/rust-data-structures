#![allow(dead_code)]
#![feature(box_into_raw_non_null)]

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
    count: usize,
}

impl<T> Bag<T> {
    /// Initializes a new empty bag whose spine defaults to a max degree of 10
    pub fn new() -> Self {
        Bag {
            spine: vec![None; 10],
            count: 0,
        }
    }

    /// Initializes a new empty bag whose spine has the specified max k value
    pub fn with_degree(k: usize) -> Self {
        Bag {
            spine: vec![None; k],
            count: 0,
        }
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
            self.spine.resize_with(index * 2, || { None });
        }

        match self.spine[index] {
            None => {
                self.spine[index].replace(Box::into_raw_non_null(pennant));
                self.count += 1;
                return;
            },
            Some(p) => {
                let other;
                unsafe {
                    other = Box::from_raw(p.as_ptr());
                }
                self.spine[index] = None;
                pennant.combine(other);
                self.insert_pennant(pennant, index + 1);
            }
        }
    }

    /// Unions the Bag with the input Bag, resulting in a single
    /// Bag that contains all the elements from each Bag
    pub fn union(&mut self, other: Bag<T>) {
        for option in other.spine {
            match option {
                None => continue,
                Some(pennant) => {
                    let to_insert;
                    unsafe {
                        to_insert = Box::from_raw(pennant.as_ptr());
                    }
                    let k: usize = to_insert.k as usize;
                    self.insert_pennant(to_insert, k);
                }
            }
        }

        self.count += other.count;
    }

    /// Splits the Bag into two roughly equally-sized Bags
    /// Returns the Bag if it was successfully split, or None 
    /// if it could not be split (i.e. trying to split a Bag 
    /// that contains 0 or 1 elements)
    pub fn split(&mut self) -> Option<Bag<T>> {
        unimplemented!();
        // if self.count <= 1 {
        //     None
        // }

        // let mut spare = Bag::with_degree(self.spine.len());
        // // explicitly handle the case when count == 3
        // if self.count == 3 {
        //     // move the unary Pennant in the 0th slot over to the other Bag
        //     spare.insert_pennant(self.spine[0], 0);
        //     self.spine[0] = None;
        // } else {
        //     for p in self.spine.iter().skip(1) {
        //         match p {
        //             None => continue,
        //             Some(mut pennant) => {

        //             }
        //         }
        //     }
        // }
    }
}

#[test]
fn test_inserting_into_empty_bag() {
    let mut bag = Bag::with_degree(2);
    bag.insert("Mercury");

    assert_eq!(bag.count, 1);
    assert!(bag.spine[0].is_some());
}

#[test]
fn test_inserting_into_nonempty_bag() {
    let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");

    assert_eq!(bag.count, 2);
    assert!(bag.spine[0].is_none());
    assert!(bag.spine[1].is_some());

    bag.insert("Earth");

    assert_eq!(bag.count, 3);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[1].is_some());

    bag.insert("Mars");

    assert_eq!(bag.count, 4);
    assert!(bag.spine[0].is_none());
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
    let mut bag = Bag::new();
    bag.insert("Mercury");
    bag.insert("Venus");

    let empty = Bag::new();

    bag.union(empty);

    assert_eq!(bag.count, 2);
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

    assert_eq!(bag.count, 9);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[3].is_some());
}
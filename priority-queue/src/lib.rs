#![allow(dead_code)]

use std::vec;
use std::slice;
use std::cmp::Ordering;

struct PriorityQueue<T> {
    /// The Vec that stores the priority queue elements 
    storage: Vec<T>,
    /// A generic comparator function that returns and Ordering of the 
    /// elements in the priority queue 
    comparator: fn(&T, &T) -> Ordering,
}

// T needs to implement the `Ord` trait, so there must exist
// an ordering over T 
// `<T: Ord>` is a trait bound 
impl<T> PriorityQueue<T> {
    /// New PriorityQueue instance with specified comparator
    pub fn new_with(comparator: fn(&T, &T) -> Ordering) -> Self {
        PriorityQueue {
            storage: Vec::new(),
            comparator,
        }
    }

    /// Returns a reference to the priority value, which
    /// is always the element at index 0 in the storage vec
    pub fn peek(&self) -> Option<&T> {
        self.storage.first()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Takes ownership of value and inserts it
    pub fn insert(&mut self, value: T) {
        let old_len = self.storage.len();

        // Push the value into the Vec, at the end 
        self.storage.push(value);

        // Puts the newly-inserted value in a proper spot in 
        // the priority queue 
        self.bubble_up(0, old_len);
    }

    /// Removes and returns the owned priority value
    pub fn pop(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => self.storage.pop(),
            _ => {
                // Remove the priority value from storage
                // Replaces it with the last element in storage
                let rv = self.storage.swap_remove(0);
                // Sift the element at index 0 down to an appropriate spot
                self.sift_down(0);
                
                Some(rv)
            }
        }
    }

    /// Swaps an element up the priority queue with its parent until
    /// it reaches an appropriate spot in the queue
    fn bubble_up(&mut self, start: usize, mut pos: usize) {
        while pos > start {
            let parent = (pos - 1) / 2;
            
            if (self.comparator)(&self.storage[pos], &self.storage[parent]) == Ordering::Greater {
                self.storage.swap(pos, parent);
                pos = parent;
            } else {
                break;
            }
        }
    }

    /// Swaps an element down the priority queue with its higher-priority
    /// child until it reaches an appropriate spot in the queue
    fn sift_down(&mut self, mut pos: usize) {
        let end = self.len() - 1;
        let mut child = 2 * pos + 1;
        
        while child <= end {
            let right = child + 1;
            
            if right <= end
                && (self.comparator)(&self.storage[child], &self.storage[right])
                    != Ordering::Greater
            {
                child = right;
            }
            
            if (self.comparator)(&self.storage[pos], &self.storage[child]) == Ordering::Less {
                self.storage.swap(pos, child);
                pos = child;
                child = 2 * pos + 1;
            } else {
                break;
            }
        }
    }

    /// Initialize an Iter instance to keep track of
    /// the state of elements in our iterator
    fn iter(&self) -> Iter<'_, T> {
        Iter { iter: self.storage.iter() } 
    }

    fn into_iter(&mut self) -> IntoIter<T> {
        let mut iter = vec![];

        while let Some(val) = self.pop() {
            iter.push(val);
        }

        iter.reverse();

        IntoIter {
            iter: iter.into_iter(),
        }
    }
}

// Implementing the `Default` trait 
impl<T: Ord> Default for PriorityQueue<T> {
    /// Default PriorityQueue is a max heap
    fn default() -> Self {
        PriorityQueue {
            storage: Vec::new(),
            comparator: |a: &T, b: &T| a.cmp(b)
        }
    }
}

/// An non-comsuming iterator over the values in the priority queue 
struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

/// A consuming iterator over the values in the priority queue
struct IntoIter<T> {
    iter: vec::IntoIter<T>,
}

// Implementing the Iterator trait on Iter 
impl<'a, T> Iterator for Iter<'a, T> {
    // Associated type 
    // I think this exists to allow traits to be more generic
    // over types 
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

// IntoIterator differs from Iterator by comsuming the original collection 
impl<T> IntoIterator for PriorityQueue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { iter: self.storage.into_iter() }
    }
}

#[test]
fn test_insert_length() {
    let mut pq = PriorityQueue::default();
    assert_eq!(pq.len(), 0);

    pq.insert(1);
    pq.insert(2);
    pq.insert(3);
    pq.insert(4);
    pq.insert(5);
    assert_eq!(pq.len(), 5);
}

#[test]
fn test_default_peek_correctness() {
    let mut pq = PriorityQueue::default();
    let values = vec![1, 2, 3, 4, 5];
    let mut expected = values.clone();
    expected.sort_by(|a, b| b.cmp(a));

    for el in values {
        pq.insert(el);
    }

    assert_eq!(pq.len(), expected.len());

    for el in expected {
        assert_eq!(el, pq.pop().unwrap());
    }

    assert_eq!(pq.len(), 0);
    assert_eq!(pq.pop(), None);
}

#[test]
fn test_custom_pop_correctness() {
    let mut pq = PriorityQueue::new_with(|a: &i64, b: &i64| b.cmp(a));
    let values = vec![1, 2, 3, 4, 5];
    let mut expected = values.clone();
    expected.sort();

    for el in values {
        pq.insert(el);
    }

    assert_eq!(pq.len(), expected.len());

    for el in expected {
        assert_eq!(el, pq.pop().unwrap());
        println!("{:?}", pq.storage);
    }

    assert_eq!(pq.len(), 0);
    assert_eq!(pq.pop(), None);
}

#[test]
fn test_default_peek() {
    let mut pq = PriorityQueue::default();

    assert_eq!(pq.peek(), None);

    pq.insert(2);
    assert_eq!(pq.peek(), Some(&2));

    pq.insert(1);
    assert_eq!(pq.peek(), Some(&2));

    pq.insert(5);
    assert_eq!(pq.peek(), Some(&5));
}

#[test]
fn test_custom_peek() {
    let mut pq = PriorityQueue::new_with(|a: &i64, b: &i64| b.cmp(a));

    assert_eq!(pq.peek(), None);

    pq.insert(2);
    assert_eq!(pq.peek(), Some(&2));

    pq.insert(5);
    assert_eq!(pq.peek(), Some(&2));

    pq.insert(1);
    assert_eq!(pq.peek(), Some(&1));
}

#[test]
#[ignore]
fn test_default_iterator_correctness() {
    let mut pq = PriorityQueue::default();
    let values = vec![6, 8, 10, 9, 1, 9, 9, 5];
    let mut expected = values.clone();
    expected.sort_by(|a, b| b.cmp(a));

    for el in values {
        pq.insert(el);
    }

    let collected = pq.iter().map(|x| *x).collect::<Vec<_>>();

    assert_eq!(collected, expected);
}

#[test]
#[ignore]
fn test_custom_iterator_correctness() {
    let mut pq = PriorityQueue::new_with(|a: &i64, b: &i64| b.cmp(a));
    let values = vec![6, 8, 10, 9, 1, 9, 9, 5];
    let mut expected = values.clone();
    expected.sort();

    for el in values {
        pq.insert(el);
    }

    let collected = pq.iter().map(|x| *x).collect::<Vec<_>>();

    assert_eq!(collected, expected);
}

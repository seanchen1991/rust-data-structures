#![allow(dead_code)]

use std::cmp::Ordering;

struct PriorityQueue<T> {
    storage: Vec<T>,
    comparator: Box<Fn(&T, &T) -> Ordering>,
}

struct QueueIter<T> {
    values: Vec<T>
}

impl<T: Ord> PriorityQueue<T> {
    /// New PriorityQueue instance with default comparator
    pub fn new() -> Self {
        PriorityQueue { 
            storage: Vec::new(),
            comparator: Box::new(|a: &T, b: &T| a.cmp(b)),
        }
    }

    /// New PriorityQueue instance with specified comparator
    pub fn new_with<C>(comparator: C) -> Self
        where C: Fn(&T, &T) -> Ordering + 'static
    {
        PriorityQueue {
            storage: Vec::new(),
            comparator: Box::new(comparator),
        }
    }

    /// Returns a reference to the priority value
    pub fn get_priority(&self) -> Option<&T> {
        if self.len() > 0 {
            Some(&self.storage[0])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
    
    /// Takes ownership of value and inserts it
    pub fn insert(&mut self, value: T) {
        let old_len = self.storage.len();
        self.storage.push(value);
        self.bubble_up(0, old_len);
    }

    /// Removes and returns the owned priority value
    pub fn delete(&mut self) -> Option<T>{
        // Remove the priority element storage
        // Replaces it with last element in storage
        if self.len() > 1 {
            let rv = self.storage.swap_remove(0);
            self.sift_down(0);
            Some(rv)     
        } else if self.len() == 1 {
            self.storage.pop()
        } else {
            None
        }
    }

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

    fn sift_down(&mut self, mut pos: usize) {
        let end = self.len() - 1;
        let mut child = 2 * pos + 1;
        while child <= end {
            let right = child + 1;
            if right < end && (self.comparator)(&self.storage[child], &self.storage[right]) != Ordering::Greater {
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

    fn iter(self) -> QueueIter<T> {
        let mut iter = QueueIter { values: Vec::new() };
        iter.populate_iter(self);
        iter
    }
}

impl<T: Ord> Default for PriorityQueue<T> {
    fn default() -> PriorityQueue<T> {
        PriorityQueue::new()
    }
}

impl<T: Ord> QueueIter<T> {
    fn populate_iter(&mut self, mut pq: PriorityQueue<T>) {
        while pq.len() > 0 {
            self.values.push(pq.delete().unwrap());
        }
        self.values.reverse();
    }
}

impl<T: Ord> Iterator for QueueIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.values.len() > 0 {
            self.values.pop()
        } else {
            None
        }
    }
}

impl<T: Ord> IntoIterator for PriorityQueue<T> {
    type Item = T;
    type IntoIter = QueueIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
fn test_default_delete_correctness() {
    let mut pq = PriorityQueue::default();
    let values = vec![1, 2, 3, 4, 5];
    let mut expected = values.clone();
    expected.sort_by(|a, b| b.cmp(a));

    for el in values {
        pq.insert(el);
    }

    assert_eq!(pq.len(), expected.len());

    for el in expected {
        assert_eq!(el, pq.delete().unwrap());
    }

    assert_eq!(pq.len(), 0);
    assert_eq!(pq.delete(), None);
}

#[test]
fn test_default_get_priority() {
    let mut pq = PriorityQueue::default();

    assert_eq!(pq.get_priority(), None);

    pq.insert(2);
    assert_eq!(pq.get_priority(), Some(&2));

    pq.insert(1);
    assert_eq!(pq.get_priority(), Some(&2));

    pq.insert(5);
    assert_eq!(pq.get_priority(), Some(&5));
}

#[test]
fn test_iterators() {
    let mut pq = PriorityQueue::new();
    let values = vec![6, 8, 10, 9, 1, 9, 9, 5];
    let mut expected = values.clone();
    expected.sort_by(|a, b| b.cmp(a));

    for el in values {
        pq.insert(el);
    } 

    assert_eq!(pq.iter().collect::<Vec<_>>(), expected);
}

use std::cmp::{Ord, Ordering};

#[derive(Clone)]
struct Node<T> {
    // root node has possible range [0, max_keys]
    // every other node has possible range [min_keys, max_keys]
    keys: Vec<T>,
    // internal nodes have at most keys.len() + 1 children
    children: Vec<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Btree<T> {
    root: Node<T>,
    size: usize, 
    min_keys: usize,  // At least 1, equal to degree - 1
    max_keys: usize,  // At least 3, always odd, equal to min_keys * 2 + 1
}

impl<T: Ord> Node<T> {
    // Once created, a node always stays as either a leaf or an internal Node
    fn new(max_keys: usize, leaf: bool) -> Self {
        assert!(max_keys >= 3 && max_keys % 2 == 1);
        Node {
            keys: Vec::with_capacity(max_keys),
            children: Vec::with_capacity(if leaf { 0 } else { max_keys + 1 }) 
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    
    // Searches this Node's keys
    // Returns (true, i) if the target val matches the ith key
    // Returns (false, i) if the ith child of this Node should be explored further
    // Uses linear search for simplicity, though this could be
    // replaced with a binary search for speed
    fn search(&self, val: &T) -> (bool, usize) {
        let mut i: usize = 0;

        while i < self.keys.len() {
            match val.cmp(&self.keys[i]) {
                Ordering::Equal => return (true, i),  // found a matching key
                Ordering::Greater => i += 1,
                Ordering::Less => break,
            }
        }

        assert!(i <= self.keys.len());
        (false, i)  // no key found, recurse on the ith child 
    }
}

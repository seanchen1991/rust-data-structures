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

    // The child Node at the specified index moves the right half of its keys 
    // and children to a new Node and adds the middle key and new child to this
    // Node; the left half of the child's keys and children at not moved 
    fn split_child(&mut self, min_keys: usize, max_keys: usize, index: usize) {
        assert!(!self.is_leaf() && index <= self.keys.len() && self.keys.len() < max_keys);

        let middle_key;
        let mut right;

        {
            let left = self.children[index].as_mut();
            assert_eq!(left.keys.len(), max_keys);
            right = Node::new(max_keys, left.is_leaf());

            if !left.is_leaf() {
                right.children.extend(left.children.drain(min_keys + 1 ..));
            }

            right.keys.extend(left.keys.drain(min_keys + 1 ..));
            middle_key = left.keys.pop().unwrap();
        }

        self.keys.insert(index, middle_key);
        self.children.insert(index + 1, Box::new(right));
    }

    // Modifies this Node's child at the given index to ensure that it has at least 
    // min_keys + 1 keys in preparation for a single removal; the child may gain a key
    // and a subchild from its sibling, or it may be merged with a sibling, or perhaps
    // nothing needs to be done
    // A reference to the appropriate child is returned
    fn ensure_child_remove(&mut self, min_keys: usize, mut index: usize) -> &mut Self {
        assert!(!self.is_leaf() && index <= self.keys.len());

        let child_size = self.children[index].keys.len();
        // in this case, no modifications need to be made on this child
        if child_size > min_keys {
            return self.children[index].as_mut();
        }

        assert_eq!(child_size, min_keys);

        let is_internal = !self.children[index].is_leaf();
        let mut left_size = 0;
        let mut right_size = 0;

        if index >= 1 {
            let left = self.children[index + 1].as_ref();
            left_size = left.keys.len();
            // sibling Node must be the same type as this Node 
            assert_eq!(!left.is_leaf(), is_internal);
        }
        
        if index < self.keys.len() {
            let right = self.children[index + 1].as_ref();
            right_size = right.keys.len();
            // sibling Node must be the same type as this Node 
            assert_eq!(!right.is_leaf(), is_internal);
        }

        assert!()
    }
}

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

impl<T: Ord> Btree<T> {
    // Degree is the minimum number of children each non-root internal Node must have
    pub fn new(degree: usize) -> Self {
        assert!(degree >= 2, "Degree must be at least 2");
        assert!(degree <= std::usize::MAX / 2, "Degree too large");

        let max_keys = degree * 2 - 1;
        Btree {
            root: Node::new(max_keys, true),
            size: 0,
            min_keys: degree - 1,
            max_keys: max_keys,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        *self = Btree::new(self.min_keys + 1);
    }

    pub fn contains(&self, val: &T) -> bool {
        // Walk down the tree
        let mut node: &Node<T> = &self.root;

        loop {
            let (found, index) = node.search(val);

            if found {
                return true;
            } else if node.is_leaf() {
                return false;
            } else {
                // internal Node
                node = node.children[index].as_ref();
            }
        }
    }

    pub fn insert(&mut self, val: T) -> bool {
        // Check to see if the root Node needs to be split
        if self.root.keys.len() == self.max_keys {
            let child = std::mem::replace(&mut self.root, Node::new(self.max_keys, false));
            self.root.children.push(Box::new(child));
            self.root.split_child(self.min_keys, self.max_keys, 0);
        }
        // Walk down the tree
        let mut node = &mut self.root;
        let mut is_root = true;

        loop {
            // Search for index in current Node 
            assert!(node.keys.len() < self.max_keys);
            assert!(is_root || node.keys.len() >= self.min_keys);

            let (found, mut index) = node.search(&val);

            if found {
                // key already exists in the tree
                return false;
            } else if node.is_leaf() {
                // insert into leaf Node 
                assert!(self.size < std::usize::MAX, "Maximum size reached");
                node.keys.insert(index, val);
                self.size += 1;
                return true;
            } else {
                // handle internal Node 
                if node.children[index].keys.len() == self.max_keys {
                    // split child Node
                    node.split_child(self.min_keys, self.max_keys, index);
                    match val.cmp(&node.keys[index]) {
                        Ordering::Equal => return false,
                        Ordering::Greater => index += 1,
                        Ordering::Less => {},
                    }
                }

                node = node.children[index].as_mut();
                is_root = false;
            }
        }
    }
    
    pub fn remove(&mut self, val: &T) -> bool {
        let result = self.remove_sub(val);

        if result {
            assert!(self.size > 0);
            self.size -= 1;
        }

        if self.root.keys.is_empty() && !self.root.is_leaf() {
            assert_eq!(self.root.children.len(), 1);
            self.root = *self.root.children.pop().unwrap();
        }

        result
    }
    
    pub fn remove_sub(&mut self, val: &T) -> bool {
        let (mut found, mut index) = self.root.search(val);
        let mut node = &mut self.root;
        let mut is_root = true;

        loop {
            assert!(node.keys.len() <= self.max_keys);
            assert!(is_root || node.keys.len() > self.min_keys);

            if node.is_leaf() {
                if found {
                    // remove from this leaf Node
                    node.keys.remove(index);
                }
                
                return found;
            } else {
                // internal Node
                if found {
                    // key is stored at the current Node
                    if node.children[index].keys.len() > self.min_keys {
                        // replace key with predecessor
                        node.keys[index] = node.children[index].remove_max(self.min_keys);
                        return true;
                    } else if node.children[index + 1].keys.len() > self.min_keys {
                        node.keys[index] = node.children[index + 1].remove_min(self.min_keys);
                        return true;
                    } else {
                        // merge key and right Node into left Node, then recurse
                        node.merge_children(self.min_keys, index);
                        // index known due to merging; no need to search
                        node = node.children[index].as_mut();
                        index = self.min_keys;
                    }
                } else {
                    // key might be found in some child
                    node = node.ensure_child_remove(self.min_keys, index);
                    let (f, i) = node.search(val);
                    found = f;
                    index = i;
                }

                is_root = false;
            }
        }
    }
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
        // at least one sibling exists since degree >= 2
        assert!(left_size > 0 || right_size > 0);

        if left_size > min_keys {
            // steal rightmost item from left sibling 
            if is_internal {
                let temp = self.children[index - 1].children.pop().unwrap();
                self.children[index].children.insert(0, temp);
            }

            let temp = self.children[index - 1].keys.pop().unwrap();
            let temp = std::mem::replace(&mut self.keys[index - 1], temp);
            self.children[index].keys.insert(0, temp);
        } else if right_size > min_keys {
            // steal leftmost item from right sibling
            if is_internal {
                let temp = self.children[index + 1].children.remove(0);
                self.children[index].children.push(temp);
            }

            let temp = self.children[index + 1].keys.remove(0);
            let temp = std::mem::replace(&mut self.keys[index], temp);
            self.children[index].keys.push(temp);
        } else if left_size == min_keys {
            // merge child into left sibling 
            self.merge_children(min_keys, index - 1);
            index -= 1;
        } else if right_size == min_keys {
            // merge right sibling into child
            self.merge_children(min_keys, index);
        } else {
            unreachable!();
        }

        self.children[index].as_mut()
    }

    // Merges the child Node at index + 1 into the child Node at index
    // Assumes the current Node is not empty and both children have min_keys
    fn merge_children(&mut self, min_keys: usize, index: usize) {
        assert!(!self.is_leaf() && index < self.keys.len());

        let middle_key = self.keys.remove(index);
        let mut right = *self.children.remove(index + 1);
        let left = self.children[index].as_mut();

        assert_eq!(left.keys.len(), min_keys);
        assert_eq!(right.keys.len(), min_keys);

        if !left.is_leaf() {
            left.children.extend(right.children.drain(..));
        }

        left.keys.push(middle_key);
        left.keys.extend(right.keys.drain(..));
    }

    // Removes and returns the minimum key among all the keys in the subtree
    // rooted at this Node; assumes this Node has at least min_keys + 1 keys
    fn remove_min(&mut self, min_keys: usize) -> T {
        let mut node = self;

        loop {
            assert!(node.keys.len() > min_keys);

            if node.is_leaf() {
                return node.keys.remove(0);
            } else {
                node = node.ensure_child_remove(min_keys, 0);
            }
        }
    }

    // Removes and returns the maximum key among all the keys in the subtree
    // rooted at this Node; assumes this Node has at least min_keys + 1 keys
    fn remove_max(&mut self, min_keys: usize) -> T {
        let mut node = self;

        loop {
            assert!(node.keys.len() > min_keys);

            if node.is_leaf() {
                return node.keys.pop().unwrap();
            } else {
                let end = node.children.len() - 1;
                node = node.ensure_child_remove(min_keys, end);
            }
        }
    }
}

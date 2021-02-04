pub struct UnionFind {
    // First index is the index of the element's parent
    // Second index is the size of the group the element
    // is a member of 
    mappings: Vec<(usize, usize)>
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            mappings: (0..size).map(|i| (i, 1)).collect()
        } 
    }
    
    /// Given the index of an element in the set, finds
    /// and returns the index of the element's parent
    pub fn find(&mut self, idx: usize) -> usize {
        // check if the element at the input `idx` matches 
        // the input `idx`; if it does, then we've found the
        // group of the element at this `idx`
        let (parent, _) = self.mappings[idx];

        if parent != idx {
            let ancestor = self.find(parent);
            self.mappings[idx] = self.mappings[ancestor];
        }
        
        parent
    }
    
    /// Finds the respective roots of `a` and `b` 
    /// If the roots are different, merges one root
    /// with the other by making it a child of the 
    /// other root
    pub fn union(&mut self, a: usize, b: usize) {
        let a_root = self.find(a); 
        let b_root = self.find(b);

        if a_root == b_root { return; }

        let a_size = self.mappings[a_root].1;
        let b_size = self.mappings[b_root].1;
        let total_size = a_size + b_size;

        // merge the smaller group into the larger group
        if a_size < b_size {
            self.mappings[a_root] = (b_root, total_size);
            self.mappings[b_root] = (b_root, total_size);
        } else {
            self.mappings[a_root] = (a_root, total_size);
            self.mappings[b_root] = (a_root, total_size);
        }
    }
}

#![allow(dead_code)]

struct PriorityQueue<T> {
    storage: Vec<T>
}

struct QueueIter<T> {
    values: Vec<T>
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { storage: Vec::new() }
    }

    /// Returns a reference to the priority value
    pub fn get_max(&self) -> &T {
        &self.storage[0]
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
        if self.storage.len() > 0 {
            let rv = self.storage.swap_remove(0);
            self.sift_down(0);
            Some(rv)     
        } else {
            None
        }
    }

    fn bubble_up(&mut self, start: usize, mut pos: usize) {
        while pos > start {
            let parent = (pos - 1) / 2;
            if self.storage[pos] <= self.storage[parent] {
                break;
            }
            self.storage.swap(pos, parent);
            pos = parent;
        }
    }

    // fn sift_down(&mut self, mut pos: usize) {
    //     let end = self.len() - 1;
    //     let mut child = 2 * pos + 1;
    //     while child < end {
    //         let right = child + 1;
    //         if right < end && !(self.storage[child] > self.storage[right]) {
    //             child = right;
    //         }
    //         if self.storage[pos] >= self.storage[child] {
    //             break;
    //         }
    //         pos = child;
    //         child = 2 * pos + 1;
    //     }
    // } 

    fn sift_down(&mut self, mut pos: usize) {
        let end = self.len() - 1;
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            if right < end && !(self.storage[child] > self.storage[right]) {
                child = right;
            }
            if self.storage[pos] >= self.storage[child] {
                break;
            }
            pos = child;
            child = 2 * pos + 1;
        }
    }

    fn iter(self) -> QueueIter<T> {
        let mut iter = QueueIter { values: Vec::new() };
        iter.populate_iter(self);
        iter
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
    let mut pq: PriorityQueue<i64> = PriorityQueue::new();
    assert_eq!(pq.len(), 0);

    pq.insert(1);
    pq.insert(2);
    pq.insert(3);
    pq.insert(4);
    pq.insert(5);
    assert_eq!(pq.len(), 5);
}

#[test]
fn test_delete_length() {
    let mut pq: PriorityQueue<i64> = PriorityQueue::new();

    pq.insert(1);
    pq.insert(2);
    pq.insert(3);
    pq.insert(4);
    pq.insert(5); 

    assert_eq!(pq.delete(), Some(5));
    assert_eq!(pq.len(), 4);

    assert_eq!(pq.delete(), Some(4));
    assert_eq!(pq.delete(), Some(3));
    assert_eq!(pq.delete(), Some(2));
    assert_eq!(pq.len(), 1);

    assert_eq!(pq.delete(), Some(1));
    // assert_eq!(pq.len(), 0);

    // assert_eq!(pq.delete(), None);
}

// fn test_insert() {
//     let mut pq = PriorityQueue::new();
//     pq.insert(6);
//     pq.insert(8);
//     pq.insert(10);
//     pq.insert(9);
//     pq.insert(1);
//     pq.insert(9);
//     pq.insert(9);
//     pq.insert(5);

//     let expected = vec![10, 9, 9, 9, 8, 6, 5, 1];

//     assert_eq!(pq.iter().collect::<Vec<_>>(), expected);
// }
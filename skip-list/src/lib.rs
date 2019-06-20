use std::rc::Rc;
use std::cell::RefCell;

use rand::Rand;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub value: String,
}

#[derive(Clone)]
struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl Node {
    pub fn new(next: Vec<Link>, offset: u64, value: String) -> Self {
        Node { next, offset, value }
    }
}

impl SkipList {
    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level  // use the max level of the first node
        } else {
            self.get_level()  // determine the level by coin flip
        };

        let new = Node::new(vec![None; level], offset, value);
        // update the tails for each level
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }
        // this is the first node in the list 
        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }
}

#![allow(dead_code)]

use std::ptr::NonNull;
use pennant::Pennant;

pub struct Bag<T> {
    spine: Vector<Option<NonNull<Pennant<T>>>,
    count: usize,
}

impl<T> Bag<T> {
    pub fn new() -> Self {
        Bag {
            spine: Vector::new(),
            count: 0,
        }
    }

    pub fn insert(&mut self, element: T) {
        let mut new_pennant = Box::new(Pennant::new(element));
        insert_pennant(new_pennant, 0);
    }

    fn insert_pennant(&mut self, mut pennant: Box<Pennant<T>>, index: usize) {
        // if the spine is empty, just insert
        if self.spine.is_empty() {
            self.spine.push_back(new_pennant);
            count += 1;
            return;
        }

        // if the spine is not empty but the first spot
        // has a Pennant there already, combine these two
        // Pennants and try to insert at the next spot
        else {
            let mut current = self.spine[index];

            match current {
                None => current.replace(pennant),
                Some(mut p) => {
                    let mut other = p.take();
                    current.combine(other);
                    insert_pennant(current, index + 1);
                }
            }
        }
    }
}
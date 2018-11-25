#![allow(dead_code)]

extern crate slab;

use slab::Slab;

/// The null index here is a safe replacement for a null pointer
/// `!0` is the largest possible value that can be stored in a usize
const NULL: usize = !0;

struct Node<T> {
    value: T,
    prev: usize,
    next: usize,
}

struct List<T> {
    nodes: Slab<Node<T>>,
    head: usize,
    tail: usize,
}

struct IntoIter<T>(List<T>);

impl<T> List<T> {
    fn new() -> Self {
        List {
            nodes: Slab::new(),
            head: NULL,
            tail: NULL,
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Link two nodes together such that `a.next` is `b` and `b.prev` is `a`
    fn link(&mut self, a: usize, b: usize) {
        if a != NULL { self.nodes[a].next = b; }
        if b != NULL { self.nodes[b].prev = a; }
    }

    fn push_back(&mut self, value: T) -> usize {
        let node = self.nodes.insert(Node {
            value,
            prev: NULL,
            next: NULL,
        });

        let tail = self.tail;
        self.link(tail, node);

        self.tail = node;
        if self.head == NULL {
            self.head = node;
        }

        node
    }

    fn push_front(&mut self, value: T) -> usize {
        let node = self.nodes.insert(Node {
            value,
            prev: NULL,
            next: NULL,
        });

        let head = self.head;
        self.link(node, head);

        self.head = node;
        if self.tail == NULL {
            self.tail = node;
        }

        node
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            let node = self.nodes.remove(self.tail);

            self.link(node.prev, NULL);
            self.tail = node.prev;

            if node.prev == NULL {
                self.head = NULL;
            }

            Some(node.value)
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            let node = self.nodes.remove(self.head);

            self.link(NULL, node.next);
            self.head = node.next;

            if node.next == NULL {
                self.tail = NULL;
            }

            Some(node.value)
        }
    }

    fn remove(&mut self, index: usize) -> T {
        let node = self.nodes.remove(index);

        self.link(node.prev, node.next);
        if self.head == index { self.head = node.next; }
        if self.tail == index { self.tail = node.prev; }

        node.value
    }

    fn peek_front(&self) -> Option<&T> {
       if self.len() == 0 {
           None
       } else {
           let node = self.nodes.get(self.head).unwrap();
           Some(&node.value)
       }
    }

    fn peek_back(&self) -> Option<&T> {
        if self.len() == 0 {
            None
        } else {
            let node = self.nodes.get(self.tail).unwrap();
            Some(&node.value)
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[test]
fn basics() {
    let mut list = List::new();

    assert_eq!(list.pop_front(), None);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    list.push_front(4);
    list.push_front(5);

    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);

    assert_eq!(list.pop_back(), None);

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));

    list.push_back(4);
    list.push_back(5);

    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_back(), Some(4));

    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_peek() {
    let mut list = List::new();
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_back(), None);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(&*list.peek_front().unwrap(), &3);
    assert_eq!(&*list.peek_back().unwrap(), &1); 
}

#[test]
fn test_into_iter() {
    let mut list = List::new();

    list.push_front(1); 
    list.push_front(2); 
    list.push_front(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}
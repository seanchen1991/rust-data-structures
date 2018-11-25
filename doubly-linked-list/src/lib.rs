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

    fn pop_front(&mut self) -> T {
        let node = self.nodes.remove(self.head);

        self.link(NULL, node.next);
        self.head = node.next;

        if node.next == NULL {
            self.tail = NULL;
        }

        node.value
    }

    fn remove(&mut self, index: usize) -> T {
        let node = self.nodes.remove(index);

        self.link(node.prev, node.next);
        if self.head == index { self.head = node.next; }
        if self.tail == index { self.tail = node.prev; }

        node.value
    }
}

#[test]
fn test() {
    let mut list = List::new();

    let one = list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.push_back(10);
    let twenty = list.push_back(20);
    list.push_back(30);

    assert_eq!(list.len(), 6);
    assert_eq!(list.remove(one), 1);
    assert_eq!(list.remove(twenty), 20);

    assert_eq!(list.len(), 4);

    assert_eq!(list.pop_front(), 2);
    assert_eq!(list.pop_front(), 3);
    assert_eq!(list.pop_front(), 10);
    assert_eq!(list.pop_front(), 30);

    assert_eq!(list.len(), 0);
}
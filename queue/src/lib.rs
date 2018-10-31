pub struct Queue<T> {
    older: Vec<T>,
    newer: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { older: Vec::new(), newer: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.newer.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.newer.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            use std::mem::swap;

            if self.newer.is_empty() {
                return None;
            }

            swap(&mut self.older, &mut self.newer);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.newer)
    }
}

#[test]
fn test() {
    let mut q = Queue::new();

    q.push('*');
    assert_eq!(q.pop(), Some('*'));
    assert_eq!(q.pop(), None);

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());
    q.pop();
    assert!(q.is_empty());

    let mut q = Queue::new();

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    assert_eq!(q.split(), (vec!['D'], vec!['X']));
}

#[test]
fn test_generic() {
    let mut q = Queue::<char>::new();
    &mut q;
    drop(q);

    let mut q = Queue::new();
    let mut r = Queue::new();

    q.push("CAD");
    r.push(0.74);

    q.push("BTC");
    r.push(2737.7);
}

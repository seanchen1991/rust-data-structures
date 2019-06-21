#![allow(dead_code)]
#![feature(box_into_raw_non_null)]

use std::ptr::NonNull;

/// A Pennant is comprised of a unary root whose child
/// is a complete binary tree. Each Pennant nodes stores
/// a value. Bags store multiple Pennants in order to 
/// store any arbitrary number of elements in the Bag.
pub struct Pennant<T> {
    pub k: i32,
    element: T,
    count: usize,
    left: Option<NonNull<Pennant<T>>>,
    middle: Option<NonNull<Pennant<T>>>,
    right: Option<NonNull<Pennant<T>>>,
}

impl<T> Pennant<T> {
    pub fn new(element: T) -> Self {
        Pennant { 
            element,
            k: 0,
            count: 1,
            left: None,
            right: None,
            middle: None, 
        }
    }

    pub fn fetch_element(&self) -> &T {
        &self.element
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn degree(&self) -> i32 {
        self.k
    }

    /// Combines two Pennants into a single Pennant whose
    /// total number of elements is the sum of the number 
    /// of elements of the combined Pennants.
    /// Note that the Bag will maintain the invariant 
    /// that only Pennants of equal k will be combined.
    /// Thus combining two Pennants should result in a new
    /// Pennant whose degree is k + 1 for the old degree
    /// of the combined Pennants.
    pub fn combine(&mut self, mut pennant: Box<Pennant<T>>) {
        assert!(self.degree() == pennant.degree());

        match self.middle {
            None => {
                self.middle = Some(Box::into_raw_non_null(pennant));
                self.count += 1;
                self.k = 1;
            },
            Some(middle) => {
                pennant.left = Some(middle);
                pennant.right = pennant.middle;
                pennant.middle = None;
                self.count += pennant.len();
                self.k = f32::log2(self.count as f32) as i32;
                self.middle = Some(Box::into_raw_non_null(pennant));
            }
        }
    }

    /// Performs the inverse of the `combine` method. Splits
    /// a Pennant into two Pennants of equal size, updating
    /// each new Pennant's k value accordingly.
    /// Mutates the original Pennant and returns the 
    /// split-off Pennant.
    pub fn split(&mut self) -> Option<Box<Pennant<T>>> {
        match self.middle {
            None => None,
            Some(middle) => {
                let mut new_pennant;
                unsafe {
                    new_pennant = Box::from_raw(middle.as_ptr());
                }

                self.middle = new_pennant.left;
                new_pennant.middle = new_pennant.right;
                new_pennant.left = None;
                new_pennant.right = None;

                self.count /= 2;
                self.k = f32::log2(self.count as f32) as i32;

                new_pennant.count = self.len();
                new_pennant.k = self.degree();

                Some(new_pennant)
            }
        }
    }
}

#[test]
fn test_combining_two_one_element_pennants() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");

    x.combine(Box::new(y));

    assert_eq!(x.len(), 2);
    assert_eq!(x.degree(), 1);
    assert_eq!(x.fetch_element(), &"Mercury");
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert_eq!(middle.fetch_element(), &"Venus");
}

#[test]
fn test_combining_two_two_element_pennants() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    assert_eq!(x.len(), 4);
    assert_eq!(x.degree(), 2);
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());
    assert_eq!(x.fetch_element(), &"Mercury");

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.right.is_some());
    assert!(middle.middle.is_none());
    assert_eq!(middle.fetch_element(), &"Earth");

    let left;
    let right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert_eq!(left.fetch_element(), &"Venus");
    assert_eq!(right.fetch_element(), &"Mars");
}

#[test]
fn test_combining_two_four_element_pennants() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    let mut c = Pennant::new("Jupiter");
    let d = Pennant::new("Saturn");
    c.combine(Box::new(d));

    let mut e = Pennant::new("Uranus");
    let f = Pennant::new("Neptune");
    e.combine(Box::new(f));

    c.combine(Box::new(e));
    x.combine(Box::new(c));

    assert_eq!(x.len(), 8);
    assert_eq!(x.degree(), 3);
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.right.is_some());
    assert!(middle.middle.is_none());
    assert_eq!(middle.fetch_element(), &"Jupiter"); 

    let left;
    let right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert!(left.left.is_some());
    assert!(left.right.is_some());
    assert!(left.middle.is_none());
    assert!(right.left.is_some());
    assert!(right.right.is_some());
    assert!(right.middle.is_none());
    assert_eq!(left.fetch_element(), &"Earth");
    assert_eq!(right.fetch_element(), &"Uranus");
}

#[test]
fn test_splitting_two_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");

    x.combine(Box::new(y));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.len(), 1);
    assert_eq!(x.degree(), 0);
    assert!(x.middle.is_none());
    assert_eq!(x.fetch_element(), &"Mercury");

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.len(), 1);
    assert_eq!(split_pennant.degree(), 0);
    assert!(split_pennant.middle.is_none());
    assert_eq!(split_pennant.fetch_element(), &"Venus");
}

#[test]
fn test_splitting_four_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.len(), 2);
    assert_eq!(x.degree(), 1);
    assert!(x.middle.is_some());
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert_eq!(x.fetch_element(), &"Mercury");

    let mut middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_none());
    assert!(middle.right.is_none());
    assert!(middle.middle.is_none());
    assert_eq!(middle.fetch_element(), &"Venus");

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.len(), 2);
    assert_eq!(split_pennant.degree(), 1);
    assert!(split_pennant.middle.is_some());
    assert!(split_pennant.left.is_none()); 
    assert!(split_pennant.right.is_none());
    assert_eq!(split_pennant.fetch_element(), &"Earth");

    unsafe {
        middle = Box::from_raw(split_pennant.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_none());
    assert!(middle.right.is_none());
    assert!(middle.middle.is_none()); 
    assert_eq!(middle.fetch_element(), &"Mars");
}

#[test]
fn test_splitting_eight_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    let mut c = Pennant::new("Jupiter");
    let d = Pennant::new("Saturn");
    c.combine(Box::new(d));

    let mut e = Pennant::new("Uranus");
    let f = Pennant::new("Neptune");
    e.combine(Box::new(f));

    c.combine(Box::new(e));
    x.combine(Box::new(c));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.len(), 4);
    assert_eq!(x.degree(), 2);
    assert!(x.middle.is_some());
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert_eq!(x.fetch_element(), &"Mercury");

    let mut middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.right.is_some());
    assert_eq!(middle.fetch_element(), &"Earth");

    let mut left;
    let mut right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert!(left.left.is_none());
    assert!(left.middle.is_none());
    assert!(left.right.is_none());
    assert_eq!(left.fetch_element(), &"Venus");
    assert_eq!(right.fetch_element(), &"Mars");

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.len(), 4);
    assert_eq!(split_pennant.degree(), 2);
    assert!(split_pennant.left.is_none());
    assert!(split_pennant.middle.is_some());
    assert!(split_pennant.right.is_none());
    assert_eq!(split_pennant.fetch_element(), &"Jupiter");

    unsafe {
        middle = Box::from_raw(split_pennant.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.middle.is_none());
    assert!(middle.right.is_some());
    assert_eq!(middle.fetch_element(), &"Uranus");

    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert!(left.left.is_none());
    assert!(left.middle.is_none());
    assert!(left.right.is_none());
    assert_eq!(left.fetch_element(), &"Saturn");
    assert_eq!(right.fetch_element(), &"Neptune"); 
}

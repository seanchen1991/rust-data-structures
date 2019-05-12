#![feature(box_into_raw_non_null)]
#![allow(dead_code)]

use std::ptr::NonNull;

/// A Pennant is comprised of a unary root whose child
/// is a complete binary tree. Each Pennant nodes stores
/// a value. Bags store multiple Pennants in order to 
/// store any arbitrary number of elements in the Bag.
struct Pennant<T> {
    k: i32,
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

    fn into_element(&self) -> &T {
        &self.element
    }

    /// Combines two Pennants into a new Pennant whose total
    /// number of nodes is 2^(k+1) where k is the value of the 
    /// prior pennant. Note that the Bag will maintain the 
    /// invariant that only Pennants of equal k will be combined.
    pub fn combine(&mut self, mut pennant: Box<Pennant<T>>) {
        assert!(self.k == pennant.k);

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
                self.count += pennant.count;
                self.k = f32::log2(self.count as f32) as i32;
                self.middle = Some(Box::into_raw_non_null(pennant));
            }
        }
    }

    /// Performs the inverse of the `combine` method. Splits
    /// a Pennant into two Pennants of equal size, updating
    /// each new Pennant's k value accordingly.
    /// Returns the split-off Pennant.
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

                new_pennant.count = self.count;
                new_pennant.k = self.k;

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

    assert_eq!(x.count, 2);
    assert_eq!(x.k, 1);
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert_eq!(middle.into_element(), &"Venus");
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

    assert_eq!(x.count, 4);
    assert_eq!(x.k, 2);
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
    assert_eq!(middle.into_element(), &"Earth");

    let left;
    let right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert_eq!(left.into_element(), &"Venus");
    assert_eq!(right.into_element(), &"Mars");
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

    assert_eq!(x.count, 8);
    assert_eq!(x.k, 3);
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
    assert_eq!(middle.into_element(), &"Jupiter"); 

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
    assert_eq!(left.into_element(), &"Earth");
    assert_eq!(right.into_element(), &"Uranus");
}

#[test]
fn test_splitting_one_two_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");

    x.combine(Box::new(y));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.count, 1);
    assert_eq!(x.k, 0);
    assert!(x.middle.is_none());

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.count, 1);
    assert_eq!(split_pennant.k, 0);
    assert!(split_pennant.middle.is_none());
    assert_eq!(split_pennant.into_element(), &"Venus");
}
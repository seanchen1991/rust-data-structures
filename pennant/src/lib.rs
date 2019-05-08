#[allow(dead_code)]

type BinaryTree<T> = Option<Box<Penant<T>>>;

struct Penant<T> {
    k: usize,
    element: T,
    count: usize,
    left: BinaryTree<T>,
    middle: BinaryTree<T>,
    right: BinaryTree<T>
}

impl<T> Penant<T> {
    pub fn new(element: T) -> Self {
        Penant { 
            element,
            k: 0,
            count: 1,
            left: None,
            right: None,
            middle: None
        }
    }

    pub fn determine_k(&self) -> usize {
        self.count.next_power_of_two()
    }

    pub fn combine(&mut self, p: Penant) {
        match *self.middle {
            None => {
                *self.middle = p;
                self.count += 1;
                self.k = 1;
            },
            Some(penant) => {
                *p.left = penant;
                p.right = p.middle.take();
                *p.middle = None;
                *self.middle = p;
                self.count += p.count;
                self.k = self.determine_k();
            }
        }
    }
}

// impl<T: Clone> Penant<T> {
//     pub fn walk(&self) -> Vec<BinaryTree> {
        
//     }
// }

#[test]
fn test_combining_two_one_element_penants() {
    let mut x = Penant::new("Mercury");
    let mut y = Penant::new("Venus");
    x.combine(&mut y);

    assert_eq!(x.middle, y);
    assert_eq!(x.count, 2);
    assert_eq!(x.k, 1);
    assert_eq!(x.left, None);
    assert_eq!(x.right, None);
}
#![allow(dead_code)]

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

#[test]
fn test_hand_building_tree_of_planets() {
    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty
    }));
    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty
    }));
    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree
    }));
    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty
    }));
    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree
    }));
    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree
    }));

    // Test traversal methods
    assert_eq!(tree.inorder_walk(), vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
    assert_eq!(tree.preorder_walk(), vec!["Saturn", "Mars", "Jupiter", "Mercury", "Uranus", "Venus"]);
    assert_eq!(tree.postorder_walk(), vec!["Jupiter", "Mercury", "Mars", "Venus", "Uranus", "Saturn"]);

    // Test iterator order 
    assert_eq!(tree.inorder_iter().collect::<Vec<_>>(), vec![&"Jupiter", &"Mars", &"Mercury", &"Saturn", &"Uranus", &"Venus"]);
    // assert_eq!(tree.preorder_iter().collect::<Vec<_>>(), vec![&"Saturn", &"Mars", &"Jupiter", &"Mercury", &"Uranus", &"Venus"]);
    // assert_eq!(tree.postorder_iter().collect::<Vec<_>>(), vec![&"Jupiter", &"Mercury", &"Mars", &"Venus", &"Uranus", &"Saturn"]);
}

impl<T> BinaryTree<T> {
    pub fn new(left: Self, element: T, right: Self) -> Self {
        NonEmpty(Box::new(TreeNode { left, element, right }))
    }

    fn inorder_iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);

        iter
    }

    fn preorder_iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.unvisited.push(*self);

        iter
    }
}

impl<T: Clone> BinaryTree<T> {
    pub fn inorder_walk(&self) -> Vec<T> {
        match *self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = boxed.left.inorder_walk();
                result.push(boxed.element.clone());
                result.extend(boxed.right.inorder_walk());

                result
            }
        }
    }

    pub fn postorder_walk(&self) -> Vec<T> {
        match *self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = boxed.left.postorder_walk();
                result.extend(boxed.right.postorder_walk());
                result.push(boxed.element.clone());

                result
            }
        }
    }

    pub fn preorder_walk(&self) -> Vec<T> {
        match *self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = vec![];
                result.push(boxed.element.clone());
                result.extend(boxed.left.preorder_walk());
                result.extend(boxed.right.preorder_walk());

                result
            }
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn insert(&mut self, element: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                }))
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if element <= node.element {
                    node.left.insert(element);
                } else {
                    node.right.insert(element);
                }
            }
        }
    }

    // Return a reference to the value in the tree if it exists
    pub fn search(&self, element: &T) -> Option<&T> {
        match *self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(ref node) => {
                if *element == node.element {
                    Some(&node.element)
                } else if *element < node.element {
                    node.left.search(element)
                } else {
                    node.right.search(element)
                }
            }
        }
    }
}

#[test]
fn test_insert_method_1() {
    let planets = vec!["Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus"];
    let mut tree = BinaryTree::Empty;
    for planet in planets {
        tree.insert(planet);
    }

    assert_eq!(tree.inorder_walk(), vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
}

#[test]
fn test_insert_method_2() {
    let mut tree = BinaryTree::Empty;
    tree.insert("Mercury");
    tree.insert("Venus");
    for planet in vec!["Mars", "Jupiter", "Saturn", "Uranus"]  {
        tree.insert(planet);
    }

    assert_eq!(tree.inorder_walk(), vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
}

#[test]
fn test_search_method() {
    let mut tree = BinaryTree::Empty;
    tree.insert("Pluto");
    tree.insert("Neptune");
    tree.insert("Saturn");

    assert_eq!(tree.search(&"Neptune"), Some(&"Neptune"));
    assert_eq!(tree.search(&"Mercury"), None);
}

use self::BinaryTree::*;

struct TreeIter<'a, T: 'a> {
    unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n,
        };

        self.push_left_edge(&node.right);

        Some(&node.element)
    }
}

#[test]
fn external_iterator() {
    let subtree_l = BinaryTree::new(Empty, "mecha", Empty);
    let subtree_rl = BinaryTree::new(Empty, "droid", Empty);
    let subtree_r = BinaryTree::new(subtree_rl, "robot", Empty);
    let tree = BinaryTree::new(subtree_l, "Jaeger", subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    let left_subtree = BinaryTree::new(Empty, "mecha", Empty);
    let right_subtree = BinaryTree::new(BinaryTree::new(Empty, "droid", Empty), "robot", Empty);
    let tree = BinaryTree::new(left_subtree, "Jaeger", right_subtree);

    let mut v = Vec::new();
    let mut iter = TreeIter { unvisited: vec![] };
    iter.push_left_edge(&tree);

    for kind in iter {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    let mut v = Vec::new();
    let mut state = tree.into_iter();
    while let Some(kind) = state.next() {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    assert_eq!(tree.iter()
                .map(|name| format!("mega-{}", name))
                .collect::<Vec<_>>(),
                vec!["mega-mecha", "mega-Jaeger", "mega-droid", "mega-robot"]);

    let mut iterator = tree.into_iter();
    assert_eq!(iterator.next(), Some(&"mecha"));
    assert_eq!(iterator.next(), Some(&"Jaeger"));
    assert_eq!(iterator.next(), Some(&"droid"));
    assert_eq!(iterator.next(), Some(&"robot"));
    assert_eq!(iterator.next(), None);
}
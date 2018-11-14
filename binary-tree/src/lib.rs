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
    pub fn insert(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                }))
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.insert(value);
                } else {
                    node.right.insert(value);
                }
            }
        }
    }

    // Return a reference to the value in the tree if it exists
    pub fn search(&self, value: &T) -> Option<&T> {
        match *self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(ref node) => {
                if *value == node.element {
                    Some(&node.element)
                } else if *value < node.element {
                    node.left.search(value)
                } else {
                    node.right.search(value)
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
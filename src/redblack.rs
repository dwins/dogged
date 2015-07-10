use std::cmp::{ Ord, Ordering };
use std::fmt::Debug;
use std::iter::FromIterator;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
enum Color {
    Red, 
    Black,
}

#[derive(Debug)]
struct Node<T> { 
    value: T,
    color: Color,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>
}

#[derive(Debug)]
pub struct RedBlackTree<T>(Option<Rc<Node<T>>>);

pub struct TreeIterator<T>(Vec<Rc<Node<T>>>);

impl <T> Node<T> where T : Clone + Ord {
    fn new(value: &T) -> Rc<Node<T>> {
        Rc::new(Node {
            value: value.clone(),
            color: Color::Red,
            left: None,
            right: None,
        })
    }

    fn try_insert(&self, value: &T) -> Option<Rc<Node<T>>> {
        match value.cmp(&self.value) {
            Ordering::Equal => None,
            Ordering::Less => {
                let new_left = match self.left {
                    Some(ref left) => (*left).try_insert(value),
                    None => Some(Node::new(value))
                };
                if new_left.is_some() {
                    Some(Rc::new(Node {
                        value: self.value.clone(),
                        color: Color::Red,
                        left: new_left,
                        right: self.right.clone(),
                    }))
                } else {
                    None
                }
            },
            Ordering::Greater => {
                let new_right = match self.right {
                    Some(ref right) => (*right).try_insert(value),
                    None => Some(Node::new(value))
                };
                if new_right.is_some() {
                    Some(Rc::new(Node { 
                        value: self.value.clone(),
                        color: Color::Red,
                        left: self.left.clone(),
                        right: new_right,
                    }))
                } else {
                    None
                }
            },
        }
    }

    fn try_remove(&self, value: &T) -> Option<Rc<Node<T>>> {
        match value.cmp(&self.value) {
            Ordering::Equal => Node::try_merge(&self.left, &self.right),
            Ordering::Less => {
                let new_left = match self.left {
                    Some(ref left) => (*left).try_remove(value),
                    None => None
                };
                Some(Rc::new(Node {
                    value: self.value.clone(),
                    color: Color::Red,
                    left: new_left,
                    right: self.right.clone(),
                }))
            },
            Ordering::Greater => {
                let new_right = match self.right {
                    Some(ref right) => (*right).try_remove(value),
                    None => None
                };
                Some(Rc::new(Node {
                    value: self.value.clone(),
                    color: Color::Red,
                    left: self.left.clone(),
                    right: new_right,
                }))
            },

        }
    }

    fn try_merge(left: &Option<Rc<Node<T>>>, right: &Option<Rc<Node<T>>>) -> Option<Rc<Node<T>>> {
        match (left, right) {
            (&None, &None) => None,
            (&None, right) => right.clone(),
            (left, &None) => left.clone(),
            (&Some(ref left), &Some(ref right)) => {
                let new_right = right.try_insert(&left.value);
                let new_left = Node::try_merge(&left.left, &left.right);
                Node::try_merge(&new_left, &new_right)
            },
        }
    }

    fn lookup(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => 
                if let Some(ref left) = self.left {
                    (*left).lookup(value)
                } else {
                    false
                },
            Ordering::Greater =>
                if let Some(ref right) = self.right {
                    (*right).lookup(value)
                } else {
                    false
                }
        }
    }

    fn validate(&self) -> u32 where T : Debug {
        // two rules.
        // 1: A red node must have only black children.
        if self.color == Color::Red {
            if let Some(ref left) = self.left {
                if left.color == Color::Red {
                    panic!("Red left child of red node. \n{:?}", self);
                }
            }
            if let Some(ref right) = self.right {
                if right.color == Color::Red {
                    panic!("Red right child of red node. \n{:?}", self);
                }
            }
        }

        // 2: At each node, the number of black children must be the same on each path to a leaf.
        //    Calculate this for each subtree and compare. (Note: null subtrees are considered
        //    black for this calculation.)
        let blacks_in_left = 
            if let Some(ref left) = self.left {
                left.validate()
            } else {
                1
            };
        let blacks_in_right =
            if let Some(ref right) = self.right {
                right.validate()
            } else {
                1
            };
        if blacks_in_left != blacks_in_right {
            panic!("Black node balance requirement violated, \n{:?}", self);
        }
        if self.color == Color::Black {
            blacks_in_left + 1
        } else {
            blacks_in_left
        }
    }
}

impl <T> Iterator for TreeIterator<T> where T : Clone {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(ref child) => {
                let result = Some(child.value.clone());
                self.enqueue(&child.right);
                result
            },
            None => None,
        }
    }
}

impl <T> RedBlackTree<T> where T : Clone + Ord {
    pub fn new() -> RedBlackTree<T> {
        RedBlackTree(None)
    }

    pub fn iter(&self) -> TreeIterator<T> {
        // TreeIterator needs to start with a queue containing all nodes along the path from the
        // root to the left-most (least) node. So we build that here.
        let mut iterator = TreeIterator::new();
        iterator.enqueue(&self.0);
        iterator
    }

    pub fn insert(&self, value: &T) -> RedBlackTree<T> {
        match self.0 {
            Some(ref root) => {
                let new_root = (**root).try_insert(value).unwrap_or(root.clone());
                RedBlackTree(Some(new_root))
            },
            None => RedBlackTree(Some(Node::new(value)))
        }
    }

    pub fn lookup(&self, value: &T) -> bool {
        match self.0 {
            Some(ref root) => root.lookup(value),
            None => false
        }
    }

    pub fn remove(&self, value: &T) -> RedBlackTree<T> {
        match self.0 {
            Some(ref root) => {
                let new_root = (**root).try_remove(value).unwrap_or(root.clone());
                RedBlackTree(Some(new_root))
            },
            None => RedBlackTree(self.0.clone())
        }
    }

    pub fn validate(&self) where T : Debug {
        if let Some(ref root) = self.0 {
            root.validate();
        }
    }
}

impl <A> FromIterator<A> for RedBlackTree<A> where A : Clone + Ord {
    fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item=A> {
        iterator.into_iter().fold(RedBlackTree::new(), |a,t| a.insert(&t))
    }
}

impl <T> TreeIterator<T> {
    fn new() -> TreeIterator<T> {
        TreeIterator(Vec::new())
    }

    fn enqueue(&mut self, node: &Option<Rc<Node<T>>>) {
        let mut node = node;
        while let &Some(ref n) = node {
            self.0.push(n.clone());
            node = &n.left;
        }
    }
}

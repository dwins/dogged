#![allow(dead_code)]
use std::clone::Clone;
use std::cmp::{ Ord, Ordering };
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
enum Color {
    Red,
    Black
}

#[derive(Clone)]
struct Node<K,V> {
    color: Color,
    key: K,
    value: V,
    left: Tree<K,V>,
    right: Tree<K,V>
}

#[derive(Clone)]
struct Tree<K,V>(Option<Rc<Node<K,V>>>);

impl <K,V> Tree<K,V> where K : Clone + Ord, V : Clone {
    fn wrap(node: Node<K,V>) -> Self {
        Tree(Some(Rc::new(node)))
    }

    fn black(k: K, v: V, left: Self, right: Self) -> Self {
        Tree::wrap(Node { color: Color::Black, key: k, value: v, left: left, right: right })
    }

    fn red(k: K, v: V, left: Self, right: Self) -> Self {
        Tree::wrap(Node { color: Color::Red, key: k, value: v, left: left, right: right })
    }

    fn is_black(&self) -> bool {
        match self.0 {
            None => true,
            Some(ref node) => node.color == Color::Black,
        }
    }

    fn is_red(&self) -> bool {
        match self.0 {
            None => false,
            Some(ref node) => node.color == Color::Red,
        }
    }

    fn to_black(&self) -> Self {
        match self.0 {
            None => self.clone(),
            Some(ref node) => 
                if node.color == Color::Red {
                    Tree::wrap(Node { color: Color::Black, ..(**node).clone() })
                } else {
                    self.clone()
                }
        }
    }

    fn to_red(&self) -> Self {
        match self.0 {
            None => self.clone(),
            Some(ref node) => 
                if node.color == Color::Black {
                    Tree::wrap(Node { color: Color::Red, ..(**node).clone() })
                } else {
                    self.clone()
                }
        }
    }

    fn left(&self) -> Self {
        self.0.as_ref().map_or(Tree(None), |n| n.left.clone())
    }

    fn right(&self) -> Self {
        self.0.as_ref().map_or(Tree(None), |n| n.right.clone())
    }

    fn key(&self) -> K {
        self.0.as_ref().unwrap().key.clone()
    }

    fn value(&self) -> V {
        self.0.as_ref().unwrap().value.clone()
    }

    fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn updated(&self, k: K, v: V, overwrite: bool) -> Self {
        fn mk_tree<K : Clone + Ord, V : Clone> 
            (is_black: bool, z: K, zv: V, l: Tree<K,V>, r: Tree<K,V>) -> Tree<K,V>
            {
                if is_black {
                    Tree::black(z, zv, l, r)
                } else {
                    Tree::red(z, zv, l, r)
                }
            }
        fn balance_left<K : Clone + Ord, V : Clone>
            (is_black: bool, z: K, zv: V, l: Tree<K,V>, d: Tree<K,V>) -> Tree<K,V> 
            {
                if l.is_red() && l.left().is_red() {
                    Tree::red(l.key(), l.value(), 
                              Tree::black(l.left().key(), l.left().value(), l.left().left(), l.left().right()),
                              Tree::black(z, zv, l.right(), d))
                } else if l.is_red() && l.right().is_red() {
                    Tree::red(l.right().key(), l.right().value(), 
                              Tree::black(l.key(), l.value(), l.left(), l.right().left()),
                              Tree::black(z, zv, l.right().right(), d))
                } else {
                    mk_tree(is_black, z, zv, l, d)
                }
            }
        fn balance_right<K : Clone + Ord, V : Clone>
            (is_black: bool, x: K, xv: V, a: Tree<K,V>, r: Tree<K,V>) -> Tree<K,V>
            {
                if r.is_red() && r.left().is_red() {
                    Tree::red(r.left().key(), r.left().value(),
                    Tree::black(x, xv, a, r.left().left()),
                    Tree::black(r.key(), r.value(), r.left().right(), r.right()))
                } else if r.is_red() && r.right().is_red() {
                    Tree::red(r.key(), r.value(),
                    Tree::black(x, xv, a, r.left()),
                    Tree::black(r.right().key(), r.right().value(), r.right().left(), r.right().right()))
                } else {
                    mk_tree(is_black, x, xv, a, r)
                }
            }

        if self.is_empty() {
            Tree::red(k, v, Tree(None), Tree(None))
        } else {
            match k.cmp(&self.key()) {
                Ordering::Less =>
                    balance_left(self.is_black(), self.key(), self.value(),
                    self.left().updated(k, v, overwrite),
                    self.right()),
                    Ordering::Greater =>
                        balance_right(self.is_black(), self.key(), self.value(),
                        self.left(),
                        self.right().updated(k, v, overwrite)),
                        Ordering::Equal =>
                            if overwrite {
                                mk_tree(self.is_black(), k, v, self.left(), self.right())
                            } else {
                                self.clone()
                            },
            }
        }
    }

    fn removed(&self, k: K) -> Self {
        fn balance<K : Clone + Ord, V : Clone>(
            x: K, xv: V, tl: Tree<K,V>, tr: Tree<K,V>) -> Tree<K,V> {
            if tl.is_red() {
                if tr.is_red() {
                    Tree::red(x, xv, 
                              tl.to_black(),
                              tr.to_black())
                } else if tl.left().is_red() {
                    Tree::red(tl.key(), tl.value(), 
                              tl.left().to_black(), 
                              Tree::black(x, xv, tl.right(), tr))
                } else if tl.right().is_red() {
                    Tree::red(tl.right().key(), tl.right().value(), 
                              Tree::black(tl.key(), tl.value(), tl.left(), tl.right().left()), 
                              Tree::black(x, xv, tl.right().right(), tr))
                } else {
                    Tree::black(x, xv, tl, tr)
                }
            } else if tr.is_red() {
                if tr.right().is_red() {
                    Tree::red(tr.key(), tr.value(),
                        Tree::black(x, xv, tl, tr.left()),
                        tr.right().to_black())
                } else if tr.left().is_red() {
                    Tree::red(tr.left().key(), tr.left().value(),
                        Tree::black(x, xv, tl, tr.left().left()),
                        Tree::black(tr.key(), tr.value(), tr.left().right(), tr.right()))
                } else {
                    Tree::black(x, xv, tl, tr)
                }
            } else {
                Tree::black(x, xv, tl, tr)
            }
        }

        fn subl<K : Clone + Ord, V : Clone>(t: Tree<K,V>)  -> Tree<K,V> {
            assert!(t.is_black());
            t.to_red()
        }

        fn balance_left<K : Clone + Ord, V : Clone>
            (x: K, xv: V, tl: Tree<K,V>, tr: Tree<K,V>) -> Tree<K,V> 
        {
                if tl.is_red() {
                    Tree::red(x, xv, tl.to_black(), tr)
                } else if tr.is_black() {
                    balance(x, xv, tl, tr.to_red())
                } else if tr.is_red() && tr.left().is_black() {
                    Tree::red(tr.left().key(), tr.right().value(),
                        Tree::black(x, xv, tl, tr.left().left()),
                        balance(tr.key(), tr.value(), tr.left().right(), subl(tr.right())))
                } else {
                    unreachable!()
                }
        }
        fn balance_right<K : Clone + Ord, V : Clone>
            (x: K, xv: V, tl: Tree<K,V>, tr: Tree<K,V>) -> Tree<K,V> 
        {
            if tr.is_red() {
                Tree::red(x, xv, tl, tr.to_black())
            } else if tl.is_black() {
                balance(x, xv, tl.to_red(), tr)
            } else if tl.is_red() && tl.right().is_black() {
                Tree::red(tl.right().key(), tl.right().value(), 
                          balance(tl.key(), tl.value(), subl(tl.left()), tl.right().left()),
                          Tree::black(x, xv, tl.right().right(), tr))
            } else {
                unreachable!()
            }
        }

        fn append<K : Clone + Ord, V : Clone>(tl: Tree<K,V>, tr: Tree<K,V>) -> Tree<K,V> {
            if tl.is_empty() {
                tr
            } else if tr.is_empty() {
                tl
            } else if tl.is_red() && tr.is_red() {
                let bc = append(tl.right(), tr.left());
                if bc.is_red() {
                    Tree::red(bc.key(), bc.value(), 
                              Tree::red(tl.key(), tl.value(), tl.left(), bc.left()),
                              Tree::red(tr.key(), tr.value(), bc.right(), tr.right()))
                } else {
                    Tree::red(tl.key(), tl.value(), tl.left(), Tree::red(tr.key(), tr.value(), bc, tr.right()))
                }
            } else if tl.is_black() && tr.is_black() {
                let bc = append(tl.right(), tr.left());
                if bc.is_red() {
                    Tree::red(bc.key(), bc.value(), 
                              Tree::black(tl.key(), tl.value(), tl.left(), bc.left()),
                              Tree::black(tr.key(), tr.value(), bc.right(), tr.right()))
                } else {
                    balance_left(tl.key(), tl.value(), tl.left(), Tree::black(tr.key(), tr.value(), bc, tr.right()))
                }
            } else if tr.is_red() {
                Tree::red(tr.key(), tr.value(), append(tl, tr.left()), tr.right())
            } else if tl.is_red() {
                Tree::red(tl.key(), tl.value(), tl.left(), append(tl.right(), tr))
            } else {
                unreachable!()
            }
        }

        if self.is_empty() {
            Tree(None)
        } else {
            match k.cmp(&self.key()) {
                Ordering::Less =>
                    if self.left().is_black() {
                        balance_left(self.key(), self.value(), self.left().removed(k), self.right()) 
                    } else {
                        Tree::red(self.key(), self.value(), self.left().removed(k), self.right())
                    },
                Ordering::Greater =>
                    if self.right().is_black() {
                        balance_right(self.key(), self.value(), self.left(), self.right().removed(k))
                    } else {
                        Tree::red(self.key(), self.value(), self.left(), self.right().removed(k))
                    },
                Ordering::Equal => append(self.left(), self.right())
            }
        }
    }

    fn lookup(&self, k: K) -> Self {
        let mut tree = self.clone();
        while !tree.is_empty() {
            match k.cmp(&tree.key()) {
                Ordering::Less => tree = tree.left(),
                Ordering::Greater => tree = tree.right(),
                Ordering::Equal => break,
            }
        }
        tree
    }

    pub fn contains(&self, k: K) -> bool {
        !self.lookup(k).is_empty()
    }

    pub fn get(&self, k: K) -> Option<V> {
        self.lookup(k).0.map(|n| n.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test] 
    fn construction() {
        let tree = (0..10).fold(Tree(None), |acc, e| acc.updated(e, (), false));
        for i in 0..10 {
            assert!(tree.contains(i));
            assert!(!tree.removed(i).contains(i));
        }
        assert!(!tree.contains(-1));
        assert!(!tree.contains(10))
    }
}

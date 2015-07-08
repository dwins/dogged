use std::clone::Clone;
use std::iter::{ FromIterator, IntoIterator };
use std::ops::Index;
use std::rc::Rc;

pub struct Vector<T> {
    root: Rc<Tree32<T>>,
    size: u32,
    shift: u32
}

impl <T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            root: Tree32::new(),
            size: 0,
            shift: 0,
        }
    }

    pub fn append(&self, value: &T) -> Vector<T> where T : Clone {
        if self.size >> self.shift == 32 {
            let root = Tree32::deepen(&self.root);
            let shift = self.shift + 5;
            let root = root.update(shift, self.size, &Some(value));
            Vector {
                root: root,
                size: self.size + 1,
                shift: shift
            }
        } else {
            let root = self.root.update(self.shift, self.size, &Some(value));
            Vector {
                root: root,
                size: self.size + 1,
                shift: self.shift
            }
        }
    }
}

impl <A> FromIterator<A> for Vector<A> where A : Clone {
    fn from_iter<T>(iterator: T) -> Self where T : IntoIterator<Item=A> {
        iterator.into_iter().fold(Vector::new(), |v, t| v.append(&t))
    }
}

impl <T> Index<u32> for Vector<T> {
    type Output = T;
    fn index<'a>(&'a self, index: u32) -> &'a Self::Output {
        self.root.lookup(self.shift, index).unwrap()
    }
}

enum Tree32<T> {
    Root([Option<Rc<Tree32<T>>>; 32]),
    Leaf(T)
}

impl <T> Tree32<T> {
    fn new() -> Rc<Tree32<T>> {
        Rc::new(Tree32::Root([
             None, None, None, None, None, None, None, None, None, None, None,
             None, None, None, None, None, None, None, None, None, None, None,
             None, None, None, None, None, None, None, None, None, None, ]))
    }

    fn get_or_create_subtree(children: &[Option<Rc<Tree32<T>>>;32], i: u32) -> Rc<Tree32<T>> {
        children[i as usize].as_ref().map_or_else(Tree32::new, |c| c.clone())
    }

    fn replace_subtree(&self, index: u32, subtree: &Rc<Tree32<T>>) -> Rc<Tree32<T>> {
        if let &Tree32::Root(ref children) = self {
            let children = [
                if index == 0 { Some(subtree.clone()) } else { children[0].clone() },
                if index == 1 { Some(subtree.clone()) } else { children[1].clone() },
                if index == 2 { Some(subtree.clone()) } else { children[2].clone() },
                if index == 3 { Some(subtree.clone()) } else { children[3].clone() },
                if index == 4 { Some(subtree.clone()) } else { children[4].clone() },
                if index == 5 { Some(subtree.clone()) } else { children[5].clone() },
                if index == 6 { Some(subtree.clone()) } else { children[6].clone() },
                if index == 7 { Some(subtree.clone()) } else { children[7].clone() },
                if index == 8 { Some(subtree.clone()) } else { children[8].clone() },
                if index == 9 { Some(subtree.clone()) } else { children[9].clone() },
                if index == 10 { Some(subtree.clone()) } else { children[10].clone() },
                if index == 11 { Some(subtree.clone()) } else { children[11].clone() },
                if index == 12 { Some(subtree.clone()) } else { children[12].clone() },
                if index == 13 { Some(subtree.clone()) } else { children[13].clone() },
                if index == 14 { Some(subtree.clone()) } else { children[14].clone() },
                if index == 15 { Some(subtree.clone()) } else { children[15].clone() },
                if index == 16 { Some(subtree.clone()) } else { children[16].clone() },
                if index == 17 { Some(subtree.clone()) } else { children[17].clone() },
                if index == 18 { Some(subtree.clone()) } else { children[18].clone() },
                if index == 19 { Some(subtree.clone()) } else { children[19].clone() },
                if index == 20 { Some(subtree.clone()) } else { children[20].clone() },
                if index == 21 { Some(subtree.clone()) } else { children[21].clone() },
                if index == 22 { Some(subtree.clone()) } else { children[22].clone() },
                if index == 23 { Some(subtree.clone()) } else { children[23].clone() },
                if index == 24 { Some(subtree.clone()) } else { children[24].clone() },
                if index == 25 { Some(subtree.clone()) } else { children[25].clone() },
                if index == 26 { Some(subtree.clone()) } else { children[26].clone() },
                if index == 27 { Some(subtree.clone()) } else { children[27].clone() },
                if index == 28 { Some(subtree.clone()) } else { children[28].clone() },
                if index == 29 { Some(subtree.clone()) } else { children[29].clone() },
                if index == 30 { Some(subtree.clone()) } else { children[30].clone() },
                if index == 31 { Some(subtree.clone()) } else { children[31].clone() },
            ];
            Rc::new(Tree32::Root(children))
        } else {
            panic!("Tried to replace child of Leaf")
        }
    }

    fn deepen(root: &Rc<Tree32<T>>) -> Rc<Tree32<T>> {
        Rc::new(Tree32::Root([
             Some(root.clone()), None, None, None, None, None, None, None, None, None, None,
             None, None, None, None, None, None, None, None, None, None, None,
             None, None, None, None, None, None, None, None, None, None, ]))
    }

    fn lookup<'a> (&'a self, shift: u32, index: u32) -> Option<&'a T> {
        if let &Tree32::Root(ref children) = self {
            let idx = (index >> shift) & 0b11111;
            if shift > 0 {
                match children[idx as usize].as_ref() {
                    Some(ref child) => child.lookup(shift - 5, index),
                    None => None
                }
            } else {
                match children[idx as usize] {
                    Some(ref child) =>
                        if let Tree32::Leaf(ref t) = **child {
                            Some(t)
                        } else {
                            panic!("Lookup ended at non-leaf node");
                        },
                    None => None
                }
            }
        } else {
            panic!("Tried to do direct lookup on Leaf");
        }
    }

    fn update(&self, shift: u32, index: u32, value: &Option<&T>) -> Rc<Tree32<T>>
        where T : Clone {
        if let &Tree32::Root(ref children) = self {
            let idx = (index >> shift) & 0b11111;
            if shift > 0 {
                let subtree = Tree32::get_or_create_subtree(children, idx);
                let subtree = subtree.update(shift - 5, index, value);
                self.replace_subtree(idx, &subtree)
            } else {
                let subtree = value.map_or_else(Tree32::new, |v| Rc::new(Tree32::Leaf(v.clone())));
                self.replace_subtree(idx, &subtree)
            }
        } else {
            panic!("Requested direct update to Leaf!")
        }
    }
}


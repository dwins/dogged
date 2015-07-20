use std::fmt::{ Debug, Error, Formatter };
use std::ops::Deref; 
use std::rc::Rc;

#[derive(Clone)]
pub struct List<T>(Rc<Node<T>>);

impl<T> List<T> {
    pub fn new() -> List<T> {
        List(Node::nil())
    }

    pub fn from_slice(xs: &[T]) -> List<T>
        where T : Clone {
        let node = xs.iter().rev().cloned().fold(Node::nil(), Node::cons);
        List(node)
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator(self.0.deref())
    }

    pub fn head_tail(&self) -> Option<(&T, List<T>)> {
        match self.0.deref() {
            &Node::Cons(ref t, ref tail) => Some((t, List(tail.clone()))),
            &Node::Nil => None
        }
    }

    pub fn head(&self) -> Option<&T> {
        match self.0.deref() {
            &Node::Cons(ref t, _) => Some(t),
            &Node::Nil => None,
        }
    }

    pub fn tail(&self) -> Option<List<T>> {
        match self.0.deref() {
            &Node::Cons(_, ref tail) => Some(List(tail.clone())),
            &Node::Nil => None
        }
    }

    pub fn cons(&self, head: T) -> List<T> {
        let tail = self.0.clone();
        List(Node::cons(tail, head))
    }

    pub fn reverse(&self) -> List<T>
        where T : Clone {
        let node = self.iter().cloned().fold(Node::nil(), Node::cons);
        List(node)
    }
}

impl <T> Debug for List<T> where T : Debug {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try! { fmt.write_str("[") };
        let mut node = self.0.clone();
        let mut first = true;
        loop {
            let next = if let Node::Cons(ref t, ref next) = *node {
                if first {
                    first = false;
                    try! { Debug::fmt(t, fmt) }
                } else {
                    try! { fmt.write_fmt(format_args!(", {:?}", t)) };
                }
                next.clone()
            } else {
                break;
            };
            node = next;
        }
        try! { fmt.write_str("]") };
        Ok(())
    }
}

enum Node<T> {
    Cons(T, Rc<Node<T>>),
    Nil,
}

impl<T> Node<T> {
    fn nil() -> Rc<Node<T>> {
        Rc::new(Node::Nil)
    }

    fn cons(tail: Rc<Node<T>>, head: T) -> Rc<Node<T>> {
        Rc::new(Node::Cons(head, tail))
    }
}

pub struct ListIterator<'a, T>(&'a Node<T>)
where T : 'a;

impl <'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.0 {
            &Node::Cons(ref t, ref rest) => {
                self.0 = rest;
                Some(t)
            },
            &Node::Nil => None
        }
    }
}

#[macro_export]
macro_rules! list {
    () => {{ $crate::cons::List::new() }};
    ($x : expr) => {{ list![].cons($x) }};
    ($x : expr,) => {{ list![].cons($x) }};
    ($x : expr, $($xs : expr),* ,) => {{
        list![$($xs),*].cons($x)
    }};
    ($x : expr, $($xs : expr),*) => {{
        list![$($xs),*].cons($x)
    }};
}

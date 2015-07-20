use std::fmt::{ Debug, Error, Formatter };
use std::rc::Rc;

#[derive(Clone)]
pub struct List<T>(Option<Rc<Cons<T>>>);

struct Cons <T> (T, List<T>);

impl<T> List<T> where T : Clone {
    pub fn new() -> List<T> {
        List(None)
    }

    pub fn from_slice(xs: &[T]) -> List<T>
        where T : Clone {
        xs.iter()
          .rev()
          .cloned()
          .fold(List(None), |l, e| l.cons(e))
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator(self)
    }

    pub fn head_tail(&self) -> Option<(&T, List<T>)> {
        self.0.as_ref().map(|c| (&c.0, c.1.clone()))
    }

    pub fn head(&self) -> Option<&T> {
        self.0.as_ref().map(|c| &c.0)
    }

    pub fn tail(&self) -> Option<List<T>> {
        self.0.as_ref().map(|c| c.1.clone())
    }

    pub fn cons(&self, head: T) -> List<T> {
        List(Some(Rc::new(Cons(head, self.clone()))))
    }

    pub fn reverse(&self) -> List<T>
        where T : Clone {
        self.iter()
            .cloned()
            .fold(List(None), |a,e| a.cons(e))
    }
}

impl <T> Debug for List<T> where T : Debug {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try! { fmt.write_str("[") };
        let mut list = self.clone();
        let mut first = true;
        loop {
            let next = list.0.as_ref().and_then(|p| {
                let Cons(ref t, ref rest) = **p;
                Some((t, rest))
            });
            match next {
                Some((t, rest)) => {
                    if first { 
                        first = false;
                        try! { Debug::fmt(t, fmt) };
                    } else {
                        try! { fmt.write_fmt(format_args!(", {:?}", t)) };
                    }
                    list = rest
                },
                None => break,
            }
        }
        try! { fmt.write_str("]") };
        Ok(())
    }
}

pub struct ListIterator<'a, T>(&'a List<T>)
where T : 'a;

impl <'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.0 {
            &List(Some(ref ptr)) => {
                self.0 = &ptr.1;
                Some(&ptr.0)
            },
            &List(None) => None,
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

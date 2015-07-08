#[macro_use] extern crate persevere;
use persevere::cons::*;
use std::clone::Clone;

#[test]
fn list_head() {
    let list = list![1,2,3];
    assert_eq!(*(list.head().unwrap()), 1);
}

#[test]
fn nil_head() {
    let list: List<u32> = list![];
    assert!(list.head().is_none());
}

#[test]
fn list_tail() {
    let list = list![1,2,3];
    let tail: Vec<u32> = list.tail().unwrap().iter().map(Clone::clone).collect();
    assert_eq!(tail, vec![2,3]);
}

#[test]
fn nil_tail() {
    let list: List<u32> = list![];
    assert!(list.tail().is_none());
}

#[test]
fn stack_evaluator() {
    #[derive(Clone)]
    enum Expr {
        Value(i32),
        Add,
        Sub,
        Mul,
        Div
    }

    fn help(expr: List<Expr>) -> Option<(i32, List<Expr>)> {
        match expr.head_tail() {
            Some((&Expr::Value(v), tail)) => Some((v.clone(), tail)),
            Some((expr, tail)) => 
                if let Some((x, t1)) = help(tail) {
                    if let Some((y, t2)) = help(t1) {
                        let v = match expr {
                            &Expr::Add => x + y,
                            &Expr::Sub => x - y,
                            &Expr::Mul => x * y,
                            &Expr::Div => x / y,
                            &Expr::Value(_) => unreachable!(),
                        };
                        Some((v, t2))
                    } else {
                        None
                    }
                } else {
                    None
                },
            None => None
        }
    }

    fn eval(expr: List<Expr>) -> Option<i32> {
        help(expr).map(|p| p.0)
    }
    let operands = list![Expr::Value(5), Expr::Value(-22)];
    let add = operands.cons(Expr::Add);
    let mul = operands.cons(Expr::Mul);
    let sub = operands.cons(Expr::Sub);
    let div = operands.cons(Expr::Div);
    assert_eq!(eval(add).unwrap(), -17);
    assert_eq!(eval(mul).unwrap(), -110);
    assert_eq!(eval(sub).unwrap(), 27);
    assert_eq!(eval(div).unwrap(), 0);
}

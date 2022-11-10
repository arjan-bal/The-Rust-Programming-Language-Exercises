use std::ops::Deref;

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// Implement a struct with functionality similar to Box.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Return with & as *MyBox is converted to
        // *(MyBox.deref()) by Rust compiler.
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {name}");
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");

    // use of normal references
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(x, *y);

    // Do the same thing using Box.
    let y = Box::new(x);
    assert_eq!(x, *y);

    // Do the same with MyBox.
    let y = MyBox::new(x);
    assert_eq!(x, *y);

    // Two levels of deref coercion happening here:
    // MyBox -> String -> str
    hello(&MyBox::new(String::from("Arjan")));
}

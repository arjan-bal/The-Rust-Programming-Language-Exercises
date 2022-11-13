use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};
use crate::ListRc::{ConsRc, NilRc};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
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

    // Two levels of derf coercion happening here:
    // MyBox -> String -> str
    hello(&MyBox::new(String::from("Arjan")));

    // Need to create the following structure using Cons:
    // b-----
    //      |
    // c----a
    let a = Rc::new(ConsRc(
        1,
        Rc::new(ConsRc(2, Rc::new(ConsRc(3, Rc::new(NilRc))))),
    ));
    println!("Reference count for a before creating b: {}", Rc::strong_count(&a));
    let b = ConsRc(4, Rc::clone(&a));
    println!("Reference count for a after creating b: {}", Rc::strong_count(&a));
    {
        let c = ConsRc(5, Rc::clone(&a));
        println!("Reference count of a after creating c: {}", Rc::strong_count(&a));
    }
    println!("Reference count of a after c goes out of scope: {}", Rc::strong_count(&a));
    
}

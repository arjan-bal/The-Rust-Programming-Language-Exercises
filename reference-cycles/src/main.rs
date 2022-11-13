use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    children: RefCell<Vec<Rc<Node>>>,
    value: i32,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 1,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "leaf strong {} weak {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 2,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("Parent of leaf is {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong {} weak {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        println!(
            "branch strong {} weak {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }
    println!("Parent of leaf is {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong {} weak {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

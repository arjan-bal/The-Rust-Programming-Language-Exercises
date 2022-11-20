use std::vec;

use trait_for_oops::{Button, Draw, Screen};

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("I am a SelectBox with props: {:?}", &self);
    }
}

fn main() {
    let my_scren = Screen {
        components: vec![
            Box::new(Button {
                height: 10,
                width: 20,
                label: String::from("Press me!"),
            }),
            Box::new(SelectBox {
                width: 15,
                height: 25,
                options: vec![
                    String::from("Some"),
                    String::from("boring"),
                    String::from("options"),
                ],
            }),
        ],
    };
    my_scren.run();
}

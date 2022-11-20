pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("I am a Button with info: {:?}", &self);
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for item in &self.components {
            item.draw();
        }
    }
}

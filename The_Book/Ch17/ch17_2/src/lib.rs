pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    //dyn Draw is a stand in for any type inside Box that implements the draw trait
    //doing things this way utilizes dynamic dispatch, a determination made at runtime to see which method to call
    //this incurs penalties to runtime cost, and some optimizations
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
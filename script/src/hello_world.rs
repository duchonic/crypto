use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Coin;

#[methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        //godot_print!("hello, rust");
    }
}

#[methods]
impl Coin {
    fn new(_owner: &Node) -> Self {
        Coin
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        //godot_print!("hello, coin rust");
    }
}

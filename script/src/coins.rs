use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Coins;

#[methods]
impl Coins {
    fn new(_owner: &Node) -> Self {
        Coins
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, this is rust from coins");
    }
}


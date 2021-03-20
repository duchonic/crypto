#[macro_use]
extern crate gdnative;

use gdnative::prelude::*;


mod hello_world;
mod coins;

use hello_world::Coin;
use hello_world::HelloWorld;
use coins::Coins;

fn init(handle: InitHandle) {
  handle.add_class::<Coins>();
  handle.add_class::<HelloWorld>();
  handle.add_class::<Coin>()
}

godot_init!(init);
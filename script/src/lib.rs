#[macro_use]
extern crate gdnative;

use gdnative::prelude::*;

mod hello_world;
mod coins;
mod travel_menu;
mod game_state;
mod stat;

use hello_world::Coin;
use hello_world::HelloWorld;
use coins::Coins;
use travel_menu::Travelmenu;


fn init(handle: InitHandle) {
  handle.add_class::<Coins>();
  handle.add_class::<HelloWorld>();
  handle.add_class::<Coin>();
  handle.add_class::<Travelmenu>();
  handle.add_class::<game_state::GameState >();
  handle.add_class::<stat::Stat>();
  handle.add_class::<travel_menu::End>();
}

godot_init!(init);
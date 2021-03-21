use gdnative::prelude::*;
use crate::{game_state::load_game_state};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Travelmenu;

#[methods]
impl Travelmenu {
    fn new(_owner: &Node) -> Self {
        Travelmenu
    }

    #[export]
    fn _ready(&self, _owner: TRef<Node>) {
        godot_print!("hello, this is the travel menu");
    }

    #[export]
    fn _process(&mut self, _owner: &Node, _delta: f32) {
        let input = Input::godot_singleton();
        if Input::is_action_just_released(&input, "ui_left") {
            godot_print!("left pressed");
        }
        else if Input::is_action_just_released(&input, "ui_right") {
            godot_print!("right pressed");
        }
    }


    #[export]
    fn _on_home_pressed(&self, owner: &Node) {
        godot_print!("home pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state.map_mut(|gs, _| gs.travel_to(&owner, 1)).expect("could not travel to home");
    }
    #[export]
    fn _on_zurich_pressed(&self, owner: &Node) {
        godot_print!("zurich pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state.map_mut(|gs, _| gs.travel_to(&owner, 2)).expect("could not travel to home");
    }
    #[export]
    fn _on_tokyo_pressed(&self, owner: &Node) {
        godot_print!("tokyo pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state.map_mut(|gs, _| gs.travel_to(&owner, 5)).expect("could not travel to home");
    }
    #[export]
    fn _on_moskow_pressed(&self, owner: &Node) {
        godot_print!("moskow pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state.map_mut(|gs, _| gs.travel_to(&owner, 3)).expect("could not travel to home");
    }
    #[export]
    fn _on_sydney_pressed(&self, owner: &Node) {
        godot_print!("sydney pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state.map_mut(|gs, _| gs.travel_to(&owner, 4)).expect("could not travel to home");
    }
}


#[derive(NativeClass)]
#[inherit(Node)]
pub struct End;

#[methods]
impl End {
    fn new(_owner: &Node) -> Self {
        End
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("end reached");
    }

    #[export]
    fn _on_restart_pressed(&self, owner: &Node) {
        godot_print!("restart pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state.map_mut(|gs, _| gs.restartgame(&owner)).expect("could not travel to home");
        rust_game_state.map_mut(|gs, _| gs.travel_to(&owner, 1)).expect("could not travel to home");
    }
}

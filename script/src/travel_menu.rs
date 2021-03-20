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
    fn _ready(&self, _owner: &Node) {
        //godot_print!("hello, this is the travel menu");
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
        rust_game_state
            .map_mut(|gs, _| gs.decrement_day(&owner))
            .expect("Could not decrement days");

        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://cities/home.tscn")
                .expect("Game Scene could not be changed!");
        }
    }
    #[export]
    fn _on_zurich_pressed(&self, owner: &Node) {
        godot_print!("zurich pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.decrement_day(&owner))
            .expect("Could not decrement days");

        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://cities/zurich.tscn")
                .expect("Game Scene could not be changed!");
        }
    }
    #[export]
    fn _on_tokyo_pressed(&self, owner: &Node) {
        godot_print!("tokyo pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.decrement_day(&owner))
            .expect("Could not decrement days");

        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://cities/tokyo.tscn")
                .expect("Game Scene could not be changed!");
        }
    }
    #[export]
    fn _on_moskow_pressed(&self, owner: &Node) {
        godot_print!("moskow pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, _| gs.decrement_day(&owner))
            .expect("Could not decrement days");


        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://cities/moskow.tscn")
                .expect("Game Scene could not be changed!");
        }
    }


    #[export]
    fn _on_sydney_pressed(&self, owner: &Node) {
        godot_print!("sydney pressed");
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, _| gs.decrement_day(&owner))
            .expect("Could not decrement days");

        godot_print!("sydney chaged");
        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://cities/sydney.tscn")
                .expect("Game Scene could not be changed!");
        }
    }
}
use gdnative::prelude::*;
use crate::{game_state::load_game_state};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Stat;

#[methods]
impl Stat {
    fn new(_owner: &Node) -> Self {
        return Stat;
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Stat is started");
    }

    #[export]
    fn _on_timer_timeout(&self, owner: &Node) {

        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, _| gs.update(&owner))
            .expect("could not read days");

        let days = rust_game_state
            .map_mut(|gs, _| gs.get_day())
            .expect("could not read days");

        if let Some(button) = owner.get_node("./stat/stats/daysleft").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(days.to_string());
        };

        let money = rust_game_state
            .map_mut(|gs, _| gs.get_money())
            .expect("could not read money");

        if let Some(button) = owner.get_node("./stat/stats/money").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(money.to_string());
        };

        let credit = rust_game_state
            .map_mut(|gs, _| gs.get_credit())
            .expect("could not read credit");

        if let Some(button) = owner.get_node("./stat/stats/credit").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(credit.to_string());
        };


        let bitcoin = rust_game_state
            .map_mut(|gs, _| gs.get_btc())
            .expect("could not read bitcoin");

        if let Some(button) = owner.get_node("./coin/coins/btc").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(bitcoin.to_string());
        };

        let etherum = rust_game_state
            .map_mut(|gs, _| gs.get_eth())
            .expect("could not read etherum");
        if let Some(button) = owner.get_node("./coin/coins/eth").and_then(|node| {
                let node = unsafe { node.assume_safe() };
                node.cast::<Button>()
        }) {
                let button = unsafe { button.assume_unique() };
                button.set_text(etherum.to_string());
        };

        let litecoin = rust_game_state
            .map_mut(|gs, _| gs.get_ltc())
            .expect("could not read litecoin");
        if let Some(button) = owner.get_node("./coin/coins/ltc").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(litecoin.to_string());
        };

        let mybtc = rust_game_state
            .map_mut(|gs, _| gs.get_my_btc())
            .expect("could not read my bitcoin");
        if let Some(button) = owner.get_node("./wallet/coins/btc").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(mybtc.to_string());
        };

        let myeth = rust_game_state
            .map_mut(|gs, _| gs.get_my_eth())
            .expect("could not read my etherum");
        if let Some(button) = owner.get_node("./wallet/coins/eth").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(myeth.to_string());
        };
        let myltc = rust_game_state
            .map_mut(|gs, _| gs.get_my_ltc())
            .expect("could not read my litecoin");
        if let Some(button) = owner.get_node("./wallet/coins/ltc").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(myltc.to_string());
        };
    }


    #[export]
    fn _on_buy_btc_pressed(&self, owner: &Node) {
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.buy_btc())
            .expect("could not read days");
    }

    #[export]
    fn _on_buy_eth_pressed(&self, owner: &Node) {
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.buy_eth())
            .expect("eth buy failed");
    }


    #[export]
    fn _on_buy_ltc_pressed(&self, owner: &Node) {
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.buy_ltc())
            .expect("ltc buy failed");
    }

    #[export]
    fn _on_sell_btc_pressed(&self, owner: &Node) {
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.sell_btc())
            .expect("could not read days");
    }

    #[export]
    fn _on_sell_eth_pressed(&self, owner: &Node) {
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.sell_eth())
            .expect("eth sell failed");
    }


    #[export]
    fn _on_sell_ltc_pressed(&self, owner: &Node) {
        let rust_game_state = load_game_state(owner).expect("Failed to get game state instance");
        rust_game_state
            .map_mut(|gs, _| gs.sell_ltc())
            .expect("ltc buy failed");
    }

}


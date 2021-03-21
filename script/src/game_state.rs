use gdnative::prelude::*;
use rand::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameState{
    pub day: u8,
    pub money: i32,
    pub credit: i32,
    pub health: i32,
    pub btc_price: i32,
    pub eth_price: i32,
    pub ltc_price: i32,
    pub my_btc: i32,
    pub my_eth: i32,
    pub my_ltc: i32,
}

#[methods]
impl GameState {
    fn new(_owner: &Node) -> Self {
        GameState{
            day: 10,
            money: 8000,
            credit: 4800,
            health: 100,
            btc_price: 10_000,
            eth_price: 1_000,
            ltc_price: 100,
            my_btc: 0,
            my_eth: 0,
            my_ltc: 0,
        }
    }

    #[export]
    fn _ready(&self, _owner: TRef<Node>) {

    }

    #[export]
    pub fn restartgame(&mut self, _owner: &Node){
        godot_print!("restart game in rust, yeah");
        self.day = 11;
        self.money = 8000;
        self.credit = 4000;
        self.health = 100;
        self.btc_price = 10_000;
        self.eth_price = 1_000;
        self.ltc_price = 100;
        self.my_btc = 0;
        self.my_eth = 0;
        self.my_ltc = 0;
    }



    #[export]
    pub fn update(&mut self, owner: &Node) {

        let mut rng = rand::thread_rng();

        let btcchange : u8 = rng.gen();

        if btcchange > 128 {
            self.btc_price += 1;
        }
        else if self.btc_price > 0 {
            self.btc_price -= 1;

        }

        let ethchange : bool = rng.gen();
        if ethchange{
            self.eth_price += 1;
        }
        else if self.eth_price > 0 {
            self.eth_price -= 1;
        }

        let ltcchange : bool = rng.gen();
        if ltcchange {
            self.ltc_price += 1;
        }
        else if self.ltc_price > 0{
            self.ltc_price -= 1;
        }



        if let Some(button) = owner.get_node("../gameover/score").and_then(|node| {
            let node = unsafe { node.assume_safe() };
            node.cast::<Button>()
        }) {
            let button = unsafe { button.assume_unique() };
            button.set_text(self.money.to_string());
        };

    }

    pub fn get_money(&self) -> i32 {
        self.money
    }
    pub fn get_credit(&self) -> i32 {
        self.credit
    }


    fn update_price_and_credit(&mut self){



        let mut rng = rand::thread_rng();

        let btcchange : f32 = rng.gen();
        self.btc_price = ((self.btc_price as f32) * (btcchange + 0.5)) as i32;
        let ethchange : f32 = rng.gen();
        self.eth_price = ((self.eth_price as f32) * (ethchange + 0.5)) as i32;
        let ltcchange : f32 = rng.gen();
        self.ltc_price = ((self.ltc_price as f32) * (ltcchange + 0.5)) as i32;

        if self.credit > 0 {
            self.credit = ( (self.credit as f32) * 1.2 ) as i32;
            godot_print!("actual credit: {}", self.credit);
        }
        else{
            godot_print!("credit: {}", self.credit);
        }


    }
    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn get_btc(&self) -> i32 {
        self.btc_price
    }
    pub fn get_eth(&self) -> i32 {
        self.eth_price
    }
    pub fn get_ltc(&self) -> i32 {
        self.ltc_price
    }

    pub fn get_my_btc(&self) -> i32 {
        self.my_btc
    }
    pub fn get_my_eth(&self) -> i32 {
        self.my_eth
    }
    pub fn get_my_ltc(&self) -> i32 {
        self.my_ltc
    }


    pub fn buy_btc(&mut self){
        if self.money > self.btc_price && self.day > 0 {
            self.money -= self.btc_price;
            self.my_btc += 1;
        }
    }
    pub fn buy_eth(&mut self) {
        if self.money > self.eth_price && self.day > 0 {
            self.money -= self.eth_price;
            self.my_eth += 1;
        }
    }
    pub fn buy_ltc(&mut self){
        if self.money > self.ltc_price && self.day > 0 {
            self.money -= self.ltc_price;
            self.my_ltc += 1;
        }
    }
    pub fn sell_btc(&mut self){
        if self.my_btc > 0 {
            godot_print!("sell at: {}", self.btc_price);
            self.money += self.btc_price;
            self.my_btc -= 1;
        }
    }
    pub fn sell_eth(&mut self){
        if self.my_eth > 0 {
            self.money += self.eth_price;
            self.my_eth -= 1;
        }
    }
    pub fn sell_ltc(&mut self){
        if self.my_ltc > 0 {
            self.money += self.ltc_price;
            self.my_ltc -= 1;
        }
    }
    pub fn buy_credit(&mut self){
        if self.day > 0 {
            self.money += 1000;
            self.credit += 1000;
        }
    }
    pub fn kill_credit(&mut self){
        if self.credit >= 1000 && self.money >= 1000 {
            self.credit -= 1000;
            self.money -= 1000;
        }
        else if self.money > self.credit {
            self.money -= self.credit;
            self.credit = 0;
        }

    }



    #[export]
    pub fn travel_to(&mut self, owner: &Node, city: u8){
        if self.day > 0 {
            self.day -= 1;

            GameState::update_price_and_credit(self);

            if let Some(tree) = &owner.get_tree() {
                let tree = unsafe { tree.assume_safe() };
                match city {
                    // Match a single value
                    1 => tree.change_scene("res://cities/home.tscn").expect("Game Scene could not be changed!"),
                    2 => tree.change_scene("res://cities/zurich.tscn").expect("Game Scene could not be changed!"),
                    3 => tree.change_scene("res://cities/moskow.tscn").expect("Game Scene could not be changed!"),
                    4 => tree.change_scene("res://cities/sydney.tscn").expect("Game Scene could not be changed!"),
                    5 => tree.change_scene("res://cities/tokyo.tscn").expect("Game Scene could not be changed!"),
                    _ => godot_print!("citiy not found"),
                }
            }
        }
        else {
            godot_print!("its overs");

            self.money -= self.credit;
            self.credit = 0;

            if let Some(tree) = &owner.get_tree() {
                let tree = unsafe { tree.assume_safe() };
                tree.change_scene("res://cities/gameover.tscn")
                    .expect("Game Scene could not be changed!");
            }
        }


    }


}




pub fn load_game_state(node: &Node) -> Option<Instance<GameState, Unique>> {
    let tree = node.get_tree()?;
    let tree = unsafe { tree.assume_safe() };

    let root = tree.root()?;
    let root = unsafe { root.assume_safe() };

    let game_state_node = root.get_node("./GameState")?;
    let game_state_node = unsafe { game_state_node.assume_unique() };

    Instance::<GameState, _>::from_base(game_state_node)
}
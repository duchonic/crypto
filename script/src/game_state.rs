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
            day: 7,
            money: 8000,
            credit: -4000,
            health: 100,
            btc_price: 1_000,
            eth_price: 1_000,
            ltc_price: 1_000,
            my_btc: 0,
            my_eth: 0,
            my_ltc: 0,
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        //godot_print!("game state ready");
    }

    #[export]
    pub fn update(&mut self, _owner: &Node) {
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

    }

    pub fn get_money(&self) -> i32 {
        self.money
    }
    pub fn get_credit(&self) -> i32 {
        self.credit
    }

    #[export]
    pub fn decrement_day(&mut self, owner: &Node) {

        if self.day == 0 {
            godot_print!("game over");




            if let Some(tree) = &owner.get_tree() {
                let tree = unsafe { tree.assume_safe() };
                tree.change_scene("res://cities/gameover.tscn")
                    .expect("Game Scene could not be changed!");
            }

            return;
        }

        let mut rng = rand::thread_rng();

        let btcchange : f32 = rng.gen();
        self.btc_price = ((self.btc_price as f32) * (btcchange + 0.5)) as i32;
        let ethchange : f32 = rng.gen();
        self.eth_price = ((self.eth_price as f32) * (ethchange + 0.5)) as i32;
        let ltcchange : f32 = rng.gen();
        self.ltc_price = ((self.ltc_price as f32) * (ltcchange + 0.5)) as i32;

        if self.credit < 0 {
            self.credit = ( (self.credit as f32) * 1.1 ) as i32;
            //godot_print!("credit {}", self.credit);
        }



        if self.day > 0 {
            self.day -= 1;
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
        if self.money > self.btc_price {
            self.money -= self.btc_price;
            self.my_btc += 1;
        }
    }
    pub fn buy_eth(&mut self) {
        if self.money > self.eth_price {
            self.money -= self.eth_price;
            self.my_eth += 1;
        }
    }
    pub fn buy_ltc(&mut self){
        if self.money > self.ltc_price {
            self.money -= self.ltc_price;
            self.my_ltc += 1;
        }
    }
    pub fn sell_btc(&mut self){
        if self.my_btc > 0 {
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
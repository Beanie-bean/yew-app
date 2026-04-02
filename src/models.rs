use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub id: usize,
    pub name: AttrValue,
    pub released: AttrValue
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct MyGame {
    pub game_id: String,
    pub name: String,
    pub released: String
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GameToAdd {
    pub name: AttrValue,
    pub released: AttrValue
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct MyList {
    pub _id: String,
    pub name: String,
    pub desc: String,
    pub games: Vec<MyGame>
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Results {
    pub results: Vec<Game>
}

use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub id: u32,
    pub name: AttrValue,
    pub released: AttrValue
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct MyGame {
    pub _id: String,
    pub name: AttrValue,
    pub released: AttrValue
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
    pub next: Option<AttrValue>,
    pub results: Vec<Game>
}

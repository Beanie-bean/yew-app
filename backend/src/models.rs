use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(serialize_with = "mongodb::bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub game_id: ObjectId,
    pub name: String,
    pub year: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyList{
    #[serde(serialize_with = "mongodb::bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub desc: String,
    pub games: Vec<Game>,
}

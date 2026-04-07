use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(default, serialize_with = "mongodb::bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub released: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyList{
    #[serde(default, serialize_with = "mongodb::bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub desc: String,
    pub games: Vec<Game>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameToAdd {
    pub name: String,
    pub released: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateList {
    pub name: String,
    pub desc: String
}

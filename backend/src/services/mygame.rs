use actix_web::web;
use futures::stream::{ StreamExt, TryStreamExt};
use mongodb::{
    Collection, bson::{doc, oid::ObjectId}
};

use crate::models::{MyList, Game};

pub async fn fetch_list(mygames: &Collection<MyList>) -> Result<MyList, mongodb::error::Error> {
    let mut cursor = mygames
        .find(doc! {})
        .await?;
    cursor.next().await.unwrap()
}

pub async fn add_one_game(mygames: &Collection<MyList>, data: web::Json<Game>) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    let mut cursor = mygames
        .find(doc! {})
        .await?;
    let id = cursor.next().await.unwrap()?._id;
    
    mygames.update_one(doc! { "_id": id}, 
        doc! { "$push": doc! {
                    "games": doc! {
                        "game_id": ObjectId::new(),
                        "name": data.clone().name,
                        "released": data.clone().released
                    }
                }}).await
}

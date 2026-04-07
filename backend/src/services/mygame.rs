use actix_web::web;
use futures::stream::{ StreamExt, TryStreamExt};
use mongodb::{
    Collection, bson::{doc, oid::ObjectId}, options::ReturnDocument
};

use crate::models::{Game, GameToAdd, MyList, UpdateList};

pub async fn fetch_list(mygames: &Collection<MyList>) -> Result<MyList, mongodb::error::Error> {
    let mut cursor = mygames
        .find(doc! {})
        .await?;
    cursor.next().await.unwrap()
}

pub async fn edit_list_details(mygames: &Collection<MyList>, data: web::Json<UpdateList>) -> mongodb::error::Result<Option<MyList>> {
    let mut cursor = mygames
        .find(doc! {})
        .await?;
    let id = cursor.next().await.unwrap()?._id;
        
    mygames.find_one_and_update(doc! { "_id": id}, 
        doc! { "$set": {
                    "name": data.clone().name,
                    "desc": data.clone().desc
                }}).return_document(ReturnDocument::After)
                .await
    
}

pub async fn add_one_game(mygames: &Collection<MyList>, data: web::Json<GameToAdd>) -> mongodb::error::Result<Option<MyList>> {
    let mut cursor = mygames
        .find(doc! {})
        .await?;
    let id = cursor.next().await.unwrap()?._id;

    let new_doc = doc! {
                        "_id": ObjectId::new(),
                        "name": data.clone().name,
                        "released": data.clone().released
                    };
    
    mygames.find_one_and_update(doc! { "_id": id}, 
        doc! { "$push": {
                    "games": new_doc
                }}).return_document(ReturnDocument::After)
                .await
    
}

pub async fn delete_one_game(mygames: &Collection<MyList>, game_id: ObjectId) -> mongodb::error::Result<Option<MyList>> {
    let mut cursor = mygames
        .find(doc! {})
        .await?;
    let id = cursor.next().await.unwrap()?._id;
    
    mygames.find_one_and_update(doc! { "_id": id}, 
        doc! { "$pull": {
                    "games": doc! {
                        "_id": game_id,
                    }
                }}).return_document(ReturnDocument::After)
                .await
}

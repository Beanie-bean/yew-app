use actix_web::{HttpResponse, Responder, delete, get, patch, web::{self, Path}};
use mongodb::bson::oid::ObjectId;

use crate::{AppState, models::{Game, GameToAdd}, services::mygame};

//Get all games in list
#[get("/mygames")]
async fn get_list(db: web::Data<AppState>) -> impl Responder {
    let result = mygame::fetch_list(&db.mygames).await;
    match result {
        Ok(rows) => {
            HttpResponse::Ok()
                .json(rows)
                
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {e}")),
    }
}

//Add a new game to list
#[patch("/add")]
async fn add_game(db: web::Data<AppState>, data: web::Json<GameToAdd>) -> impl Responder {
    let result = mygame::add_one_game(&db.mygames, data).await;
    match result {
        Ok(result) => {
            HttpResponse::Ok()
                .json(result)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {e}")),
    }
}

// Delete game from list
#[delete("/{id}")]
async fn delete_game(db: web::Data<AppState>, path: web::Path<ObjectId>) -> impl Responder {
    let id = path.into_inner();
    let result = mygame::delete_one_game(&db.mygames, id).await;
    match result {
        Ok(result) => {
            HttpResponse::Ok()
                .json(result)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {e}")),
    }
}


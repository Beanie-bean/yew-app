use actix_web::{HttpResponse, Responder, get, patch, web};

use crate::{AppState, services::mygame, models::Game};

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
async fn add_game(db: web::Data<AppState>, data: web::Json<Game>) -> impl Responder {
    let result = mygame::add_one_game(&db.mygames, data).await;
    match result {
        Ok(result) => {
            HttpResponse::Ok()
                .json(result)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {e}")),
    }
}


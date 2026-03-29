use actix_web::{get, web, HttpResponse, Responder};

use crate::{AppState, services::mygame};

//Get all games in list
#[get("/mygames")]
async fn get_list(data: web::Data<AppState>) -> impl Responder {
    let result = mygame::fetch_all(&data.mygames).await;
    match result {
        Ok(rows) => {
            HttpResponse::Ok()
                .json(rows)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {e}")),
    }
}


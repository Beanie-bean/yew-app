mod db;
mod models;
mod services;
mod endpoints;

use actix_cors::Cors;
use actix_web::{ web, App, HttpServer };
use dotenv_codegen::dotenv;
use mongodb::bson::doc;
use mongodb::Collection;
use crate::models::MyList;


#[derive(Clone)]
struct AppState {
    mygames: Collection<MyList>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = dotenv!("ATLAS_URI");
    let port = dotenv!("PORT")
        .parse()
        .expect("PORT must be a valid u16 number");

    let db = db::init_db(&uri).await;

    let mygames: Collection<MyList> = db.collection::<MyList>("mygames");

    // Extra ping to be sure connection is working
    let ping_result = db.run_command(doc! {"ping": 1},).await;
    print!("MongoDB ping result: {ping_result:?}\n");

    let state = AppState {mygames};

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(state.clone()))
            .service(endpoints::get_list)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
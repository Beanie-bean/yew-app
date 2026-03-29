use mongodb::{options::ClientOptions, Client, Database};

pub async fn init_db(atlas_uri: &str) -> Database {
    let mut client_options = ClientOptions::parse(atlas_uri)
        .await
        .expect("Failed to parse MongoDB connection string");

    client_options.app_name = Some("yew-app".to_string());

    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");

    client.database("games")
}
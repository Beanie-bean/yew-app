use gloo_net::{http::Request, Error};
use web_sys::{console, wasm_bindgen::JsValue};

use crate::models::{Game, GameToAdd, MyGame, MyList, Results};

pub async fn get_all_games(key: &str, page: u32) -> Result<Results, Error> {
    let fetched_games = Request::get(format!("https://rawg.io/api/games?key={}&page={}", key, page).as_str())
        .send()
        .await;

    match fetched_games {
        Ok(response) => {
            let json = response.json::<Results>().await;
            match json {
                Ok(json_resp) => {
                    return Ok(json_resp);
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn get_my_games() -> Result<MyList, Error> {
    let fetched_list = Request::get("http://localhost:5050/mygames")
        .send()
        .await;

    match fetched_list {
        Ok(response) => {
            let json = response.json::<MyList>().await;
            match json {
                Ok(json_resp) => {
                    return Ok(json_resp);
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn add_game(game: &Game) -> Result<MyGame, Error> {
    let game_to_add =  GameToAdd {
        name: game.clone().name,
        released: game.clone().released
    };

    let response = Request::patch("http://localhost:5050/add")
        .json(&game_to_add)?
        .send()
        .await;
    
    match response {
        Ok(res) => {
            let json = res.json::<MyList>().await;

            match json {
                Ok(json_resp) => {
                    let mygame = json_resp.games.iter().find(|e| e.name == game_to_add.name);
                    let result = mygame.ok_or(Error::GlooError("Failed to add game".to_string()));
                    match result {
                        Ok(game) => return Ok(game.clone()),     
                        Err(e) => Err(Error::GlooError(e.to_string()))
                    }
                }
                Err(e) => {
                    let js_error = JsValue::from_str(&e.to_string());
                    console::log_1(&js_error);
                    return Err(Error::GlooError(e.to_string()));
                }
            }
        }
        Err(e) => Err(Error::GlooError(e.to_string()))

    }
}

pub async fn delete_game(mygame: &MyGame) -> Result<MyGame, Error> {
    let response = Request::delete(format!("http://localhost:5050/{}", mygame._id).as_str())
        .send()
        .await;
    

    match response {
        Ok(res) => {
            let json = res.json::<MyList>().await;

            match json {
                Ok(json_resp) => {
                    let game_deleted = !json_resp.games.contains(&mygame);
                    if game_deleted {
                        return Ok(mygame.clone())
                    }
                    else {
                        return Err(Error::GlooError("Failed to delete game".to_string()))
                    }
                }
                Err(e) => {
                    let js_error = JsValue::from_str(&e.to_string());
                    console::log_1(&js_error);
                    return Err(Error::GlooError(e.to_string()));
                }
            }
        }
        Err(e) => Err(Error::GlooError(e.to_string()))

    }
}
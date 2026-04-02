use gloo_net::{http::Request, Error};
use yew::prelude::*;

use crate::models::{MyList, Results};

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
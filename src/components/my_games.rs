use std::clone;

use yew::prelude::*;
use gloo_net::{http::Request, Error};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Game {
    pub game_id: String,
    pub name: String,
    pub year: String
}

#[derive(Clone, PartialEq, Deserialize)]
struct MyList {
    _id: String,
    name: String,
    desc: String,
    games: Vec<Game>
}

#[component]
pub fn MyGames() -> Html {
    let results: UseStateHandle<Option<Vec<MyList>>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        let results = results.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_list = Request::get("http://localhost:5050/mygames")
                        .send()
                        .await;
                    match fetched_list {
                        Ok(response) => {
                            let json = response.json::<Vec<MyList>>().await;
                            match json {
                                Ok(json_resp) => {
                                    results.set(Some(json_resp));
                                }
                                Err(e) => error.set(Some(e))
                            }
                        }
                        Err(e) => error.set(Some(e))
                    }
                });
                || ()
            }
        );
    }

    let my_games = match results.as_ref() {
        Some(results) => results[0]
            .games
            .iter()
            .map(|game| {
                html!{
                    <tr key={game.game_id.clone()}>
                        <td>{game.name.clone()}</td>
                        <td>{&game.year.clone()[..4]}</td>
                        <td class="d-flex justify-content-center"><button class="btn btn-danger">{"Delete"}</button></td>

                    </tr>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(_) => {
                html! {
                    <p>{error.as_ref().unwrap().to_string()}</p>
                }
            }
            None => {
                html! {
                    <></>
                }
            }
        }
    };

    html! {
        <>
            <h2 class="p-3 d-flex justify-content-center">{"My Games"}</h2>
            if results.as_ref() != None {
                <div class="pt-2 row justify-content-end">
                    <div class="col-4 d-flex justify-content-center">
                        <h5 class="d-flex justify-content-center">{&(*results.as_ref().unwrap()[0].name)}</h5>
                    </div>
                    <div class="col-4 d-flex justify-content-end">
                    </div>
                </div>
                <p class="d-flex justify-content-center">{&(*results.as_ref().unwrap()[0].desc)}</p>
                <div class="d-flex justify-content-center">
                    <div style="width: 50%">
                        <table class="table table-striped align-middle table-bordered">
                            <thead>
                                <tr>
                                    <th>{"Name"}</th>
                                    <th width="25%">{"Release Year"}</th>
                                    <th width="14%"></th>
                                </tr>
                            </thead>
                            <tbody>
                                {my_games}
                            </tbody>    
                        </table>
                    </div>
                </div>
            }
        </>    
    }
}



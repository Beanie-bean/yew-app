use web_sys::console;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;
use gloo_net::{http::Request, Error};
use serde::Deserialize;

use crate::utils::{delete_game, get_my_games};
use crate::models::{MyGame, MyList};

#[component]
pub fn MyGames() -> Html {
    let selected_game = use_state(|| MyGame {_id: "".to_string(), name: "".into(), released: "".into()});

    let results: UseStateHandle<Option<MyList>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        let results = results.clone();
        let error = error.clone();
        
        use_effect_with(selected_game.clone(), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    results.set(get_my_games().await.ok());
                    error.set(get_my_games().await.err());
                });
                || ()
            }
        );
    }

    let my_games = match results.as_ref() {
        Some(mylist) => mylist
            .games
            .iter()
            .map(|game| {
                let game = game.clone();
                let results = results.clone();
                let selected_game = selected_game.clone();

                html!{
                    <tr key={game._id.clone().to_string()}>
                        <td>{game.name.clone()}</td>
                        <td>{&game.released.clone()[..4]}</td>
                        <td class="d-flex justify-content-center">
                            <button 
                                onclick={Callback::from(move |_| {
                                    let game = game.clone();
                                    let results = results.clone();
                                    let selected_game = selected_game.clone();

                                    let mygame = results.as_ref().and_then(|mygames| {
                                        mygames.games.iter()
                                            .find(|e| e.name == game.name && e.released == game.released)
                                            .cloned()
                                    });
                                    
                                    wasm_bindgen_futures::spawn_local(async move {
                                        match delete_game(&mygame.unwrap()).await {
                                            Ok(game) => {
                                                selected_game.set({MyGame {_id: game._id.to_string(), name: game.name.clone(), released: game.released.clone()}});
                                                let newlist = (*results).clone();
                                                if let Some(i) = newlist.clone().unwrap().games.iter().position(|e| e.name == game.name && e.released == game.released) {
                                                    newlist.clone().unwrap().games.remove((i).clone());
                                                    results.set(newlist.clone());
                                                }
                                            },
                                            Err(e) => {
                                                let js_error = JsValue::from_str(&e.to_string());
                                                console::log_1(&js_error);
                                            },
                                        }
                                    });
                                })}
                                class="btn btn-danger">{"Delete"}
                            </button>
                        </td>

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
                        <h5 class="d-flex justify-content-center">{&(*results.as_ref().unwrap().name)}</h5>
                    </div>
                    <div class="col-4 d-flex justify-content-end">
                    </div>
                </div>
                <p class="d-flex justify-content-center">{&(*results.as_ref().unwrap().desc)}</p>
                <div class="d-flex justify-content-center">
                if results.as_ref().unwrap().games.len() != 0 {
                    <div style="min-width: 50%">
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
                }
                </div>
            }
        </>    
    }
}



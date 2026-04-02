use yew::prelude::*;
use gloo_net::{http::Request, Error};
use serde::Deserialize;

use crate::utils::{get_my_games};
use crate::models::{MyGame, MyList};

#[component]
pub fn MyGames() -> Html {
    let results: UseStateHandle<Option<MyList>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        let results = results.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    results.set(get_my_games().await.ok());
                    error.set(get_my_games().await.err());
                });
                || ()
            }
        );
    }

    let my_games = match results.as_ref() {
        Some(results) => results
            .games
            .iter()
            .map(|game| {
                html!{
                    <tr key={game.game_id.clone()}>
                        <td>{game.name.clone()}</td>
                        <td>{&game.released.clone()[..4]}</td>
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
                        <h5 class="d-flex justify-content-center">{&(*results.as_ref().unwrap().name)}</h5>
                    </div>
                    <div class="col-4 d-flex justify-content-end">
                    </div>
                </div>
                <p class="d-flex justify-content-center">{&(*results.as_ref().unwrap().desc)}</p>
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



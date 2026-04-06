use gloo_net::{http::Request, Error};
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;
use dotenv_codegen::dotenv;
use crate::components::pagination::*;
use web_sys::{ScrollToOptions};
use web_sys::ScrollBehavior;
use web_sys::console;

use crate::utils::{get_all_games, get_my_games, add_game};
use crate::models::{Game, MyGame, MyList, Results};


#[component]
pub fn AllGames() -> Html {
    let key = dotenv!("RAWGIO_API_KEY");
    let current_page = use_state(|| 1);
    let added_game = use_state(|| MyGame {_id: "".to_string(), name: "".into(), released: "".into()});

    let on_set_page = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            let options = ScrollToOptions::new();
            options.set_top(0.0);
            options.set_behavior(ScrollBehavior::Smooth);
            web_sys::window().unwrap().scroll_to_with_scroll_to_options(&options);
            current_page.set(page);
        })
    };
    
    let results: UseStateHandle<Option<Results>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        let results = results.clone();
        let error = error.clone();
        let page = current_page.clone();
        
        use_effect_with(page.clone(), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    results.set(get_all_games(key, *page).await.ok());
                    error.set(get_all_games(key, *page).await.err());
                });
                || ()
            }
        );
    }

    let my_games_results: UseStateHandle<Option<MyList>> = use_state(|| None);
    let my_games_error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        let my_games_results = my_games_results.clone();
        let my_games_error = my_games_error.clone();
        
        use_effect_with(added_game.clone(), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    my_games_results.set(get_my_games().await.ok());
                    my_games_error.set(get_my_games().await.err());
                });
                || ()
            }
        );
    }

    let all_games = match results.as_ref() {
        Some(results) => results
            .results
            .iter()
            .map(|game| {
                let game = game.clone();
                let my_games_results = my_games_results.clone();
                let added_game = added_game.clone();

                let mut my_game_status = false;
                
                let mygame = match my_games_results.as_ref() {
                    Some(mygames) => mygames.games.iter().find(|e| e.name == game.name),
                    None => None
                };

                {
                    match mygame.as_ref() {
                        Some(_) => 
                            my_game_status = true
                        ,
                        None => my_game_status = false
                    };
                }
                
                html!{
                    <tr key={game.id.clone()}>
                        <td>{game.name.clone()}</td>
                        <td>{&game.released.clone()[..4]}</td>
                        <td class="d-flex justify-content-center">
                            if my_game_status {
                            <button 
                                onclick={Callback::from(move |_| {
                                    let game = game.clone();
                                    wasm_bindgen_futures::spawn_local(async move {
                                        add_game(&game).await.ok();
                                    })
                                })}
                                class="btn btn-danger">{"Delete"}
                            </button>
                            }
                            else {
                                <button 
                                    onclick={Callback::from(move |_| {
                                        let game = game.clone();
                                        let my_games_results = my_games_results.clone();
                                        let added_game = added_game.clone();
                                        
                                        wasm_bindgen_futures::spawn_local(async move {
                                            match add_game(&game).await {
                                                Ok(game) => {
                                                    added_game.set({MyGame {_id: game._id.to_string(), name: game.name, released: game.released}});
                                                    let newlist = (*my_games_results).clone();
                                                    newlist.clone().unwrap().games.push((*added_game).clone());
                                                    my_games_results.set(newlist.clone());
                                                },
                                                Err(e) => {
                                                    let js_error = JsValue::from_str(&e.to_string());
                                                    console::log_1(&js_error);
                                                },
                                            }
                                        });
                                    })}
                                    class="btn btn-primary">{"Add"}
                                </button>
                            }
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
            <h2 class="p-3 d-flex justify-content-center">{"All Games"}</h2>
            <div class="d-flex justify-content-center">
                if results.as_ref() != None && my_games_results.as_ref() != None {
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
                                {all_games}
                            </tbody>    
                        </table>
                        <Pagination current={*current_page} on_click={on_set_page}/>
                    </div>
                }
                else {}
            </div>
        </>    
    }
}



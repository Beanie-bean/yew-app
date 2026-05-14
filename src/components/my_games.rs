use web_sys::console;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;
use gloo_net::Error;
use web_sys::ScrollToOptions;
use web_sys::ScrollBehavior;

use crate::utils::{delete_game, get_my_games, edit_list, add_game};
use crate::models::{MyGame, MyList, UpdateList, GameToAdd};
use crate::components::{pagination::*, edit_list_modal::*, add_game_modal::*};

#[component]
pub fn MyGames() -> Html {
    let selected_game = use_state(|| MyGame {_id: "".to_string(), name: "".into(), released: "".into()});
    let current_page = use_state(|| 1);
    let edit_modal_shown = use_state(|| false);
    let add_modal_shown = use_state(|| false);

    let results: UseStateHandle<Option<MyList>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);
    let total_pages = use_state(|| 0);


    let toggle_edit_list_modal = {
        let edit_modal_shown = edit_modal_shown.clone();
        Callback::from(move |_| edit_modal_shown.set(!*edit_modal_shown))
    };

    let toggle_add_game_modal = {
        let add_modal_shown = add_modal_shown.clone();
        Callback::from(move |_| add_modal_shown.set(!*add_modal_shown))
    };

    let submit_list = {
        let edit_modal_shown = edit_modal_shown.clone();
        let results = results.clone();
        Callback::from(move |list: UpdateList| {
            let results = results.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match edit_list(&list).await {
                    Ok(newlist) => {
                        results.set(Some(MyList { _id: results.as_ref().unwrap()._id.clone(), name: newlist.name, desc: newlist.desc, games: results.as_ref().unwrap().games.clone()}));
                    },
                    Err(e) => {
                        let js_error = JsValue::from_str(&e.to_string());
                        console::log_1(&js_error);
                    },
                }
            });
            edit_modal_shown.set(!*edit_modal_shown)
        })
    };

    let submit_game = {
        let add_modal_shown = add_modal_shown.clone();
        let results = results.clone();
        let selected_game = selected_game.clone();

        Callback::from(move |game_to_add: GameToAdd| {
            let results = results.clone();
            let selected_game = selected_game.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match add_game(&game_to_add).await {
                    Ok(game) => {
                        selected_game.set(MyGame {_id: game._id.to_string(), name: game.name, released: game.released});
                        let newlist = (*results).clone();
                        newlist.clone().unwrap().games.push((*selected_game).clone());
                        results.set(newlist.clone());
                    },
                    Err(e) => {
                        let js_error = JsValue::from_str(&e.to_string());
                        console::log_1(&js_error);
                    },
                }
            });
            add_modal_shown.set(!*add_modal_shown)
        })
    };

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

    {
        let results = results.clone();
        let error = error.clone();
        let total_pages = total_pages.clone();
        
        use_effect_with(selected_game.clone(), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    results.set(get_my_games().await.ok());
                    error.set(get_my_games().await.err());
                    total_pages.set(((results.as_ref().unwrap().games.len() as f64) / 20.0).ceil() as usize);
                });
                || ()
            }
        );
    }

    let my_games = match results.as_ref() {
        
        Some(mylist) => mylist
            .games.get((((*current_page).clone() - 1) * 20) as usize..(((*current_page).clone() - 1) * 20 + 20) as usize).unwrap_or(mylist.games.chunks_exact(20).remainder())
            .iter()
            .map(|game| {
                let game = game.clone();
                let results = results.clone();
                let selected_game = selected_game.clone();

                html!{
                    <tr key={game._id.clone().to_string()}>
                        <td>{game.name.clone()}</td>
                        <td>{&game.released.clone()}</td>
                        <td class="d-flex justify-content-center">
                            <button 
                                onclick={Callback::from(move |_| {
                                    let game = game.clone();
                                    let results = results.clone();
                                    let selected_game = selected_game.clone();
                                    selected_game.set(MyGame {_id: "".to_string(), name: "".into(), released: "".into()});

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
            <div class="row p-3 pb-2 g-0">
                <div class="col-4 text-start">
                    <button onclick={toggle_add_game_modal.clone()} type="button" class="btn btn-primary">
                        {"Add Game"}
                    </button>
                    if results.as_ref() != None {
                        <AddGameModal show={*add_modal_shown} hide={toggle_add_game_modal.clone()} save={submit_game}/>
                    }
                </div>
                <div class="col-4 text-center">
                    <div>
                        <h2>{"My Games"}</h2>
                    </div>
                </div>
                <div class="col-4 text-end">
                    <button onclick={toggle_edit_list_modal.clone()} type="button" class="btn btn-outline-primary">
                        {"Edit"}<i class="ps-1 bi bi-pencil-square"></i>
                    </button>
                    if results.as_ref() != None {
                        <EditListModal list={UpdateList { name: results.as_ref().unwrap().name.clone(), desc: results.as_ref().unwrap().desc.clone(),}} show={*edit_modal_shown} hide={toggle_edit_list_modal.clone()} save={submit_list}/>
                    }
                </div>
            </div>
            if results.as_ref() != None {
                <h4 class="d-flex justify-content-center">{&(*results.as_ref().unwrap().name)}</h4>
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
                        if results.as_ref().unwrap().games.len() > 20 {
                            <MyGamesPagination total_pages={*total_pages} current={*current_page} on_click={on_set_page}/>
                        }
                    </div>
                }
                </div>
            }
        </>    
    }
}



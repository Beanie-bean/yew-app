use gloo_net::Error;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;
use dotenv_codegen::dotenv;
use web_sys::ScrollToOptions;
use web_sys::ScrollBehavior;
use web_sys::console;

use crate::utils::{get_all_games, get_games_by_text, get_my_games, add_game, delete_game};
use crate::models::{MyGame, MyList, Results, GameToAdd};
use crate::components::pagination::*;

#[component]
pub fn AllGames() -> Html {
    let key = dotenv!("RAWGIO_API_KEY");
    let current_page = use_state(|| 1);
    let selected_game = use_state(|| MyGame {_id: "".to_string(), name: "".into(), released: "".into()});
    let is_loading = use_state(|| true);
    let search_word= use_state(|| String::new());
    let input_node = use_node_ref();

    let on_set_page = {
        let current_page = current_page.clone();
        let is_loading = is_loading.clone();
        Callback::from(move |page: u32| {
            let options = ScrollToOptions::new();
            options.set_top(0.0);
            options.set_behavior(ScrollBehavior::Smooth);
            web_sys::window().unwrap().scroll_to_with_scroll_to_options(&options);
            current_page.set(page);
            is_loading.set(true);
        })
    };
    
    let on_search_submit = {
        let input_node = input_node.clone();
        let search_word = search_word.clone();
        let is_loading = is_loading.clone();
        let current_page = current_page.clone();
        Callback::from(move |_| {
            if let Some(input) = input_node.cast::<HtmlInputElement>() {
                search_word.set(input.value());
                current_page.set(1);
                is_loading.set(true);
            }
        }) 
    };

    let on_search_clear = {
        let input_node = input_node.clone();
        let search_word = search_word.clone();
        let is_loading = is_loading.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = input_node.cast::<HtmlInputElement>() {
                input.set_value("");
                search_word.set(String::new());
                is_loading.set(true);
            }
        })
    };

    let results: UseStateHandle<Option<Results>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);


    {
        let results = results.clone();
        let error = error.clone();
        let page = current_page.clone();
        let is_loading = is_loading.clone();
        let search_word = search_word.clone();
        
        use_effect_with((page.clone(), search_word.clone()), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if search_word.clone().is_empty() {
                        results.set(get_all_games(key, *page).await.ok());
                        error.set(get_all_games(key, *page).await.err());
                        is_loading.set(false);
                    }
                    else {
                        results.set(get_games_by_text(key, *page, (*search_word).clone()).await.ok());
                        error.set(get_games_by_text(key, *page, (*search_word).clone()).await.err());
                        is_loading.set(false);
                    }
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
        
        use_effect_with(selected_game.clone(), move |_| {
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
                let selected_game = selected_game.clone();

                let mut my_game_status = false;
                
                let mygame = my_games_results.as_ref().and_then(|mygames| {
                    mygames.games.iter()
                        .find(|e| e.name == game.name && e.released == game.released[..4])
                        .cloned()
                });

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
                                        let my_games_results = my_games_results.clone();
                                        let selected_game = selected_game.clone();
                                        let mygame = mygame.clone();
                                        selected_game.set(MyGame {_id: "".to_string(), name: "".into(), released: "".into()});

                                        wasm_bindgen_futures::spawn_local(async move {
                                            match delete_game(&mygame.unwrap()).await {
                                                Ok(game) => {
                                                    selected_game.set(MyGame {_id: game._id.to_string(), name: game.name.clone(), released: game.released.clone()});
                                                    let newlist = (*my_games_results).clone();
                                                    if let Some(i) = newlist.clone().unwrap().games.iter().position(|e| e.name == game.name && e.released == game.released) {
                                                        newlist.clone().unwrap().games.remove((i).clone());
                                                    }
                                                    my_games_results.set(newlist.clone());
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
                            }
                            else {
                                <button 
                                    onclick={Callback::from(move |_| {
                                        let game = game.clone();
                                        let game_to_add = GameToAdd { name: game.name, released: game.released[..4].into() };
                                        let my_games_results = my_games_results.clone();
                                        let selected_game = selected_game.clone();
                                        selected_game.set(MyGame {_id: "".to_string(), name: "".into(), released: "".into()});

                                        wasm_bindgen_futures::spawn_local(async move {
                                            match add_game(&game_to_add).await {
                                                Ok(game) => {
                                                    selected_game.set(MyGame {_id: game._id.to_string(), name: game.name, released: game.released});
                                                    let newlist = (*my_games_results).clone();
                                                    newlist.clone().unwrap().games.push((*selected_game).clone());
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
        <main>
            <div style="minHeight: 100vh, overflow-y: auto;">  
                <h2 class="p-3 d-flex justify-content-center">{"All Games"}</h2>
                <div class="d-flex justify-content-center">
                    if (*is_loading) == false && results.as_ref() != None && my_games_results.as_ref() != None {
                        <div style="min-width: 50%">
                            <div class="input-group mb-3">
                                <input type="text" class="form-control" aria-label="Search field" ref={input_node} value={(*search_word).clone()} />
                                <button onclick={on_search_clear} class="input-group-text" aria-label="Clear search"><i class="bi bi-x-lg"></i></button>
                                <button onclick={on_search_submit} class="input-group-text" aria-label="Submit search">{"Search"}</button>
                            </div>
                            <table class="table table-striped align-middle table-bordered">
                                <thead>
                                    <tr>
                                        <th>{"Name"}</th>
                                        <th width="20%">{"Release Year"}</th>
                                        <th width="14%"></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {all_games}
                                </tbody>    
                            </table>
                            <Pagination current={*current_page} on_click={on_set_page} check_next_page={&results.as_ref().unwrap().next}/>
                        </div>
                    }
                    else {
                        <div class="spinner-border text-primary" role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                    }
                </div>
            </div>    
        </main>
    }
}



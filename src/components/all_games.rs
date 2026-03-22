use gloo_net::{http::Request, Error};
use yew::prelude::*;
use serde::Deserialize;
use dotenv_codegen::dotenv;
use crate::components::pagination::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Game {
    id: usize,
    name: AttrValue,
    released: AttrValue
}

#[derive(Clone, PartialEq, Deserialize)]
struct Results {
    results: Vec<Game>
}

#[component]
pub fn AllGames() -> Html {
    let key = dotenv!("RAWGIO_API_KEY");
    let current_page = use_state(|| 1);
    let query = use_state(||format!("https://rawg.io/api/games?key={}&page={}", key, *current_page));

    let on_set_page = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
        })
    };
    
    let results: UseStateHandle<Option<Results>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

   {
        let results = results.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_games = Request::get(query.as_str())
                        .send()
                        .await;
                    match fetched_games {
                        Ok(response) => {
                            let json = response.json::<Results>().await;
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

    let all_games = match results.as_ref() {
        Some(results) => results
            .results
            .iter()
            .map(|game| {
                html!{
                    <tr key={game.id.clone()}>
                        <td>{game.name.clone()}</td>
                        <td>{&game.released.clone()[..4]}</td>
                        <td class="d-flex justify-content-center"><button class="btn btn-primary">{"Add"}</button></td>
                    </tr>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(_) => {
                html! {
                    <p>{"error"}</p>
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
                if true {
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

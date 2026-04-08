use web_sys::HtmlInputElement;
use yew::{prelude::*};

use crate::models::{GameToAdd};

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    #[prop_or_default]
    pub show: bool,
    pub hide: Callback<MouseEvent>,
    pub save: Callback<GameToAdd>
}
// #[hook]
// pub fn use_modal(props: &ModalProps) -> (bool, Callback<MouseEvent>, Callback<GameToAdd>) {
//     let ModalProps { show, hide, save } = props.clone();
//     let mygame = use_state(|| game);
    
//     let set_game = {
//         let mygame = mygame.clone();
//         Callback::from(move |game_to_add: GameToAdd| {
//             mygame.set(game_to_add.clone());
//             save.emit(game_to_add);
        
//         })
//     };
//     (show, hide, set_game)
// }

#[component]
pub fn AddGameModal(props: &ModalProps) -> Html { 
    let game_state = use_state(|| GameToAdd { name: "".into(), released: "".into() });

    let set_game = {
         let mygame = game_state.clone();
         let props = props.clone();
         Callback::from(move |game_to_add: GameToAdd| {
             mygame.set(game_to_add.clone());
             props.save.emit(game_to_add);
         })
     };

    let on_name_change = {
        let game_state = game_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_game = (*game_state).clone();
            new_game.name = input.value().into();
            game_state.set(new_game);
        }) 
    };

    let on_released_change = {
        let game_state = game_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_game = (*game_state).clone();
            new_game.released = input.value().into();
            game_state.set(new_game);
        }) 
    };

    html! {
        <>
            <div class={classes!(
                if props.show {"modal fade show"} else {"modal fade"}
            )} style={format!("display: {}", if props.show {"block"} else {"none"})} tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title fs-5">{"Add Game"}</h1>
                            <button type="button" class="btn-close" onclick={props.hide.clone()}></button>
                        </div>
                        <div class="modal-body">
                            <div class="mb-3">
                                <label class="form-label mt-1">{"Name"}</label>
                                <input oninput={on_name_change} value={(*game_state).clone().name} class="form-control" />
                                <label class="form-label mt-1">{"Release Year"}</label>
                                <input oninput={on_released_change} value={(*game_state).clone().released} class="form-control" />
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" onclick={props.hide.clone()}>{"Close"}</button>
                            <button type="button" 
                                onclick={{
                                    let set_game = set_game.clone();
                                    Callback::from(move |_| {
                                        set_game.emit((*game_state).clone());
                                        game_state.set(GameToAdd { name: "".into(), released: "".into() });
                                    })
                                }}
                            class="btn btn-primary">{"Save Game"}</button>
                        </div>
                    </div>
                </div>
            </div>
            if props.show {
                <div class="modal-backdrop fade show"></div>
            }
        </>
    }
}
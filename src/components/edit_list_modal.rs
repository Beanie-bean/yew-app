use web_sys::HtmlInputElement;
use yew::{prelude::*};

use crate::{models::{UpdateList}, utils::edit_list};

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    #[prop_or_default]
    pub show: bool,
    pub hide: Callback<MouseEvent>,
    pub list: UpdateList,
    pub save: Callback<UpdateList>
}
#[hook]
pub fn use_modal(props: &ModalProps) -> (bool, Callback<MouseEvent>, UpdateList, Callback<UpdateList>) {
    let ModalProps { show, hide, list, save } = props.clone();
    let mylist = use_state(|| list);
    
    let set_list = {
        let mylist = mylist.clone();
        Callback::from(move |update_list: UpdateList| {
            mylist.set(update_list.clone());
            save.emit(update_list);
        
        })
    };
    (show, hide, (*mylist).clone(), set_list)
}

#[component]
pub fn EditListModal(props: &ModalProps) -> Html {
    let (show, hide, list, set_list) = use_modal(&ModalProps { 
        show: props.show, 
        hide: props.hide.clone(), 
        list: props.list.clone(), 
        save: props.save.clone() 
    });
    
    let list_state = use_state(|| list);

    let on_name_change = {
        let list_state = list_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_list = (*list_state).clone();
            new_list.name = input.value();
            list_state.set(new_list);
        }) 
    };

    let on_desc_change = {
        let list_state = list_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_list = (*list_state).clone();
            new_list.desc = input.value();
            list_state.set(new_list);
        }) 
    };

    html! {
        <>
            <div class={classes!(
                if show {"modal fade show"} else {"modal fade"}
            )} style={format!("display: {}", if show {"block"} else {"none"})} tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title fs-5">{"Edit List"}</h1>
                            <button type="button" class="btn-close" onclick={hide.clone()}></button>
                        </div>
                        <div class="modal-body">
                            <div class="mb-3">
                                <label class="form-label mt-1">{"Name"}</label>
                                <input oninput={on_name_change} value={(*list_state).clone().name} class="form-control" />
                                <label class="form-label mt-1">{"Description"}</label>
                                <input oninput={on_desc_change} value={(*list_state).clone().desc} class="form-control" />
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" onclick={hide.clone()}>{"Close"}</button>
                            <button type="button" 
                                onclick={{
                                    let set_list = set_list.clone();
                                    Callback::from(move |_| {
                                        set_list.emit((*list_state).clone())
                                    })
                                }}
                            class="btn btn-primary">{"Save List"}</button>
                        </div>
                    </div>
                </div>
            </div>
            if show {
                <div class="modal-backdrop fade show"></div>
            }
        </>
    }
}
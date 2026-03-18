use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::all_games::AllGames;
use crate::components::my_lists::MyLists;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    AllGames,
    #[at("/mylists")]
    MyLists
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::AllGames => html! { <AllGames /> },
        Route::MyLists => html! { <MyLists /> },
    }
}

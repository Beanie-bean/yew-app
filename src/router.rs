use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::all_games::AllGames;
use crate::components::my_games::MyGames;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    AllGames,
    #[at("/mygames")]
    MyGames
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::AllGames => html! { <AllGames /> },
        Route::MyGames => html! { <MyGames /> },
    }
}

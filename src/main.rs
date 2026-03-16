use crate::components::all_games::AllGames;
use yew_router::prelude::*;
use yew::prelude::*;

pub mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    AllGames,
    #[at("/mylists")]
    MyLists
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::AllGames => html! { <AllGames /> },
        Route::MyLists => html! { <h1>{ "My lists" }</h1> },
    }
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
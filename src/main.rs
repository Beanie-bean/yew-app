use yew_router::prelude::*;
use yew::prelude::*;
use crate::router::{Route, switch};
use crate::components::navbar::Navbar;

mod components;
mod router;
mod utils;
mod models;

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Navbar /> 
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
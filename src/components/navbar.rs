use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-sm bg-body-tertiary">
                <div class="container-fluid">
                    <Link<Route> classes={classes!("navbar-brand")} to={Route::AllGames}>{ "Yew Games" }</Link<Route>>
                    <div class="navbar-collapse">
                        <ul class="navbar-nav">
                            <li class="nav-item">
                                <Link<Route> classes={classes!("nav-link")} to={Route::AllGames}>{ "All Games" }</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> classes={classes!("nav-link")}to={Route::MyGames}>{ "My Games" }</Link<Route>>                    
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        }
    }

}
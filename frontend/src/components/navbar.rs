use yew::prelude::*;
use yew_router::prelude::*;

use crate::{utils::class_if, Route};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let active_route = use_route::<Route>().unwrap_or_default();

    html! {
        <header class="navbar">
            <h1 class="site-title">
                <a href="/">{"Frigu"}</a>
            </h1>
            <div class="nav-links">
                <Link<Route> to={Route::ProductPage} classes={classes!(class_if(active_route == Route::ProductPage, "active"))}>{"Products"}</Link<Route>>
                <Link<Route> to={Route::PurchasesPage} classes={classes!(class_if(active_route == Route::PurchasesPage, "active"))}>{"Purchases"}</Link<Route>>
            </div>
        </header>
    }
}

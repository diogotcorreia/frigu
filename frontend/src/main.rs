use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

mod api;
mod utils;

use pages::product_page::ProductPage;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/products")]
    ProductPage,
}

fn switch(routes: &Route) -> Html {
    match routes {
        /* TODO redirect to login page if logged out */
        Route::Home => html! { <Redirect<Route> to={Route::ProductPage} /> },
        Route::ProductPage => html! { <ProductPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}

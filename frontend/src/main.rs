use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod hooks;
mod pages;

mod api;
mod utils;

use pages::{
    login_page::LoginPage, product_insert_page::ProductInsertPage, product_page::ProductPage,
    purchases_page::PurchasesPage,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/products")]
    ProductPage,
    #[at("/product/insert")]
    ProductInsertPage,
    #[at("/purchases")]
    PurchasesPage,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::ProductPage} /> },
        Route::Login => html! { <LoginPage /> },
        Route::ProductPage => html! { <ProductPage /> },
        Route::ProductInsertPage => html! { <ProductInsertPage /> },
        Route::PurchasesPage => html! { <PurchasesPage /> },
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

use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class="navbar">
            <h1 class="site-title">{"Frigu"}</h1>
            <div class="nav-links">
                <a href="/products" class="active">{"Products"}</a>
                <a href="/my-transactions">{"Transactions"}</a>
            </div>
        </header>
    }
}

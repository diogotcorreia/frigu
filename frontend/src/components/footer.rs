use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{api, hooks::auth::use_auth, Route};

#[function_component(Footer)]
pub fn footer() -> Html {
    let history = use_history().unwrap();
    // The use_auth hook requires the user to be logged in
    let user = use_auth();

    let handle_logout = {
        Callback::from(move |_| {
            let history = history.clone();
            spawn_local(async move {
                match api::logout().await {
                    Ok(_) => history.push(Route::Home),
                    Err(_) => { /* TODO */ }
                }
            });
        })
    };

    html! {
        <footer class="footer">
            <div class="user">
                {
                    if let Some(user) = user {
                        html! {
                            <span>{format!("Logged in as {}. ", user.name)}</span>
                        }
                    } else {
                        html! {
                            <span>{"Loading... "}</span>
                        }
                    }
                }
                <a href="#" onclick={handle_logout}>{"Logout"}</a>
            </div>
        </footer>
    }
}

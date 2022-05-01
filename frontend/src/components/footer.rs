use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{api, hooks::auth::use_auth, Route};

#[function_component(Footer)]
pub fn footer() -> Html {
    let history = use_history().expect("yew-router must be accessible");
    // The use_auth hook requires the user to be logged in
    let user = use_auth();
    let logout_action = use_async(async move { api::logout().await });

    if logout_action.data.is_some() {
        history.push(Route::Home);
    }

    let handle_logout = {
        let logout_action = logout_action.clone();
        Callback::from(move |_| {
            logout_action.run();
        })
    };

    html! {
        <footer class="footer">
            <div class="user">
                {
                    match user {
                        Some(user) => html! {
                            <span>{format!("Logged in as {}. ", user.name)}</span>
                        },
                        None => html! { <span>{"Loading... "}</span> }
                    }
                }
                <a href="#" onclick={handle_logout}>{"Logout"}</a>
            </div>
            {
                logout_action.error.as_ref().map_or_else(|| html!{}, |error| html! {
                    <div>{format!("Failed to logout: {}", error)}</div>
                })
            }
        </footer>
    }
}

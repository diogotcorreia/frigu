use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{api, Route};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let history = use_history().unwrap();
    let phone_ref = use_node_ref();
    let password_ref = use_node_ref();

    let handle_submit = {
        let phone_ref = phone_ref.clone();
        let password_ref = password_ref.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();

            let history = history.clone();
            let phone_ref = phone_ref.clone();
            let password_ref = password_ref.clone();
            spawn_local(async move {
                match api::login(&api::LoginPayload {
                    phone: phone_ref.cast::<HtmlInputElement>().unwrap().value(),
                    password: password_ref.cast::<HtmlInputElement>().unwrap().value(),
                })
                .await
                {
                    Ok(()) => {
                        history.push(Route::Home);
                    }
                    Err(_) => (/* TODO handle errors */),
                };
            })
        })
    };

    html! {
        <main>
            <div class="card login-card">
                <div class="card-header">
                    {"Login to Frigu"}
                </div>
                <div class="card-content">
                    <form class="form form-vertical form-margin-top" onsubmit={handle_submit}>
                        <label for="login--phone">{"Phone Number"}</label>
                        <input ref={phone_ref} type="text" id="login--phone" />

                        <label for="login--password">{"Password"}</label>
                        <input ref={password_ref} type="password" id="login--password" />

                        <button type="submit" class="btn btn--full-width">{"Login"}</button>
                    </form>
                </div>
            </div>
        </main>
    }
}

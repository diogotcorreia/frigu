use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{
    api::{self, ApiError},
    utils::class_if,
    Route,
};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let history = use_history().expect("yew-router must be accessible");
    let phone_ref = use_node_ref();
    let password_ref = use_node_ref();
    let state = {
        let phone_ref = phone_ref.clone();
        let password_ref = password_ref.clone();
        use_async(async move {
            let payload = &api::LoginPayload {
                phone: phone_ref.cast::<HtmlInputElement>().unwrap().value(),
                password: password_ref.cast::<HtmlInputElement>().unwrap().value(),
            };

            api::login(payload).await
        })
    };

    if state.data.is_some() {
        history.push(Route::Home);
    }

    let handle_submit = {
        let state = state.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default(); // avoid form submission
            state.run();
        })
    };

    let error = state.error.as_ref().map(|error| match error {
        ApiError::HttpUnauthorized(_) => "Invalid phone number or password".to_string(),
        error => format!("{}", error),
    });

    html! {
        <main>
            <div class={classes!("card", "login-card", class_if(state.loading, "card-loading"))}>
                <div class="loading-bar" />
                {
                    if let Some(error) = error {
                        html! { <div class="card-error">{error}</div> }
                    } else {
                        html! {}
                    }
                }
                <div class="card-header">
                    {"Login to Frigu"}
                </div>
                <div class="card-content">
                    <form class="form form-vertical form-margin-top" onsubmit={handle_submit}>
                        <label for="login--phone">{"Phone Number"}</label>
                        <input ref={phone_ref} type="text" id="login--phone" />

                        <label for="login--password">{"Password"}</label>
                        <input ref={password_ref} type="password" id="login--password" />

                        <button type="submit" class="btn btn--full-width btn--primary">{"Login"}</button>
                    </form>
                </div>
            </div>
        </main>
    }
}

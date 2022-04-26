use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::{self, User},
    Route,
};

pub fn use_auth() -> Option<User> {
    let history = use_history().unwrap();
    let user = use_state(|| None);

    {
        let user = user.clone();
        use_effect(move || {
            spawn_local(async move {
                if user.is_none() {
                    match api::user_info().await {
                        Ok(user_response) => user.set(Some(user_response)),
                        Err(_) => history.push(Route::Login),
                    }
                }
            });
            || {}
        });
    }

    (*user).clone()
}

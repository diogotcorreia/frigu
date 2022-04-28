use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api,
    components::{footer::Footer, navbar::Navbar},
    Route,
};

#[function_component(ProductInsertPage)]
pub fn product_insert_page() -> Html {
    let history = use_history().unwrap();
    let name_ref = use_node_ref();
    let description_ref = use_node_ref();
    let stock_ref = use_node_ref();
    let price_ref = use_node_ref();

    let handle_submit = {
        let name_ref = name_ref.clone();
        let description_ref = description_ref.clone();
        let stock_ref = stock_ref.clone();
        let price_ref = price_ref.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();

            let history = history.clone();

            let product_payload = api::ProductPayload {
                id: None,
                name: name_ref.cast::<HtmlInputElement>().unwrap().value(),
                description: Some(description_ref.cast::<HtmlInputElement>().unwrap().value()),
                stock: stock_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value_as_number() as u32,
                price: price_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value_as_number() as u32,
            };

            spawn_local(async move {
                match api::insert_product(&product_payload).await {
                    Ok(_) => history.push(Route::ProductPage),
                    Err(_) => { /* TODO handle errors */ }
                }
            });
        })
    };

    html! {
        <>
            <Navbar />
            <main>
                <div class="card products-card">
                    <div class="card-header">
                        {"Create Product"}
                    </div>
                    <div class="card-content">
                        <form class="form form-vertical form-margin-top" onsubmit={handle_submit}>
                            <label for="product--name">{"Name (*)"}</label>
                            <input ref={name_ref} type="text" id="product--name" required={true} />

                            <label for="product--description">{"Description"}</label>
                            <input ref={description_ref} type="text" id="product--description" />

                            <label for="product--stock">{"Stock (*)"}</label>
                            <input ref={stock_ref} type="number" min={0} id="product--stock" />

                            <label for="product--price">{"Price (cents) (*)"}</label>
                            <input ref={price_ref} type="number" min={0} id="product--price" />

                            <button type="submit" class="btn btn--full-width">{"Create"}</button>
                        </form>
                    </div>
                </div>
            </main>
            <Footer />
        </>
    }
}

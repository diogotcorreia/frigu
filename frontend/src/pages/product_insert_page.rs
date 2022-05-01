use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{
    api,
    components::{footer::Footer, navbar::Navbar},
    utils::class_if,
    Route,
};

#[function_component(ProductInsertPage)]
pub fn product_insert_page() -> Html {
    let history = use_history().expect("yew-router must be accessible");
    let name_ref = use_node_ref();
    let description_ref = use_node_ref();
    let stock_ref = use_node_ref();
    let price_ref = use_node_ref();

    let state = {
        let name_ref = name_ref.clone();
        let description_ref = description_ref.clone();
        let stock_ref = stock_ref.clone();
        let price_ref = price_ref.clone();

        use_async(async move {
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

            api::insert_product(&product_payload).await
        })
    };

    let handle_submit = {
        let state = state.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default(); // prevent form submission
            state.run();
        })
    };

    if state.data.is_some() {
        history.push(Route::ProductPage);
    }

    html! {
        <>
            <Navbar />
            <main>
                <div class={classes!("card", "products-card", class_if(state.loading, "card-loading"))}>
                    <div class="loading-bar" />
                    {
                        state.error.as_ref().map_or_else(|| html!{}, |error| html! {
                            <div class="card-error">{error}</div>
                        })
                    }
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

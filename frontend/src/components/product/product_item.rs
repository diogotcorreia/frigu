use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{api, utils};
use crate::{
    api::ApiError,
    components::{
        dialog::Dialog,
        product::{
            product_purchase_complete_dialog::ProductPurchaseCompleteDialog,
            product_purchase_dialog::ProductPurchaseDialog,
        },
    },
};

#[derive(Clone, Properties, PartialEq)]
pub struct ProductItemProps {
    pub product: api::Product,
    pub on_update: Callback<()>,
}

#[function_component(ProductItem)]
pub fn product_item(props: &ProductItemProps) -> Html {
    let flow_state = use_state(|| PurchaseFlow::None);

    let buy_click_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_| flow_state.set(PurchaseFlow::SelectingQuantity(false)))
    };

    let dialog_close_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_| flow_state.set(PurchaseFlow::None))
    };

    let dialog_buy_handler = {
        let flow_state = flow_state.clone();
        let on_update = props.on_update.clone();
        let product_id = props.product.id;
        Callback::from(move |quantity: u32| {
            let flow_state = flow_state.clone();
            let on_update = on_update.clone();
            flow_state.set(PurchaseFlow::SelectingQuantity(true));
            spawn_local(async move {
                let payload = api::PurchaseProductPayload { quantity };
                match api::purchase_product(product_id, &payload).await {
                    Ok(_) => {
                        flow_state.set(PurchaseFlow::Complete);
                        on_update.emit(());
                    }
                    Err(error) => flow_state.set(PurchaseFlow::Error(error)),
                };
            })
        })
    };

    let product = &props.product;

    html! {
        <div class="product-item">
            <div class="product-info">
                <div class="product-info--name">{product.name.clone()}</div>
                <div class="product-info--metadata">
                    {"By "}
                    <span class="product-info--seller">{product.seller_name.clone()}</span>
                    {" | "}
                    <span class="product-info--stock">{product.stock}</span>
                    {" in stock"}
                </div>
                <div class="product-info--description">{product.description.as_ref().unwrap_or(&String::new())}</div>
            </div>
            <div class="product-price">
                {utils::format_display_price(product.price)}
            </div>
            <div class="product-actions">
                <button onclick={buy_click_handler} class="btn product-actions--purchase">{"Buy"}</button>
            </div>
            {
                match &*flow_state {
                    PurchaseFlow::SelectingQuantity(loading) => html! {
                        <ProductPurchaseDialog
                            loading={*loading}
                            product={product.clone()}
                            on_close={dialog_close_handler}
                            on_buy={dialog_buy_handler}
                        />
                    },
                    PurchaseFlow::Error(error) => html! {
                        <Dialog>
                            <div class="card">
                                <div class="card-header">
                                    {"Error"}
                                </div>
                                <div class="card-error">
                                    {error}
                                </div>
                                <div class="card-actions product-actions">
                                    <button onclick={dialog_close_handler} class="btn product-actions--cancel">
                                        {"Close"}
                                    </button>
                                </div>
                            </div>
                        </Dialog>
                    },
                    PurchaseFlow::Complete => html! {
                        <ProductPurchaseCompleteDialog on_close={dialog_close_handler} />
                    },
                    PurchaseFlow::None => html! {},
                }
            }
        </div>
    }
}

pub enum PurchaseFlow {
    None,
    SelectingQuantity(bool),
    Error(ApiError),
    Complete,
}

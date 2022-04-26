use yew::prelude::*;

use crate::components::{
    dialog::Dialog,
    product::{
        product_purchase_complete_dialog::ProductPurchaseCompleteDialog,
        product_purchase_dialog::ProductPurchaseDialog,
    },
};
use crate::{api, utils};

#[derive(Clone, Properties, PartialEq)]
pub struct ProductItemProps {
    pub product: api::Product,
}

#[function_component(ProductItem)]
pub fn product_item(props: &ProductItemProps) -> Html {
    let flow_state = use_state(|| PurchaseFlow::None);

    let buy_click_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_| flow_state.set(PurchaseFlow::SelectingQuantity))
    };

    let dialog_close_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_| flow_state.set(PurchaseFlow::None))
    };

    let dialog_buy_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_amount: u32| {
            flow_state.set(PurchaseFlow::Loading);
            // TODO send request to backend
            flow_state.set(PurchaseFlow::Complete);
        })
    };

    let product = &props.product;

    html! {
        <div class="product-item">
            <div class="product-info">
                <div class="product-info--name">{product.name.clone()}</div>
                <div class="product-info--metadata">
                    {"By "}
                    <span class="product-info--seller">{"Rafael Girão" /* TODO */}</span>
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
                match *flow_state {
                    PurchaseFlow::SelectingQuantity => html! {
                        <ProductPurchaseDialog product={product.clone()} on_close={dialog_close_handler} on_buy={dialog_buy_handler} />
                    },
                    PurchaseFlow::Loading => html! {
                        <Dialog>
                            <div class="card">
                                <div class="card-header">
                                    {"Loading..."}
                                </div>
                            </div>
                        </Dialog>
                    },
                    PurchaseFlow::Complete => html! {
                        <ProductPurchaseCompleteDialog on_close={dialog_close_handler} />
                    },
                    _ => html! {},
                }
            }
        </div>
    }
}

pub enum PurchaseFlow {
    None,
    SelectingQuantity,
    Loading,
    Complete,
}

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api,
    utils::{self, class_if},
};

#[derive(Clone, Properties, PartialEq)]
pub struct PurchaseItemProps {
    pub purchase: api::Purchase,
    pub is_seller: bool,
    pub on_update: Callback<()>,
}

#[function_component(PurchaseItem)]
pub fn purchase_item(props: &PurchaseItemProps) -> Html {
    let purchase = &props.purchase;

    let handle_mark_as_paid = {
        let on_update = props.on_update.clone();
        let purchase_id = props.purchase.id;
        Callback::from(move |_| {
            let on_update = on_update.clone();
            spawn_local(async move {
                match api::pay_purchase(purchase_id).await {
                    Ok(_) => {
                        on_update.emit(());
                    }
                    Err(_) => { /* TODO */ }
                };
            })
        })
    };

    html! {
        <div class="purchase-item">
            <div class="purchase-info">
                <div class="purchase-info--name">
                    {purchase.product.name.clone()}
                    <span class={classes!("purchase-info--paid-badge", class_if(purchase.paid_date.is_none(), "purchase-info--paid-badge__unpaid"))}>
                        { if purchase.paid_date.is_none() { "Not Paid" } else { "Paid" } }
                    </span>
                </div>
                <div class="purchase-info--metadata">
                    {
                        if props.is_seller {
                            html! {
                                <>
                                    <span class="purchase-info--quantity">{purchase.quantity}</span>
                                    {" sold"}
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    {"Purchased from "}
                                    <span class="purchase-info--seller">{purchase.product.seller_name.clone()}</span>
                                    {" | "}
                                    <span class="purchase-info--quantity">{purchase.quantity}</span>
                                    {" bought"}
                                </>
                            }
                        }
                    }
                </div>
                <div class="purchase-info--description">{purchase.product.description.as_ref().unwrap_or(&String::new())}</div>
            </div>
            <div class="purchase-price">
                {utils::format_display_price(purchase.unit_price * purchase.quantity)}
            </div>
            {
                if props.is_seller {
                    html! {
                        <div class="purchase-actions">
                            <button onclick={handle_mark_as_paid} class="btn purchase-actions--pay">{"Settle"}</button>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

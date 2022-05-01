use yew::prelude::*;
use yew_hooks::use_async;

use crate::{
    api,
    utils::{self, class_if, format_datetime},
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
    let settle = {
        let on_update = props.on_update.clone();
        let purchase_id = purchase.id;
        use_async(async move {
            let res = api::pay_purchase(purchase_id).await;
            if res.is_ok() {
                on_update.emit(());
            }
            res
        })
    };

    let handle_settle = {
        let settle = settle.clone();
        Callback::from(move |_| {
            settle.run();
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
                <div class="purchase-info--date">{format!("At {}", format_datetime(purchase.date.clone()))}</div>
                {
                    match purchase.paid_date {
                        Some(date) => html! {
                            <div class="purchase-info--paid-date">{format!("Paid at {}", format_datetime(date.clone()))}</div>
                        },
                        None => html! {}
                    }
                }
                <div class="purchase-info--description">{purchase.product.description.as_ref().unwrap_or(&String::new())}</div>
            </div>
            <div class="purchase-price">
                {utils::format_display_price(purchase.unit_price * purchase.quantity)}
            </div>
            {
                if props.is_seller {
                    html! {
                        <div class="purchase-actions">
                            <button onclick={handle_settle} disabled={settle.loading} class="btn purchase-actions--pay">{"Settle"}</button>
                            {
                                settle.error.as_ref().map_or_else(|| html!{}, |error| html! {
                                    <div class="purchase-actions--error">{error}</div>
                                })
                            }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

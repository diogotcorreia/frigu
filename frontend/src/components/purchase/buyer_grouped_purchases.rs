use yew::prelude::*;
use yew_hooks::use_async;

use crate::{api, components::purchase::purchase_item::PurchaseItem, utils};

#[derive(Clone, Properties, PartialEq)]
pub struct BuyerGroupedPurchasesProps {
    pub grouped_purchases: api::BuyerGroupedPurchases,
    pub on_update: Callback<()>,
}

#[function_component(BuyerGroupedPurchases)]
pub fn buyer_grouped_purchases(props: &BuyerGroupedPurchasesProps) -> Html {
    let grouped_purchases = &props.grouped_purchases;
    let settle = {
        let on_update = props.on_update.clone();
        let buyer_id = props.grouped_purchases.buyer.id;
        let purchase_count = props.grouped_purchases.purchases.len() as u32;
        use_async(async move {
            let res = api::pay_purchase_user_bulk(buyer_id, purchase_count).await;
            if res.is_ok() {
                on_update.emit(());
            }
            res
        })
    };

    let handle_mark_as_paid = {
        let settle = settle.clone();
        Callback::from(move |_| {
            settle.run();
        })
    };

    html! {
        <div class="grouped-buyer-item">
            <div class="buyer-info">
                <div class="buyer-info--name">
                    {grouped_purchases.buyer.name.clone()}
                    <span class="buyer-info--count-badge">
                        {grouped_purchases.purchases.len()}
                    </span>
                </div>
                <div class="buyer-info--amount-due">
                    {utils::format_display_price(grouped_purchases.amount_due)}
                </div>
                <div class="buyer-info--actions">
                    <button onclick={handle_mark_as_paid} disabled={settle.loading} class="btn buyer-info--actions__pay">{"Settle"}</button>
                    {
                        settle.error.as_ref().map_or_else(|| html!{}, |error| html! {
                            <div class="buyer-info--actions__error">{error}</div>
                        })
                    }
                </div>
            </div>
            <div class="buyer-purchases purchases-list">
                {
                    grouped_purchases.purchases.iter()
                        .map(|purchase| {
                            html! {
                                <PurchaseItem
                                    key={purchase.id}
                                    is_seller={true}
                                    purchase={purchase.clone()}
                                    on_update={&props.on_update}
                                />
                            }
                        })
                        .collect::<Html>()
                }
            </div>
        </div>
    }
}

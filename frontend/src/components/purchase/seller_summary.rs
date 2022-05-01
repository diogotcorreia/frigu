use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    api, components::purchase::buyer_grouped_purchases::BuyerGroupedPurchases, utils::class_if,
};

#[function_component(SellerSummary)]
pub fn seller_summary() -> Html {
    let buyers = use_async_with_options(
        async move { api::seller_summary().await },
        UseAsyncOptions::enable_auto(),
    );

    let refresh_purchases = {
        let buyers = buyers.clone();
        Callback::<()>::from(move |_| {
            buyers.run();
        })
    };

    html! {
        <div class={classes!("card", "purchases-card", class_if(buyers.loading, "card-loading"))}>
            <div class="loading-bar" />
            {
                buyers.error.as_ref().map_or_else(|| html!{}, |error| html! {
                    <div class="card-error">{error}</div>
                })
            }
            <div class="card-header">
                {"Products Sold"}
            </div>
            <div class="card-content">
                <div class="purchases-list">
                    {
                        buyers.data.as_ref().map_or_else(|| html!{}, |buyers| {
                            if buyers.is_empty() {
                                html! {
                                    <p>{"You don't have any purchases to settle"}</p>
                                }
                            } else {
                                buyers.iter()
                                    .map(|buyer| {
                                        html! {
                                            <BuyerGroupedPurchases
                                                key={buyer.buyer.id}
                                                grouped_purchases={buyer.clone()}
                                                on_update={&refresh_purchases}
                                            />
                                        }
                                    })
                                    .collect()
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}

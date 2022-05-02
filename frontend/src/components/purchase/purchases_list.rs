use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{api, components::purchase::purchase_item::PurchaseItem, utils::class_if};

#[function_component(PurchasesList)]
pub fn purchases_list() -> Html {
    let purchases = use_async_with_options(
        async move { api::list_purchases().await },
        UseAsyncOptions::enable_auto(),
    );

    let refresh_purchases = {
        let purchases = purchases.clone();
        Callback::<()>::from(move |_| {
            purchases.run();
        })
    };

    html! {
        <div class={classes!("card", "purchases-card", class_if(purchases.loading, "card-loading"))}>
            <div class="loading-bar" />
            {
                purchases.error.as_ref().map_or_else(|| html!{}, |error| html! {
                    <div class="card-error">{error}</div>
                })
            }
            <div class="card-header">
                {"Purchases"}
            </div>
            <div class="card-content">
                <div class="purchases-list">
                    {
                        purchases.data.as_ref().map_or_else(|| html!{}, |purchases_list| {
                            if purchases_list.is_empty() {
                                html! {
                                    <p>{"You haven't bought any products yet"}</p>
                                }
                            } else {
                                purchases_list.iter()
                                    .map(|purchase| {
                                        html! {
                                            <PurchaseItem
                                                key={purchase.id}
                                                is_seller={false}
                                                purchase={purchase.clone()}
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

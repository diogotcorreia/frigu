use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{api, components::purchase::buyer_grouped_purchases::BuyerGroupedPurchases};

#[function_component(SellerSummary)]
pub fn seller_summary() -> Html {
    let buyers = use_state(|| None);

    let refresh_purchases = {
        let buyers = buyers.clone();
        Callback::<()>::from(move |_| {
            let buyers = buyers.clone();
            spawn_local(async move {
                match api::seller_summary().await {
                    Ok(buyers_list) => buyers.set(Some(buyers_list)),
                    Err(_error) => buyers.set(None), // TODO handle error
                };
            });
        })
    };

    {
        let buyers = buyers.clone();
        let refresh_purchases = refresh_purchases.clone();
        use_effect(move || {
            if buyers.is_none() {
                refresh_purchases.emit(());
            }

            || {}
        })
    }

    html! {
        <div class="card purchases-card">
            <div class="card-header">
                {"Products Sold"}
            </div>
            <div class="card-content">
                <div class="purchases-list">
                    {
                        if let Some(buyers) = &*buyers {
                            if buyers.is_empty() {
                                html! {
                                    <p>{"You haven't sold any products yet"}</p>
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
                        } else {
                            html! {
                                <p>{"Loading..."}</p>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}

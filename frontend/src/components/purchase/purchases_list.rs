use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{api, components::purchase::purchase_item::PurchaseItem};

#[function_component(PurchasesList)]
pub fn purchases_list() -> Html {
    let purchases = use_state(|| None);

    let refresh_purchases = {
        let purchases = purchases.clone();
        Callback::<()>::from(move |_| {
            let purchases = purchases.clone();
            spawn_local(async move {
                match api::list_purchases().await {
                    Ok(purchases_list) => purchases.set(Some(purchases_list)),
                    Err(_error) => purchases.set(None), // TODO handle error
                };
            });
        })
    };

    {
        let purchases = purchases.clone();
        let refresh_purchases = refresh_purchases.clone();
        use_effect(move || {
            if purchases.is_none() {
                refresh_purchases.emit(());
            }

            || {}
        })
    }

    html! {
        <div class="card purchases-card">
            <div class="card-header">
                {"Purchases"}
            </div>
            <div class="card-content">
                <div class="purchases-list">
                    {
                        if let Some(purchases_list) = &*purchases {
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

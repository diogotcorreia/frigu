use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api,
    components::{navbar::Navbar, product::product_item::ProductItem},
};

#[function_component(ProductPage)]
pub fn product_page() -> Html {
    let products = use_state(|| None);

    {
        let products = products.clone();
        use_effect(move || {
            if products.is_none() {
                spawn_local(async move {
                    match api::list_products().await {
                        Ok(product_list) => products.set(Some(product_list)),
                        Err(_error) => products.set(None), // TODO handle error
                    };
                })
            }

            || {}
        })
    }

    html! {
        <>
            <Navbar />
            <main>
                <div class="card products-card">
                    <div class="card-header">
                        {"Products"}
                    </div>
                    <div class="card-content">
                        <div class="product-list">
                            {
                                if let Some(product_list) = &*products {
                                    if product_list.len() == 0 {
                                        html! {
                                            <p>{"There are no products in stock"}</p>
                                        }
                                    } else {
                                        product_list.into_iter()
                                            .map(|product| {
                                                html! {
                                                    <ProductItem
                                                        key={product.id}
                                                        product={product.clone()}
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
            </main>
        </>
    }
}

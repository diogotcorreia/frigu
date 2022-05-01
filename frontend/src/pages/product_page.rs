use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    api,
    components::{footer::Footer, navbar::Navbar, product::product_item::ProductItem},
    utils::class_if,
};

#[function_component(ProductPage)]
pub fn product_page() -> Html {
    let products = use_async_with_options(
        async move { api::list_products().await },
        UseAsyncOptions::enable_auto(),
    );

    let refresh_products = {
        let products = products.clone();
        Callback::<()>::from(move |_| {
            products.run();
        })
    };

    html! {
        <>
            <Navbar />
            <main>
                <div class={classes!("card", "products-card", class_if(products.loading, "card-loading"))}>
                    <div class="loading-bar" />
                    {
                        products.error.as_ref().map_or_else(|| html!{}, |error| html! {
                            <div class="card-error">{error}</div>
                        })
                    }
                    <div class="card-header">
                        {"Products"}
                    </div>
                    <div class="card-content">
                        <div class="product-list">
                            {
                                if let Some(product_list) = &products.data {
                                    if product_list.is_empty() {
                                        html! {
                                            <p>{"There are no products in stock"}</p>
                                        }
                                    } else {
                                        product_list.iter()
                                            .map(|product| {
                                                html! {
                                                    <ProductItem
                                                        key={product.id}
                                                        product={product.clone()}
                                                        on_update={&refresh_products}
                                                    />
                                                }
                                            })
                                            .collect()
                                    }
                                } else {
                                    html! { }
                                }
                            }
                        </div>
                    </div>
                </div>
                <a class="fab" href="/product/insert">{"+"}</a>
            </main>
            <Footer />
        </>
    }
}

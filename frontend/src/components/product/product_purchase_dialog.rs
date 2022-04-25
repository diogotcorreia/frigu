use yew::prelude::*;

use crate::{
    components::{dialog::Dialog, product::product_item::ProductItemProps},
    utils,
};

#[derive(Clone, Properties, PartialEq)]
pub struct ProductPurchaseDialogProps {
    pub product: ProductItemProps, /* TODO replace with Product DTO in the future */
    pub on_close: Callback<MouseEvent>,
    pub on_buy: Callback<u32>,
}

#[function_component(ProductPurchaseDialog)]
pub fn product_purchase_dialog(props: &ProductPurchaseDialogProps) -> Html {
    let quantity: UseStateHandle<u32> = use_state(|| 1);

    let decrease_qnt_handle = {
        let quantity = quantity.clone();
        Callback::from(move |_| quantity.set(*quantity - 1))
    };

    let increase_qnt_handle = {
        let quantity = quantity.clone();
        Callback::from(move |_| quantity.set(*quantity + 1))
    };

    let on_buy_handle = {
        let quantity = quantity.clone();
        let on_buy = props.on_buy.clone();
        Callback::from(move |_| Callback::emit(&on_buy, *quantity))
    };

    let product = &props.product;

    html! {
        <Dialog>
            <div class="card">
                <div class="card-header">
                    {format!("Purchase {}", product.name.clone())}
                </div>
                <div class="card-content">
                    <div class="product-info--metadata">{format!("By {}", product.seller.clone())}</div>
                    <div class="product-quantity">
                        <div class="product-quantity--selector">
                            <button class="btn product-quantity--btn" disabled={*quantity <= 1} onclick={decrease_qnt_handle}>{"-"}</button>
                            <div class="product-quantity--text">{*quantity}<span class="product-quantity--stock">{format!("/{}", product.stock)}</span></div>
                            <button class="btn product-quantity--btn" disabled={*quantity >= product.stock} onclick={increase_qnt_handle}>{"+"}</button>
                        </div>
                    </div>
                </div>
                <div class="card-actions product-actions">
                    <button onclick={&props.on_close} class="btn product-actions--cancel">
                        {"Cancel"}
                    </button>
                    <button onclick={on_buy_handle} class="btn product-actions--purchase">
                        {format!("Buy for {}", utils::format_display_price(product.price * *quantity))}
                    </button>
                </div>
            </div>
        </Dialog>
    }
}

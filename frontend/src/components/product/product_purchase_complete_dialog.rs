use yew::prelude::*;

use crate::components::dialog::Dialog;

#[derive(Clone, Properties, PartialEq)]
pub struct ProductPurchaseCompleteDialogProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(ProductPurchaseCompleteDialog)]
pub fn product_purchase_complete_dialog(props: &ProductPurchaseCompleteDialogProps) -> Html {
    html! {
        <Dialog>
            <div class="card">
                <div class="card-header">
                    {"Purchase complete!"}
                </div>
                <div class="card-actions product-actions">
                    <button onclick={&props.on_close} class="btn product-actions--done">
                        {"Done"}
                    </button>
                </div>
            </div>
        </Dialog>
    }
}

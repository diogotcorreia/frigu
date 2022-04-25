use yew::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::product::product_item::ProductItem;

#[function_component(ProductPage)]
pub fn product_page() -> Html {
    /*let products = use_state(|| None);

    {
        let products = products.clone();
        use_effect(move || {
            if products.is_none() {
                spawn_local(async move {
                    // TODO
                })
            }
        })
    }*/

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
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={10} />
                        </div>
                    </div>
                </div>
            </main>
        </>
    }
}

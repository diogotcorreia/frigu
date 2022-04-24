use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/products")]
    ProductPage,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::HelloServer => html! { <HelloServer /> },
        Route::ProductPage => html! {<ProductPage />},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <header class="navbar">
            <h1 class="site-title">{"Frigu"}</h1>
            <div class="nav-links">
                <a href="/products" class="active">{"Products"}</a>
                <a href="/my-transactions">{"Transactions"}</a>
            </div>
        </header>
    }
}

#[function_component(ProductPage)]
fn product_page() -> Html {
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
                {/**/""}
            </main>
        </>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct ProductItemProps {
    name: String,
    description: String,
    seller: String,
    stock: u32,
    price: u32,
}

#[function_component(ProductItem)]
fn product_item(product: &ProductItemProps) -> Html {
    let flow_state = use_state(|| PurchaseFlow::None);

    let buy_click_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_| flow_state.set(PurchaseFlow::SelectingQuantity))
    };

    let dialog_close_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |_| flow_state.set(PurchaseFlow::None))
    };

    let dialog_buy_handler = {
        let flow_state = flow_state.clone();
        Callback::from(move |amount: u32| {
            flow_state.set(PurchaseFlow::Loading);
            // TODO send request to backend
            flow_state.set(PurchaseFlow::Complete);
        })
    };

    html! {
        <div class="product-item">
            <div class="product-info">
                <div class="product-info--name">{product.name.clone()}</div>
                <div class="product-info--metadata">
                    {"By "}
                    <span class="product-info--seller">{product.seller.clone()}</span>
                    {" | "}
                    <span class="product-info--stock">{product.stock}</span>
                    {" in stock"}
                </div>
                <div class="product-info--description">{product.description.clone()}</div>
            </div>
            <div class="product-price">
                {format_display_price(product.price)}
            </div>
            <div class="product-actions">
                <button onclick={buy_click_handler} class="btn product-actions--purchase">{"Buy"}</button>
            </div>
            {
            match *flow_state {
                PurchaseFlow::SelectingQuantity => html! {
                    <ProductPurchaseDialog product={product.clone()} on_close={dialog_close_handler} on_buy={dialog_buy_handler} />
                },
                PurchaseFlow::Loading => html! {
                    <Dialog>
                        <div class="card">
                            <div class="card-header">
                                {"Loading..."}
                            </div>
                        </div>
                    </Dialog>
                },
                PurchaseFlow::Complete => html! {
                    <ProductPurchaseCompleteDialog on_close={dialog_close_handler} />
                },
                _ => html! {},
            }
            }
        </div>
    }
}

fn format_display_price(price: u32) -> String {
    format!("{}.{:02}€", price / 100, price % 100)
}

#[derive(Clone, Properties, PartialEq)]
struct DialogProps {
    children: Children,
}

#[function_component(Dialog)]
fn dialog(props: &DialogProps) -> Html {
    html! {
        <div class="dialog dialog-backdrop">
            {props.children.clone()}
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct ProductPurchaseDialogProps {
    product: ProductItemProps, /* TODO replace with Product DTO in the future */
    on_close: Callback<MouseEvent>,
    on_buy: Callback<u32>,
}

#[function_component(ProductPurchaseDialog)]
fn product_purchase_dialog(props: &ProductPurchaseDialogProps) -> Html {
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
                        {format!("Buy for {}", format_display_price(product.price * *quantity))}
                    </button>
                </div>
            </div>
        </Dialog>
    }
}

enum PurchaseFlow {
    None,
    SelectingQuantity,
    Loading,
    Complete,
}

#[derive(Clone, Properties, PartialEq)]
struct ProductPurchaseCompleteDialogProps {
    on_close: Callback<MouseEvent>,
}

#[function_component(ProductPurchaseCompleteDialog)]
fn product_purchase_complete_dialog(props: &ProductPurchaseCompleteDialogProps) -> Html {
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

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}

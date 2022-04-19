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
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                            <ProductItem name="Napolitanas" description="Cenas com chocolate. Yummy!" seller="Rafael Girão" stock={20} price={50} />
                        </div>
                    </div>
                </div>
            </main>
        </>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct ProductItemProps {
    name: String,
    description: String,
    seller: String,
    stock: i32,
    price: u32,
}

#[function_component(ProductItem)]
fn product_item(product: &ProductItemProps) -> Html {
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
                <button class="product-actions--purchase">{"Buy"}</button>
            </div>
        </div>
    }
}

fn format_display_price(price: u32) -> String {
    format!("{}.{:02}€", price / 100, price % 100)
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}

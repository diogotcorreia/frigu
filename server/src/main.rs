use axum::body::{boxed, Body};
use axum::extract::{self, Extension};
use axum::http::{Response, StatusCode};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use entity::{product, sea_orm};
use migration::{Migrator, MigratorTrait};
use product::Entity as Product;
use sea_orm::{prelude::*, Database, JsonValue, QueryOrder, Set};
use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod dtos;
use dtos::ProductDto;
mod errors;
use errors::AppError;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let api_routes = Router::new()
        .route("/hello", get(hello))
        .route("/products", get(list_products))
        .route("/product", post(insert_product));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(get(|req| async move {
            match ServeDir::new(&opt.static_dir).oneshot(req).await {
                Ok(res) => match res.status() {
                    StatusCode::NOT_FOUND => {
                        let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                        let index_content = match fs::read_to_string(index_path).await {
                            Err(_) => {
                                return Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(boxed(Body::from("index file not found")))
                                    .unwrap()
                            }
                            Ok(index_content) => index_content,
                        };

                        Response::builder()
                            .status(StatusCode::OK)
                            .body(boxed(Body::from(index_content)))
                            .unwrap()
                    }
                    _ => res.map(boxed),
                },
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(conn))
                .layer(TraceLayer::new_for_http()),
        );

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

async fn list_products(Extension(ref conn): Extension<DatabaseConnection>) -> Json<Vec<JsonValue>> {
    Json(
        Product::find()
            .filter(product::Column::Stock.gt(0))
            .order_by_desc(product::Column::Stock)
            .into_json()
            .all(conn)
            .await
            .unwrap(),
    )
}

async fn insert_product(
    extract::Json(product_dto): extract::Json<ProductDto>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<ProductDto>, AppError> {
    // validate stock
    let stock = product_dto.stock;
    if stock == 0 {
        return Err(AppError::BadInput("stock must be greater than 0"));
    }
    // validate price
    let price = product_dto.price;
    if price == 0 {
        return Err(AppError::BadInput("price must be greater than 0"));
    }
    // validate name
    let name = product_dto.name.trim();
    if name.len() == 0 {
        return Err(AppError::BadInput("name can't be empty"));
    }
    // validate description
    let description = product_dto.description.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.len() > 0 {
            Some(trimmed.to_string())
        } else {
            None
        }
    });
    // TODO: get seller id from cookies
    let product = product::ActiveModel {
        stock: Set(stock),
        price: Set(price),
        name: Set(name.to_string()),
        description: Set(description),
        ..Default::default()
    };

    let product = product.insert(conn).await.expect("could not insert post"); // TODO

    let new_product_dto = ProductDto::from_entity(product, conn).await;

    Ok(Json(new_product_dto))
}

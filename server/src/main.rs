use axum::body::{boxed, Body};
use axum::extract::Extension;
use axum::http::{Response, StatusCode};
use axum::{response::IntoResponse, routing::get, Json, Router};
use clap::Parser;
use entity::sea_orm;
use entity::{product, user};
use migration::{Migrator, MigratorTrait};
use product::Entity as Product;
use sea_orm::{prelude::*, Database, JsonValue, QueryOrder};
use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use user::Entity as User;

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

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/products", get(list_products))
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

use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use ipnetwork::IpNetwork;

mod dtos;
mod errors;
mod jwt_helpers;
mod product_routes;
mod purchase_routes;
mod user_routes;

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

#[derive(Debug, Clone)]
struct Config {
    hmac_secret: Box<[u8]>,
    admin_subnet: IpNetwork,
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

    let hmac_secret = env::var("HMAC_SECRET").expect("HMAC_SECRET is not set").into_bytes().into();

    let admin_subnet: IpNetwork = {
        let admin_subnet_string = env::var("ADMIN_SUBNET").expect("ADMIN_SUBNET is not set");
        admin_subnet_string.as_str().try_into().expect("ADMIN_SUBNET was not valid")
    };

    let config = Config {
        hmac_secret,
        admin_subnet,
    };

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let api_routes = Router::new()
        .route("/login", post(user_routes::login))
        .route("/register", post(user_routes::register))
        .route("/user/info", get(user_routes::user_info))
        .route("/logout", get(user_routes::logout))
        .route("/products", get(product_routes::list))
        .route("/product", post(product_routes::insert))
        .route("/product/:id/purchase", post(product_routes::purchase))
        .route(
            "/purchases/seller-summary",
            get(purchase_routes::seller_summary),
        )
        .route("/purchases/history", get(purchase_routes::purchase_history))
        .route("/purchase/:id/pay", post(purchase_routes::pay_purchase))
        .route(
            "/purchase/user/:id/pay",
            post(purchase_routes::pay_purchase_user_bulk),
        );

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
                .layer(AddExtensionLayer::new(config))
                .layer(TraceLayer::new_for_http()),
        );

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Unable to start server");
}

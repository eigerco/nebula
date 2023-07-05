pub mod error;

use std::net::SocketAddr;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::RawBody;
use axum::routing::post;
use axum::Router;
use hyper::Method;
use lazy_static::lazy_static;
use tokio::fs;
use tokio::process::Command;
use tower::limit::GlobalConcurrencyLimitLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

lazy_static! {
    static ref CONTRACT_DIR: String =
        std::env::var("CONTRACT_DIR").unwrap_or_else(|_| "../sample-contract".to_string());
    static ref PORT: u16 = std::env::var("PORT")
        .ok()
        .and_then(|it| it.parse().ok())
        .unwrap_or(4000);
}

async fn run(RawBody(body): RawBody) -> Result<Vec<u8>, error::Error> {
    let body = hyper::body::to_bytes(body).await.unwrap();
    if body.is_empty() {
        return Err(error::Error::NoBody);
    }
    let body = String::from_utf8_lossy(&body);
    let contract_dir = fs::canonicalize(&*CONTRACT_DIR).await?;
    fs::write(contract_dir.join("src/lib.rs"), &*body).await?;
    let mut cmd = Command::new("cargo");
    let cmd = cmd
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release");
    debug!(?cmd, "running command");
    let output = cmd.current_dir(&contract_dir).output().await?;
    if !output.status.success() {
        return Err(error::Error::BuildFailed(output));
    }
    let dist = contract_dir
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release");
    let wasm = fs::read(dist.join("sample_contract.wasm")).await?;

    Ok(wasm)
}

#[tokio::main]
async fn main() {
    let contract_dir = &*CONTRACT_DIR;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "playground=trace,backend=trace,hyper=debug,tower_http=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer().with_ansi(std::env::var("NO_ANSI_LOG").is_err()))
        .init();
    debug!(?contract_dir);

    let app = Router::new()
        .route("/run", post(run))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error::timeout_or_500))
                .timeout(Duration::from_secs(10)),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(GlobalConcurrencyLimitLayer::new(1))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::new("0.0.0.0".parse().unwrap(), *PORT);
    info!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

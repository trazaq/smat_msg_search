//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-print-request-response
//! ```

use askama::Template;
use axum::response::Html;
use axum::routing::get;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use smat_msg_search::routes::index;
use std::fmt::{Display, Formatter};
use std::fs;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_print_request_response=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

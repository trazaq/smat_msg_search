use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Site {
    site: String,
}

pub async fn status(site: Query<Site>) -> impl IntoResponse {
    site.site.to_string()
}

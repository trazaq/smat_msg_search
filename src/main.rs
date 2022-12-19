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

//=============================INDEX==================================================

#[derive(Template)]
#[template(path = "index.html")]
struct Sites {
    sites: Vec<String>,
}

/*#[derive(Debug)]
struct Site {
    site: Vec<String>
}

impl Display for Site {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.site.join(","))
    }
}*/

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

lazy_static! {
    // have to enable flag '(?m)' to use '^' and '$'
    static ref RE: Regex =
        Regex::new(r"(?m)^environs=.*$").expect("Error Compiling Regex Expression");
}

async fn index() -> impl IntoResponse {
    //let RE = Regex::new(r"(?m)^environs=.*$").expect("Error Compiling Regex Expression");
    let contents = fs::read_to_string("tests/server.ini");
    match contents {
        Ok(contents) => {
            if let Some(raw_sites) = RE.find(&contents) {
                let sites: Vec<String> = raw_sites
                    .as_str()
                    .strip_prefix("environs=")
                    .unwrap()
                    .split(';')
                    //.into_iter()
                    .filter_map(|site| site.split('/').last())
                    .map(|site| site.into())
                    .sorted()
                    //.chunks(5)
                    //.into_iter()
                    //.map(|chunk| Site {site: chunk.collect()})
                    .collect();

                let template = Sites { sites: sites };
                return HtmlTemplate(template);
                //return format!("{:?}", sites);
            }
            //"No sites".to_string()
            //let template = Sites { sites: vec![Site {site: vec![]}] };
            let template = Sites { sites: vec![] };
            HtmlTemplate(template)
        }
        Err(e) => {
            let template = Sites {
                //sites: vec![Site {site: vec![format!("{}", e)]}],
                sites: vec![e.to_string()]
            };
            HtmlTemplate(template)
            //format!("There's an error {}", e)
        }
    }

    //let template = HelloTemplate;
    //HtmlTemplate(template)
}
//=============================END INDEX==================================================

//! src/routes/index/index

use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{env, fs};

lazy_static! {
    // have to enable flag '(?m)' to use '^' and '$'
    static ref RE: Regex = Regex::new(r"(?m)^environs=.*$").expect("Error Compiling Regex Expression");
}

pub async fn index() -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, mime::TEXT_HTML_UTF_8.to_string())
        .body(sites().unwrap_or_else(|| "No Sites Found".to_string()))
        .unwrap()
}

fn sites() -> Option<String> {
    let contents = match env::var("SERVER_INI") {
        Ok(path) => fs::read_to_string(path),
        Err(e) => {
            return Some(format!(
                "Error: {}!\nSet absolute path to 'server.ini' file in .env file",
                e
            ))
        }
    };

    match contents {
        Ok(contents) => {
            if let Some(raw_sites) = RE.find(&contents) {
                let sites: Vec<Vec<String>> = raw_sites
                    .as_str()
                    .strip_prefix("environs=")
                    .unwrap()
                    .split(';')
                    //.into_iter()
                    .filter_map(|site| site.split('/').last())
                    .map(|site| site.into())
                    .sorted()
                    .chunks(5)
                    .into_iter()
                    .map(|chunk| chunk.collect())
                    .collect();
                return Some(html(sites));
            }
        }
        Err(e) => return Some(e.to_string()),
    }
    None
}

fn html(sites: Vec<Vec<String>>) -> String {
    let mut html = String::new();
    html.push_str("<html>");
    html.push_str("<HEAD><TITLE>Mt Sinai IE Monitor</TITLE></HEAD>");
    html.push_str(r#"<BODY style="background-color:rgb(32,33,36);" text="white">"#);
    html.push_str(r#"<p align="center"><BR>"#);
    html.push_str(r#"<h1 align="center">Interface Monitor Application (Test Environment)</h1>"#);
    html.push_str(r#"<p align="center">&nbsp;</p>"#);
    html.push_str("<p><br>");
    for site in sites {
        html.push_str(r#"<table border="0" align="center">"#);
        html.push_str("<tr>");
        for site in site {
            html.push_str(r#"<td width="200" align="left" title="Status of Interfaces">"#);
            html.push_str(r#"<button STYLE="background-color:"#);
            if let Ok(env) = env::var("ENVIRONMENT") {
                match env.as_str() {
                    "PRODUCTION" => html.push_str("#00e472;"),
                    _ => html.push_str("#ADBECF;"),
                }
            } else {
                html.push_str("#ADBECF;");
            }
            html.push_str(r#"width: 183; height: 75; border: 4px solid white""#);
            html.push_str(r#"ONCLICK="window.location='/smat/status?site="#);
            html.push_str(&site);
            html.push_str(r#"'">"#);
            html.push_str(r#"<font face="Arial" size="4" color="black"><b>"#);
            html.push_str(&site);
            html.push_str("</b></font>");
            html.push_str("</button>");
            html.push_str("</td>");
        }
        html.push_str("</tr>");
        html.push_str("</table>");
    }
    html.push_str("</body>");
    html.push_str("</html>");

    html
}

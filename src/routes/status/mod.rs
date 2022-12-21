use axum::extract::Query;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use hyper::Response;
use rusqlite::{Connection, OpenFlags};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Site {
    site: String,
}

// #[derive(Debug)]
// struct Message {
//     row: String,
// }

pub async fn status(_site: Query<Site>) -> impl IntoResponse {
    let conn = Connection::open_with_flags(
        "tests/iTo_alladt.2022-12-07_00-01-26.smatdb",
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();

    let mut stmt = conn
        .prepare(r#"SELECT CAST(MessageContent AS TEXT) FROM smat_msgs ORDER BY TimeIn ASC;"#)
        .unwrap();

    let mut rows = stmt.query([]).unwrap();

    let mut msgs: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut msg: String = row.get(0).unwrap();
        //let mut msg = String::from_utf8_lossy(&msg).to_string();
        msg = msg.replace("\r", "<br>");
        msg.insert_str(0, "<p>");
        msg.push_str("</p>");
        msgs.push(msg);
    }
    //println!("{:?}", msgs);
    Response::builder()
        .header(CONTENT_TYPE, mime::TEXT_HTML_UTF_8.to_string())
        .body(msgs.join(""))
        .unwrap()
}

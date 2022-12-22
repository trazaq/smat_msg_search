use axum::extract::Query;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use hyper::Response;
use rusqlite::{Connection, Error, OpenFlags};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Site {
    site: String,
}

// #[derive(Debug)]
// struct Message {
//     row: String,
// }

pub async fn status(site: Query<Site>) -> impl IntoResponse {
    let conn = Connection::open_with_flags(
        "tests/fr_verity.2022-12-19_20-29-32.smatdb",
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();

    let sql = r#"SELECT CAST(MessageContent AS TEXT) FROM smat_msgs ORDER BY TimeIn ASC;"#;
    let pragma = format!("pragma key = '{}';pragma cipher_compatibility = 3;", site.site);
    let mut stmt = match conn
        .prepare(sql)
    {
        Ok(stmt) => stmt,
        Err(e) => match e {
            Error::SqliteFailure(code, _) => {
                if code.extended_code == 26 {
                    conn.execute_batch(&pragma).unwrap();
                    conn.prepare(sql).unwrap()
                } else {
                    panic!("{:?}", code)
                }
            }
            _ => {
                panic!("{:?}", e)
            }
        },
    };

    let mut rows = stmt.query([]).unwrap();

    let mut msgs: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut msg: String = row.get(0).unwrap();
        //let mut msg = String::from_utf8_lossy(&msg).to_string();
        msg = msg.replace('\r', "<br>");
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

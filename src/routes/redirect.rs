use mysql::prelude::Queryable;
use mysql::{params, Pool};
use rocket::response::Redirect;
use rocket::State;

#[get("/<id>")]
pub fn redirect_to_url(id: String, db_pool: &State<Pool>) -> Result<Redirect, String> {
    let mut conn = db_pool.get_conn().unwrap();

    let query = r"SELECT long_url FROM url_shortener WHERE id = :id";
    let params = params! {
        "id" => &id
    };

    let long_url: Result<Option<String>, _> = conn.exec_first(query, params);

    match long_url {
        Ok(Some(url)) => {
            let absolute_url = if url.starts_with("http://") || url.starts_with("https://") {
                url
            } else {
                format!("http://{}", url)
            };
            Ok(Redirect::to(absolute_url))
        }
        Ok(None) => Err("No URL found for this ID".to_string()),
        Err(_) => Err("An error occurred while retrieving the URL".to_string()),
    }
}

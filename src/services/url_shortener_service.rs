use mysql::prelude::Queryable;
use mysql::{params, PooledConn};
use nanoid::nanoid;
pub fn generate_short_url(
    conn: &mut PooledConn,
    long_url: &String,
) -> Result<String, mysql::Error> {
    let id = nanoid!(5);
    let generated_url = "http://localhost:8000/".to_string() + &id;

    let query =
        r"INSERT INTO url_shortener(id,long_url, short_url) VALUES(:id,:long_url, :short_url)";
    let params = params! {
        "id" =>id,
        "long_url" => long_url,
        "short_url" => &generated_url
    };

    conn.exec_drop(query, params)?;

    Ok(generated_url)
}

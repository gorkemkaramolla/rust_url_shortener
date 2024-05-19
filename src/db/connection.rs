use mysql::{prelude::*, Opts, Pool, PooledConn};

pub fn establish_connection(url: &str) -> Result<Pool, String> {
    let opts = match Opts::from_url(url) {
        Ok(opts) => opts,
        Err(_) => return Err("Invalid database URL".to_string()),
    };

    let pool = match Pool::new(opts) {
        Ok(pool) => pool,
        Err(_) => return Err("Failed to create connection pool".to_string()),
    };

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to get connection from pool".to_string()),
    };

    match create_table(&mut conn) {
        Ok(_) => Ok(pool),
        Err(_) => return Err("Failed to create table".to_string()),
    }
}
pub fn create_table(conn: &mut PooledConn) -> Result<(), mysql::Error> {
    conn.exec_drop(
        r"CREATE TABLE IF NOT EXISTS url_shortener (
            id VARCHAR(21) PRIMARY KEY,
            long_url VARCHAR(255) NOT NULL,
            short_url VARCHAR(255) NOT NULL UNIQUE
        )",
        (),
    )
}

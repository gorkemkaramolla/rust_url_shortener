use crate::routes::redirect;
use db::connection;
use dotenv::dotenv;
use mysql::prelude::Queryable;
use mysql::Pool;
use std::env;
#[macro_use]
extern crate rocket;

mod routes {
    pub mod redirect;
    pub mod url_shortener;
}

mod db {
    pub mod connection;
}

mod services {
    pub mod url_shortener_service;
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool = connection::establish_connection(&database_url)
        .expect("Failed to establish connection to database");
    rocket::build()
        .mount("/", routes![redirect::redirect_to_url])
        .mount(
            "/url_shortener",
            routes![routes::url_shortener::url_shortener],
        )
        .manage(pool) // Manage the Pool, not a single connection
}

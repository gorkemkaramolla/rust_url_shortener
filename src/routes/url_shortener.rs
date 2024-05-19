use crate::services::url_shortener_service::generate_short_url;
use mysql::Pool;
use rocket::data::{Data, ToByteUnit};
use rocket::serde::{json::Json, Deserialize};
use rocket::{http::Status, post, State};
use serde::Serialize;
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(message: &str) -> ErrorResponse {
        ErrorResponse {
            error: message.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Url {
    pub url: String,
    pub long_url: String,
}

#[derive(Deserialize)]
struct IncomingUrlData {
    url_data: String,
}

#[post("/", data = "<url_data>")]
pub async fn url_shortener(url_data: Data<'_>, db_pool: &State<Pool>) -> (Status, Json<Url>) {
    let raw_data = url_data.open(512.bytes()).into_string().await;

    if let Err(e) = raw_data {
        return (
            Status::InternalServerError,
            Json(Url {
                url: ErrorResponse::new(&format!("Failed to Read Data: {}", e)).error,
                long_url: "".to_string(),
            }),
        );
    }

    let raw_data = raw_data.unwrap();

    match serde_json::from_str::<IncomingUrlData>(&raw_data) {
        Ok(parsed_data) => {
            let db_conn_result = db_pool.get_conn();
            if let Err(e) = db_conn_result {
                return (
                    Status::InternalServerError,
                    Json(Url {
                        url: ErrorResponse::new(&format!("Database connection error: {}", e)).error,
                        long_url: "".to_string(),
                    }),
                );
            }

            let mut conn = db_conn_result.unwrap();

            match generate_short_url(&mut conn, &parsed_data.url_data) {
                Ok(short_url) => (
                    Status::Ok,
                    Json(Url {
                        url: short_url,
                        long_url: parsed_data.url_data.clone(),
                    }),
                ),
                Err(e) => {
                    return (
                        Status::InternalServerError,
                        Json(Url {
                            url: ErrorResponse::new(&format!("Generate Short url error {}", e))
                                .error,
                            long_url: "".to_string(),
                        }),
                    );
                }
            }
        }
        Err(e) => {
            return (
                Status::InternalServerError,
                Json(Url {
                    url: ErrorResponse::new(&format!("Database connection error: {}", e)).error,
                    long_url: "".to_string(),
                }),
            );
        }
    }
}

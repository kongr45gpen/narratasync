use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, sync::Arc};

use serde_json::json;

use axum::{Router, response::Json, routing::get};
use tower_http::cors::{CorsLayer, Any};
use http::HeaderValue;

pub mod models;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    use crate::models::Scenario;
    use crate::schema::scenario::dsl::*;

    // Configure CORS to allow requests from localhost:3001
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                let connection = &mut establish_connection();

                let results = Arc::new(
                    scenario
                        .limit(5)
                        .load::<Scenario>(connection)
                        .expect("Error loading scenarios"),
                );

                println!("{:?}", results);

                Json(json!(&*results))
            }),
        )
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

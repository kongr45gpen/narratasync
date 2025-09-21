use diesel::prelude::*;
use dotenvy::dotenv;
use std::rc::Rc;
use std::{env, sync::Arc};

use serde_json::json;

use axum::{Router, response::Json, routing::get};

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

    // build our application with a single route
    let app = Router::new().route(
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
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

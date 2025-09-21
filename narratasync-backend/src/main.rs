use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use crate::models::Scenario;
    use crate::schema::scenario::dsl::*;

    let connection = &mut establish_connection();

    let results = scenario
        .limit(5)
        .load::<Scenario>(connection)
        .expect("Error loading scenarios");

    println!("{:?}", results);

    println!("Hello, world!");
}


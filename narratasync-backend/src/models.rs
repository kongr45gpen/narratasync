use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable,Selectable,Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub token: String,
}

#[derive(Queryable,Selectable,Debug,Serialize,Clone)]
#[diesel(table_name = crate::schema::scenario)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Scenario {
    pub id: Option<String>,
    pub title: String,
    pub thumbnail: Option<String>,
    pub author: Option<String>,
}
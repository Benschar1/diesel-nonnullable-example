mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct WithName {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct WithoutName {
    pub id: i32,
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let with_name = WithName {
        id: 1,
        name: "a name".to_string(),
    };

    let res = diesel::insert_into(users::table)
        .values(&with_name)
        .execute(&mut conn);

    println!("with name: {res:?}");

    let without_name = WithoutName { id: 2 };

    let res = diesel::insert_into(users::table)
        .values(&without_name)
        .execute(&mut conn);

    println!("without name: {res:?}");
}

use postgres::Client;
use std::env::{self};

pub fn get_test_connection() -> Client {
    let db_url = get_database_url();
    Client::connect(&db_url, postgres::NoTls).unwrap()
}

fn get_database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:23af728bc84f7800f4e9@localhost:9010/postgres?sslmode=disable"
            .to_owned()
    })
}

pub mod models;
pub mod services;

use crate::services::db_service;

fn main() {
    let path = "../nutrient_db/nutrients.db";
    let connection = rusqlite::Connection::open(path).unwrap();

    let foods = db_service::get_foods_by_desc("onion", &connection);

    match foods {
        Ok(foods) => for f in foods { println!("{:?}", f.name) },
        Err(e) => println!("{:?}", e)
    }
}

pub mod models;

use crate::models::food_name::FoodName;

fn main() {
    let path = "../nutrient_db/nutrients.db";
    let connection = rusqlite::Connection::open(path).unwrap();
    let query = "SELECT * FROM food_name WHERE food_description LIKE 'beef, ground%'";
    let mut statement = connection.prepare(query).unwrap();
    let rows = statement.query_map([], |row| {
        FoodName::try_from(row)
    });
    
    match rows {
        Ok(rows) => {
            for r in rows {
                match r {
                    Ok(food) => println!("{:?}", food.name),
                    Err(e) => println!("{:?}", e)
                }
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

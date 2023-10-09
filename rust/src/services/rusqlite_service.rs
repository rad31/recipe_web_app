// use crate::models::food_name::FoodName;

// pub fn get_foods_by_desc(desc: &str, connection: &rusqlite::Connection) -> Result<Vec<FoodName>, rusqlite::Error> {
//     let query = format!("SELECT * FROM food WHERE food_description LIKE '{}%'", desc);
//     let mut statement = connection.prepare(query.as_str())?;

//     let rows = statement.query_map([], |row| FoodName::try_from(row))?;
//     let foods = rows.collect();

//     foods
// }

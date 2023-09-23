use rusqlite::Row;

#[derive(Debug)] 
pub struct FoodName {
    pub id: i32,
    pub code: i32,
    pub group_id: i32,
    pub source_id: i32,
    pub name: String,
}

impl TryFrom<&rusqlite::Row<'_>> for FoodName {
    type Error = rusqlite::Error;

    fn try_from(row: &'_ Row<'_>) -> Result<Self, Self::Error> {
        let food = FoodName {
            id: row.get(0)?,
            code: row.get(1)?,
            group_id: row.get(2)?,
            source_id: row.get(3)?,
            name: row.get(4)?,
        };

        Ok(food)
    }
}

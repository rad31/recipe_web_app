use axum::Router;
use axum::routing::post;
use tracing::debug;

use crate::models::db::{Measure, Macro};
use crate::services::auth_service::error::{Error, Result};
use crate::models::{ModelManager, db::Food};

pub mod error;

pub async fn get_foods_by_name(mm: &ModelManager, food_name: String) -> Result<Vec<Food>> {
    let db = mm.db();

    // Select all foods that contain food_name
    // Sort the results that start with the food_name first, then alphabetically
    let query = "
        SELECT id, description
        FROM food
        WHERE description ILIKE CONCAT('%', $1, '%')
        ORDER BY
            CASE WHEN description ILIKE CONCAT($1, '%') THEN 1 ELSE 2 END,
            description
    ";

    let foods = sqlx::query_as::<_, Food>(query)
        .bind(food_name)
        .fetch_all(db)
        .await?;

    Ok(foods)
}

pub async fn get_measures_by_food(mm: &ModelManager, food_id: i32) -> Result<Vec<Measure>> {
    let db = mm.db();

    let query = "
        SELECT m.id, m.name
        FROM conversion_factor cf
        LEFT JOIN measure m ON m.id = cf.measure_id
        WHERE cf.food_id = $1
        ORDER BY m.name
    ";

    let measures = sqlx::query_as::<_, Measure>(query)
        .bind(food_id)
        .fetch_all(db)
        .await?;

    Ok(measures)
}

pub async fn get_macros_by_food(mm: &ModelManager, food_id: i32, measure_id: i32) -> Result<Macro> {
    let db = mm.db();
    let protein_id = 203;
    let fat_id = 204;
    let carb_id = 205;

    let query = "
        SELECT
            (p.nutrient_value * cf.factor_value) as protein,
            (f.nutrient_value * cf.factor_value) as fat,
            (c.nutrient_value * cf.factor_value) as carb
        FROM conversion_factor AS cf
        LEFT JOIN (
            SELECT nutrient_value, food_id
            FROM nutrient_amount
            WHERE nutrient_id = $1
        ) AS p ON p.food_id = cf.food_id
        LEFT JOIN (
            SELECT nutrient_value, food_id
            FROM nutrient_amount
            WHERE nutrient_id = $2
        ) AS f ON f.food_id = cf.food_id
        LEFT JOIN (
            SELECT nutrient_value, food_id
            FROM nutrient_amount
            WHERE nutrient_id = $3
        ) AS c ON c.food_id = cf.food_id
        WHERE cf.food_id = $4 AND cf.measure_id = $5
    ";

    let macros = sqlx::query_as::<_, Macro>(query)
        .bind(protein_id)
        .bind(fat_id)
        .bind(carb_id)
        .bind(food_id)
        .bind(measure_id)
        .fetch_one(db)
        .await?;

    Ok(macros)
}

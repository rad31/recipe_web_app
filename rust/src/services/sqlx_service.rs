use std::error::Error;
use sqlx::{Row, Connection};

use crate::models::db::Measure;

pub async fn create(measure: Measure, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO measure (measure_id, measure_name, measure_name_f) VALUES ($1, $2, $3)";
    
    sqlx::query(query)
        .bind(&measure.id)
        .bind(&measure.name)
        .bind(&measure.name_f)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn insert_data(conn: &mut sqlx::PgConnection, file_name: &str, table_name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = format!("../nutrient_db/raw_data/{}", file_name);
    // CSV files are encoded in ISO 8859-1 which is named LATIN1 in postgres
    let query = format!("COPY {} FROM STDIN (FORMAT CSV, HEADER TRUE, ENCODING 'LATIN1')", table_name);
    let file = tokio::fs::File::open(file_path).await?;

    let mut copy_in = conn.copy_in_raw(&query).await?;
    copy_in.read_from(file).await?;
    copy_in.finish().await?;

    Ok(())
}

pub async fn insert_nutrient_data(nutrients_connection_string: &str) -> Result<(), Box<dyn Error>> {
    // Use PgConnection for importing data since the copy functionality isn't implemented for PgPool
    let mut conn = sqlx::postgres::PgConnection::connect(nutrients_connection_string).await?;

    insert_data(&mut conn, "YIELD NAME.csv", "yield").await?;
    insert_data(&mut conn, "REFUSE NAME.csv", "refuse").await?;
    insert_data(&mut conn, "MEASURE NAME.csv", "measure").await?;
    insert_data(&mut conn, "NUTRIENT NAME.csv", "nutrient").await?;
    insert_data(&mut conn, "FOOD GROUP.csv", "food_group").await?;
    insert_data(&mut conn, "FOOD SOURCE.csv", "food_source").await?;
    insert_data(&mut conn, "NUTRIENT SOURCE.csv", "nutrient_source").await?;
    insert_data(&mut conn, "FOOD NAME.csv", "food").await?;
    insert_data(&mut conn, "YIELD AMOUNT.csv", "yield_amount").await?;
    insert_data(&mut conn, "REFUSE AMOUNT.csv", "refuse_amount").await?;
    insert_data(&mut conn, "CONVERSION FACTOR.csv", "conversion_factor").await?;
    insert_data(&mut conn, "NUTRIENT AMOUNT.csv", "nutrient_amount").await?;

    conn.close().await?;
    Ok(())
}

pub async fn is_data_present(pool: &sqlx::PgPool) -> Result<bool, Box<dyn Error>> {
    let res = sqlx::query("SELECT COUNT(0) FROM yield")
        .fetch_one(pool)
        .await?;

    let count: i64 = res.get("count");

    Ok(count > 0)
}

pub async fn insert_new_user(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    Ok(())
}

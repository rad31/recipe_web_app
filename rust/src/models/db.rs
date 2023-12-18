use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Measure {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Food {
    pub id: i32,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Macro {
    pub calories: f32,
    pub protein: f32,
    pub fat: f32,
    pub carb: f32,
}

use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Measure {
    #[sqlx(rename = "measure_id")]
    pub id: i32,
    #[sqlx(rename = "measure_name")]
    pub name: String,
    #[sqlx(rename = "measure_name_f")]
    pub name_f: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: Uuid,
    pub username: String,
    pub password_hash: String
}

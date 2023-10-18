use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Measure {
    pub id: i32,
    pub name: String,
    pub name_f: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    // pub salt: [u8; 16],
}

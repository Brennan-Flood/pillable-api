use sqlx::{PgPool, Error, Row};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

pub async fn create_user(pool: &PgPool, name: String) -> Result<User, Error> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO users (id, name) VALUES ($1, $2)"
    )
    .bind(id)
    .bind(&name)
    .execute(pool)
    .await?;

    Ok(User { id, name })
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    let rows = sqlx::query("SELECT id, name FROM users")
        .fetch_all(pool)
        .await?;

    let users: Vec<User> = rows.iter()
        .map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    Ok(users)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

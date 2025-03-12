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
    let row = sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2) RETURNING id, name")
        .bind(id)
        .bind(&name)
        .fetch_one(pool)
        .await?;

    Ok(User {
        id: row.get("id"),
        name: row.get("name"),
    })
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplement {
    pub id: i32,
    pub name: String,
    pub store_link: Option<String>,
    pub image_url: Option<String>,
}

pub async fn create_supplement(pool: &PgPool, name: String, store_link: Option<String>, image_url: Option<String>) -> Result<Supplement, Error> {
    let row = sqlx::query("INSERT INTO supplements (name, store_link, image_url) VALUES ($1, $2, $3) RETURNING id, name, store_link, image_url")
        .bind(name)
        .bind(store_link)
        .bind(image_url)
        .fetch_one(pool)
        .await?;

    Ok(Supplement {
        id: row.get("id"),
        name: row.get("name"),
        store_link: row.get("store_link"),
        image_url: row.get("image_url"),
    })
}

pub async fn get_supplements(pool: &PgPool) -> Result<Vec<Supplement>, Error> {
    let rows = sqlx::query("SELECT id, name, store_link, image_url FROM supplements")
        .fetch_all(pool)
        .await?;

    let supplements: Vec<Supplement> = rows.iter()
        .map(|row| Supplement {
            id: row.get("id"),
            name: row.get("name"),
            store_link: row.get("store_link"),
            image_url: row.get("image_url"),
        })
        .collect();

    Ok(supplements)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Follow {
    pub following_user: Uuid,
    pub followed_user: Uuid,
}

pub async fn create_follow(pool: &PgPool, following_user: Uuid, followed_user: Uuid) -> Result<Follow, Error> {
    let row = sqlx::query("INSERT INTO follows (following_user, followed_user) VALUES ($1, $2) RETURNING following_user, followed_user")
        .bind(following_user)
        .bind(followed_user)
        .fetch_one(pool)
        .await?;

    Ok(Follow {
        following_user: row.get("following_user"),
        followed_user: row.get("followed_user"),
    })
}

pub async fn get_follows(pool: &PgPool) -> Result<Vec<Follow>, Error> {
    let rows = sqlx::query("SELECT following_user, followed_user FROM follows")
        .fetch_all(pool)
        .await?;

    let follows: Vec<Follow> = rows.iter()
        .map(|row| Follow {
            following_user: row.get("following_user"),
            followed_user: row.get("followed_user"),
        })
        .collect();

    Ok(follows)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
    pub supplement_id: i32,
    pub rating: i32,
}

pub async fn create_review(pool: &PgPool, title: String, body: String, user_id: Uuid, supplement_id: i32, rating: i32) -> Result<Review, Error> {
    let row = sqlx::query("INSERT INTO reviews (title, body, user_id, supplement_id, rating) VALUES ($1, $2, $3, $4, $5) RETURNING id, title, body, user_id, supplement_id, rating")
        .bind(title)
        .bind(body)
        .bind(user_id)
        .bind(supplement_id)
        .bind(rating)
        .fetch_one(pool)
        .await?;

    Ok(Review {
        id: row.get("id"),
        title: row.get("title"),
        body: row.get("body"),
        user_id: row.get("user_id"),
        supplement_id: row.get("supplement_id"),
        rating: row.get("rating"),
    })
}

pub async fn get_reviews(pool: &PgPool) -> Result<Vec<Review>, Error> {
    let rows = sqlx::query("SELECT id, title, body, user_id, supplement_id, rating FROM reviews")
        .fetch_all(pool)
        .await?;

    let reviews: Vec<Review> = rows.iter()
        .map(|row| Review {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            user_id: row.get("user_id"),
            supplement_id: row.get("supplement_id"),
            rating: row.get("rating"),
        })
        .collect();

    Ok(reviews)
}

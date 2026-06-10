use crate::{api::ServerError, server::db};
use bcrypt::{DEFAULT_COST, hash, verify};
use dioxus::core::Result;

pub async fn create_account(username: &str, password: &str) -> Result<i64> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)",
        username
    )
    .fetch_one(db().await)
    .await?
    .ok_or(ServerError::UnknownError)?;
    if exists {
        return Err(ServerError::UsernameExists.into());
    }
    let hashed_pw = hash(password, DEFAULT_COST)?;

    let id = sqlx::query_scalar!(
        "INSERT INTO Users(username, password_hash) VALUES ($1, $2) RETURNING id",
        username,
        hashed_pw,
    )
    .fetch_one(db().await)
    .await?;
    Ok(id)
}

pub async fn check_login_details(username: &str, password: &str) -> Result<i64> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM Users WHERE username=$1",
        username
    )
    .fetch_one(db().await)
    .await?;
    if verify(password, &user.password_hash)? {
        Ok(user.id)
    } else {
        Err(ServerError::FailedLogin.into())
    }
}

use axum_session_auth::AuthSessionLayer;
use axum_session_sqlx::SessionPgPool;
use sqlx::PgPool;
use std::sync::OnceLock;

use crate::api;

pub mod account;
pub mod queries;
pub mod settings;

static POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn db() -> &'static PgPool {
    match POOL.get() {
        Some(pool) => pool,
        None => {
            dotenvy::dotenv().unwrap();
            let pool = PgPool::connect_lazy(
                &std::env::var("DATABASE_URL").expect("could not get env var"),
            )
            .expect("could not connect to db");
            POOL.set(pool).unwrap();
            POOL.get().unwrap()
        }
    }
}

pub type AuthLayer = AuthSessionLayer::<api::User, i64, SessionPgPool, PgPool>;
pub type Session = axum_session_auth::AuthSession<api::User, i64, SessionPgPool, PgPool>;

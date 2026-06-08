use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[cfg(feature = "server")]
mod user_impl {
    use crate::api::ServerError;

    use super::*;
    use async_trait::async_trait;
    use axum_session_auth::{Authentication, HasPermission};
    use sqlx::PgPool;

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
            match perm {
                "Token::UseAdmin" => true,
                "Token::ModifyUser" => true,
                _ => false,
            }
        }
    }

    #[async_trait]
    impl Authentication<User, i64, PgPool> for User {
        // This is run when the user has logged in and has not yet been Cached in the system.
        // Once ran it will load and cache the user.
        async fn load_user(userid: i64, pool: Option<&PgPool>) -> anyhow::Result<User> {
            if userid == 1 {
                return Ok(User {
                    id: userid,
                    username: "Guest".to_string(),
                });
            }
            // should be set in constructor
            let pool = pool.ok_or(ServerError::UnknownError)?;
            let user = sqlx::query_as!(User, "SELECT id, username FROM Users WHERE id=$1", userid)
                .fetch_optional(pool)
                .await?;
            Ok(user.unwrap())
        }

        // This function is used internally to determine if they are logged in or not.
        fn is_authenticated(&self) -> bool {
            self.id != 1
        }

        fn is_active(&self) -> bool {
            self.id != 1
        }

        fn is_anonymous(&self) -> bool {
            self.id == 1
        }
    }
}

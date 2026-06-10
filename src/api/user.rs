use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            username: Default::default(),
        }
    }
}

#[cfg(feature = "server")]
mod user_impl {
    use super::*;
    use crate::{api::ServerError, server::settings::GLOBAL_SETTINGS};
    use async_trait::async_trait;
    use axum_session_auth::{Authentication, HasPermission};
    use sqlx::PgPool;

    fn default_perms(perm: &str) -> bool {
        match perm {
            "upload" => GLOBAL_SETTINGS.anyone_can_upload,
            "change_settings" => GLOBAL_SETTINGS.anyone_can_change_settings,
            "delete_own_image" => false, // can't verify if any drawing is your own if you're not logged in
            "delete_any_image" => GLOBAL_SETTINGS.anyone_can_delete_any_image,
            _ => false,
        }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, pool: &Option<&PgPool>) -> bool {
            dbg!(perm);
            if self.id == -1 {
                return default_perms(perm);
            }
            let role = match sqlx::query_scalar!("SELECT role FROM Users WHERE id=$1", self.id)
                .fetch_one(pool.unwrap())
                .await
            {
                Ok(role) => role,
                Err(e) => {
                    dioxus::prelude::error!("{:?}", e);
                    return false;
                }
            };
            if let Some(role_id) = role {
                macro_rules! query_perm {
                    ($query:literal) => {
                        sqlx::query_scalar!($query, role_id)
                            .fetch_one(pool.unwrap())
                            .await
                            .unwrap_or_default()
                    };
                }
                match perm {
                    "upload" => query_perm!("SELECT can_upload FROM Roles WHERE id=$1"),
                    "change_settings" => {
                        query_perm!("SELECT can_change_settings FROM Roles WHERE id=$1")
                    }
                    "delete_own_image" => {
                        query_perm!("SELECT can_delete_own_image FROM Roles WHERE id=$1")
                    }
                    "delete_any_image" => {
                        query_perm!("SELECT can_delete_any_image FROM Roles WHERE id=$1")
                    }
                    _ => false,
                }
            } else {
                default_perms(perm)
            }
        }
    }

    #[async_trait]
    impl Authentication<User, i64, PgPool> for User {
        // This is run when the user has logged in and has not yet been Cached in the system.
        // Once ran it will load and cache the user.
        async fn load_user(userid: i64, pool: Option<&PgPool>) -> anyhow::Result<User> {
            // should be set in constructor
            let pool = pool.ok_or(ServerError::UnknownError)?;
            let user = sqlx::query_as!(User, "SELECT id, username FROM Users WHERE id=$1", userid)
                .fetch_optional(pool)
                .await?;
            Ok(user.unwrap())
        }

        // This function is used internally to determine if they are logged in or not.
        fn is_authenticated(&self) -> bool {
            self.id != -1
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            self.id == 1
        }
    }
}

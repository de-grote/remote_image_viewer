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

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub can_upload: bool,
    pub can_change_admin_settings: bool,
    pub can_delete_own_image: bool,
    pub can_delete_any_image: bool,
}

#[cfg(feature = "server")]
impl Default for Permissions {
    fn default() -> Self {
        use crate::server::settings::global_settings;
        let settings = global_settings();
        Self {
            can_upload: settings.anyone_can_upload,
            can_change_admin_settings: false, // even having this as an option would almost be a security issue
            can_delete_own_image: false, // can't verify if any drawing is your own if you're not logged in
            can_delete_any_image: settings.anyone_can_delete_any_image,
        }
    }
}

#[cfg(feature = "server")]
mod user_impl {
    use super::*;
    use crate::api::ServerError;
    use async_trait::async_trait;
    use axum_session_auth::{Authentication, HasPermission};
    use sqlx::PgPool;

    fn default_perms(perm: &str) -> bool {
        let permissions = Permissions::default();
        match perm {
            "upload" => permissions.can_upload,
            "change_admin_settings" => permissions.can_change_admin_settings,
            "delete_own_image" => permissions.can_delete_own_image,
            "delete_any_image" => permissions.can_delete_any_image,
            _ => false,
        }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, pool: &Option<&PgPool>) -> bool {
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
                    "change_admin_settings" => {
                        query_perm!("SELECT can_change_admin_settings FROM Roles WHERE id=$1")
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

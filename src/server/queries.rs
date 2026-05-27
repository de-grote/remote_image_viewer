use dioxus::prelude::*;
use sqlx::PgPool;
use std::{collections::BTreeSet, sync::OnceLock};

use crate::{
    api::{Image, ServerError},
    server::settings::GLOBAL_SETTINGS,
};

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

pub async fn upload_new_image(link: String, tags: BTreeSet<String>) -> Result<i64> {
    let mut connection = db().await.begin().await?;
    let link_id = sqlx::query_scalar!("INSERT INTO Link(link) VALUES ($1) RETURNING id", link)
        .fetch_one(&mut *connection)
        .await?;
    let image_id = sqlx::query_scalar!(
        "INSERT INTO Image(hash, link, preview_link) VALUES (NULL, $1, NULL) RETURNING id",
        link_id,
    )
    .fetch_one(&mut *connection)
    .await?;

    for tag in tags {
        let tag_id = sqlx::query_scalar!("SELECT id FROM Tag WHERE tag=$1", tag)
            .fetch_optional(&mut *connection)
            .await
            .unwrap();
        let tag_id = match tag_id {
            Some(id) => id,
            None => {
                if GLOBAL_SETTINGS.create_unknown_tags {
                    sqlx::query_scalar!("INSERT INTO Tag(tag) VALUES ($1) RETURNING id", tag)
                        .fetch_one(&mut *connection)
                        .await
                        .unwrap()
                } else {
                    connection.rollback().await?;
                    return Err(ServerError::CreateUnknownTag.into());
                }
            }
        };
        sqlx::query!(
            "INSERT INTO Image_Tag(image, tag) VALUES ($1, $2)",
            image_id,
            tag_id,
        )
        .execute(&mut *connection)
        .await
        .unwrap();
    }

    connection.commit().await?;

    Ok(image_id)
}

pub async fn search_tags(tags: Vec<String>) -> Result<Vec<Image>> {
    Ok(sqlx::query_as!(
        Image,
        "SELECT Image.id, Link.link, PreviewLink.link AS \"preview_link?\" FROM Image
            JOIN Image_Tag ON Image.id = Image_Tag.image
            JOIN Link ON Link.id = Image.link
            LEFT JOIN Link AS PreviewLink ON PreviewLink.id = Image.preview_link
            WHERE Image_Tag.tag IN (SELECT id FROM Tag WHERE tag = ANY($1))",
        &tags,
    )
    .fetch_all(db().await)
    .await?)
}

pub async fn search_all() -> Result<Vec<Image>> {
    Ok(sqlx::query_as!(
        Image,
        "SELECT Image.id, Link.link, PreviewLink.link AS \"preview_link?\" FROM Image
            JOIN Link ON Link.id = Image.link
            LEFT JOIN Link AS PreviewLink ON PreviewLink.id = Image.preview_link",
    )
    .fetch_all(db().await)
    .await?)
}

pub async fn get_image(id: i64) -> Result<Image> {
    Ok(sqlx::query_as!(
        Image,
        "SELECT Image.id, Link.link, PreviewLink.link AS \"preview_link?\" FROM Image
            JOIN Link ON Link.id = Image.link
            LEFT JOIN Link AS PreviewLink ON PreviewLink.id = Image.preview_link
            WHERE Image.id = $1",
        id,
    )
    .fetch_one(db().await)
    .await?)
}

pub async fn get_tags(id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT Tag.tag FROM Tag
            JOIN Image_Tag on Image_Tag.tag = Tag.id
            WHERE Image_Tag.image = $1",
        id,
    )
    .fetch_all(db().await)
    .await?)
}

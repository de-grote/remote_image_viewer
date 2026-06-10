use crate::{
    api::{Image, ServerError},
    server::{db, settings::GLOBAL_SETTINGS},
};
use dioxus::prelude::*;
use sqlx::PgTransaction;
use std::collections::BTreeSet;

pub async fn upload_new_image(link: String, tags: BTreeSet<String>) -> Result<i64> {
    let mut connection = db().await.begin().await?;
    let link_id = sqlx::query_scalar!("INSERT INTO Links(link) VALUES ($1) RETURNING id", link)
        .fetch_one(&mut *connection)
        .await?;
    let image_id = sqlx::query_scalar!(
        "INSERT INTO Images(hash, link, preview_link) VALUES (NULL, $1, NULL) RETURNING id",
        link_id,
    )
    .fetch_one(&mut *connection)
    .await?;

    add_tags_inner(image_id, tags, connection).await?;

    Ok(image_id)
}

pub async fn update_tags(
    image_id: i64,
    added: BTreeSet<String>,
    removed: BTreeSet<String>,
) -> Result<()> {
    let mut connection = db().await.begin().await?;
    let removed = removed.into_iter().collect::<Vec<_>>();
    sqlx::query!(
        "DELETE FROM Images_Tags WHERE image=$1 AND tag IN (SELECT id FROM Tags WHERE tag=ANY($2))",
        image_id,
        &removed
    )
    .execute(&mut *connection)
    .await?;
    add_tags_inner(image_id, added, connection).await
}

async fn add_tags_inner<'a>(
    image_id: i64,
    tags: BTreeSet<String>,
    mut connection: PgTransaction<'_>,
) -> Result<()> {
    for tag in tags {
        let tag_id = sqlx::query_scalar!("SELECT id FROM Tags WHERE tag=$1", tag)
            .fetch_optional(&mut *connection)
            .await
            .unwrap();
        let tag_id = match tag_id {
            Some(id) => id,
            None => {
                if GLOBAL_SETTINGS.create_unknown_tags {
                    sqlx::query_scalar!("INSERT INTO Tags(tag) VALUES ($1) RETURNING id", tag)
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
            "INSERT INTO Images_Tags(image, tag) VALUES ($1, $2)",
            image_id,
            tag_id,
        )
        .execute(&mut *connection)
        .await
        .unwrap();
    }

    connection.commit().await?;
    Ok(())
}

pub async fn search_tags(tags: Vec<String>) -> Result<Vec<Image>> {
    Ok(sqlx::query_as!(
        Image,
        "SELECT Images.id, Links.link, PreviewLinks.link AS \"preview_link?\" FROM Images
            JOIN Images_Tags ON Images.id = Images_Tags.image
            JOIN Links ON Links.id = Images.link
            LEFT JOIN Links AS PreviewLinks ON PreviewLinks.id = Images.preview_link
            WHERE Images_Tags.tag IN (SELECT id FROM Tags WHERE tag = ANY($1))",
        &tags,
    )
    .fetch_all(db().await)
    .await?)
}

pub async fn search_all() -> Result<Vec<Image>> {
    Ok(sqlx::query_as!(
        Image,
        "SELECT Images.id, Links.link, PreviewLinks.link AS \"preview_link?\" FROM Images
            JOIN Links ON Links.id = Images.link
            LEFT JOIN Links AS PreviewLinks ON PreviewLinks.id = Images.preview_link",
    )
    .fetch_all(db().await)
    .await?)
}

pub async fn get_image(id: i64) -> Result<Image> {
    Ok(sqlx::query_as!(
        Image,
        "SELECT Images.id, Links.link, PreviewLinks.link AS \"preview_link?\" FROM Images
            JOIN Links ON Links.id = Images.link
            LEFT JOIN Links AS PreviewLinks ON PreviewLinks.id = Images.preview_link
            WHERE Images.id = $1",
        id,
    )
    .fetch_one(db().await)
    .await?)
}

pub async fn get_tags(id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT Tags.tag FROM Tags
            JOIN Images_Tags on Images_Tags.tag = Tags.id
            WHERE Images_Tags.image = $1",
        id,
    )
    .fetch_all(db().await)
    .await?)
}

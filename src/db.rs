use sqlx::pgPool;
use uuid::Uuid;
use nanoid::nanoid;
use crate::models::{Link, CreateLinkRequest, UpdateLinkRequest};

async fn generate_unique_slug(pool: &pgPool) -> String {
    loop {
        let slug = nanoid!(6);

        let exists = sqlx::query("SELECT 1 FROM links WHERE slug = $1", slug)
            .fetch_optional(pool)
            .await
            .unwrap()
            .is_some();

        if !exists {
            return slug;
        }
    }
}

pub async fn create_link(
    pool: &pgPool,
    req: CreateLinkRequest
) -> Result<Link, sqlx::Error> {
    let id = Uuid::new_v4();
    let slug = match req.custom_slug {
        Some(custom) => {
            let exists = sqlx::query!("SELECT 1 FROM links WHERE slug = $1", custom)
                .fetch_optional(pool)
                .await?
                .is_some();

            if exists {
                return Err(sqlx::Error::RowNotFound);
            }
            custom
        },
        None => generate_unique_slug(pool).await,
    };

    pub async fn get_link_by_slug(pool: &pgPool, slug: &str) -> Result<Option<Link>, sqlx::Error> {
        let link = sqlx::query_as!(
            Link,
            "SELECT id, slug, original_url, clicks, created_at, updated_at FROM links WHERE slug = $1",
            slug
        )
        .fetch_optional(pool)
        .await?;

        Ok(())
    }

    
}

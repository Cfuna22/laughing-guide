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

    pub async fn increment_link(pool: &pgPool, slug: &str) -> Result<Vec<Link>, sqlx::Error> {
        let links = sqlx::query_as!(
            Link,
            "SELECT id, slug, original_url, clicks, created_at, updated_at FROM links ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;
    
        Ok((links))
    }

    pub async fn update_link(
        pool: &pgPool,
        link_id:Uuid,
        req: UpdateLinkRequest,
    ) -> Result<Option<Link>, sqlx::Error> {
        let current = sqlx::query_as!(
            Link,
            "SELECT id, slug, original_url, created_at, updated_at FROM links WHERE id = $1",
            link_id
        )
        .fetch_optional(pool)
        .await?;
        
        let mut current = match current {
            Some(link) => link,
            None => return Ok(None),
        };

        if let Some(new_url) = req.original_url {
            current.original_url = new_url;
        }

        if let Some(new_slug) = req.slug {
            let exists = sqlx::query!("SELECT 1 FROM links WHERE slug = $1 AND id != $2", new_slug, link_id)
                .fetch_optional(pool)
                .await?
                .is_some();

            if exists {
                return Err(sqlx::Error::RowNotFound);
            }
            current.slug = new_slu
        }

        let updated = sqlx::query_as!(
            Link,
            r#"UPDATE links SET slug = $1, original_url = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, slug, original_url, clicks, created_at, updated_at
            "#,
            current.slug,
            current.original_url,
            link_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(updated)
    }

    pub async fn delete_link(pool: &pgPool, link_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM links WHERE id = $1", link_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

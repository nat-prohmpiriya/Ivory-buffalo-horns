use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::user::{CreateUser, UpdateUser, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, firebase_uid, email, display_name, photo_url, provider,
                   created_at, updated_at, last_login_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_firebase_uid(pool: &PgPool, firebase_uid: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, firebase_uid, email, display_name, photo_url, provider,
                   created_at, updated_at, last_login_at, deleted_at
            FROM users
            WHERE firebase_uid = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(firebase_uid)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(pool: &PgPool, input: CreateUser) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (firebase_uid, email, display_name, photo_url, provider)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, firebase_uid, email, display_name, photo_url, provider,
                      created_at, updated_at, last_login_at, deleted_at
            "#,
        )
        .bind(&input.firebase_uid)
        .bind(&input.email)
        .bind(&input.display_name)
        .bind(&input.photo_url)
        .bind(&input.provider)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update(pool: &PgPool, firebase_uid: &str, input: UpdateUser) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = COALESCE($2, email),
                display_name = COALESCE($3, display_name),
                photo_url = COALESCE($4, photo_url),
                updated_at = NOW()
            WHERE firebase_uid = $1 AND deleted_at IS NULL
            RETURNING id, firebase_uid, email, display_name, photo_url, provider,
                      created_at, updated_at, last_login_at, deleted_at
            "#,
        )
        .bind(firebase_uid)
        .bind(&input.email)
        .bind(&input.display_name)
        .bind(&input.photo_url)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_last_login(pool: &PgPool, firebase_uid: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE firebase_uid = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(firebase_uid)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert(pool: &PgPool, input: CreateUser) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (firebase_uid, email, display_name, photo_url, provider)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (firebase_uid) DO UPDATE SET
                email = COALESCE(EXCLUDED.email, users.email),
                display_name = COALESCE(EXCLUDED.display_name, users.display_name),
                photo_url = COALESCE(EXCLUDED.photo_url, users.photo_url),
                last_login_at = NOW(),
                updated_at = NOW(),
                deleted_at = NULL
            RETURNING id, firebase_uid, email, display_name, photo_url, provider,
                      created_at, updated_at, last_login_at, deleted_at
            "#,
        )
        .bind(&input.firebase_uid)
        .bind(&input.email)
        .bind(&input.display_name)
        .bind(&input.photo_url)
        .bind(&input.provider)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn soft_delete(pool: &PgPool, firebase_uid: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE firebase_uid = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(firebase_uid)
        .execute(pool)
        .await?;

        Ok(())
    }
}

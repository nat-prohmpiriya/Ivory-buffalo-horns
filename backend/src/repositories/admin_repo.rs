use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::admin::AdminLog;
use crate::models::user::User;

pub struct AdminRepository;

impl AdminRepository {
    // ==================== User Management ====================

    /// Get all users with pagination
    pub async fn list_users(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, firebase_uid, email, display_name, photo_url, provider,
                   created_at, updated_at, last_login_at, deleted_at,
                   is_admin, banned_at, banned_reason
            FROM users
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    /// Search users by email or display name
    pub async fn search_users(
        pool: &PgPool,
        query: &str,
        limit: i64,
    ) -> AppResult<Vec<User>> {
        let search_pattern = format!("%{}%", query);
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, firebase_uid, email, display_name, photo_url, provider,
                   created_at, updated_at, last_login_at, deleted_at,
                   is_admin, banned_at, banned_reason
            FROM users
            WHERE deleted_at IS NULL
              AND (email ILIKE $1 OR display_name ILIKE $1)
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(&search_pattern)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    /// Get user by ID (for admin, includes deleted users)
    pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, firebase_uid, email, display_name, photo_url, provider,
                   created_at, updated_at, last_login_at, deleted_at,
                   is_admin, banned_at, banned_reason
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Ban a user
    pub async fn ban_user(
        pool: &PgPool,
        user_id: Uuid,
        reason: Option<String>,
    ) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET banned_at = NOW(), banned_reason = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, firebase_uid, email, display_name, photo_url, provider,
                      created_at, updated_at, last_login_at, deleted_at,
                      is_admin, banned_at, banned_reason
            "#,
        )
        .bind(user_id)
        .bind(reason)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Unban a user
    pub async fn unban_user(pool: &PgPool, user_id: Uuid) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET banned_at = NULL, banned_reason = NULL, updated_at = NOW()
            WHERE id = $1
            RETURNING id, firebase_uid, email, display_name, photo_url, provider,
                      created_at, updated_at, last_login_at, deleted_at,
                      is_admin, banned_at, banned_reason
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Set user admin status
    pub async fn set_admin(pool: &PgPool, user_id: Uuid, is_admin: bool) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET is_admin = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, firebase_uid, email, display_name, photo_url, provider,
                      created_at, updated_at, last_login_at, deleted_at,
                      is_admin, banned_at, banned_reason
            "#,
        )
        .bind(user_id)
        .bind(is_admin)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    // ==================== Statistics ====================

    /// Get total user count
    pub async fn count_users(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE deleted_at IS NULL"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get active users in last 24 hours
    pub async fn count_active_users_24h(pool: &PgPool) -> AppResult<i64> {
        let cutoff = Utc::now() - Duration::hours(24);
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE last_login_at > $1 AND deleted_at IS NULL"
        )
        .bind(cutoff)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get banned user count
    pub async fn count_banned_users(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE banned_at IS NOT NULL AND deleted_at IS NULL"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get total village count
    pub async fn count_villages(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM villages"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get total alliance count
    pub async fn count_alliances(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM alliances"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get battles count today
    pub async fn count_battles_today(pool: &PgPool) -> AppResult<i64> {
        let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM battle_reports WHERE occurred_at >= $1"
        )
        .bind(today_start)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get village count for a user
    pub async fn count_user_villages(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM villages WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    // ==================== Admin Logs ====================

    /// Create admin log entry
    pub async fn create_log(
        pool: &PgPool,
        admin_id: Uuid,
        action: &str,
        target_type: &str,
        target_id: Option<Uuid>,
        details: Option<serde_json::Value>,
    ) -> AppResult<AdminLog> {
        let log = sqlx::query_as::<_, AdminLog>(
            r#"
            INSERT INTO admin_logs (admin_id, action, target_type, target_id, details)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, admin_id, action, target_type, target_id, details, created_at
            "#,
        )
        .bind(admin_id)
        .bind(action)
        .bind(target_type)
        .bind(target_id)
        .bind(details)
        .fetch_one(pool)
        .await?;

        Ok(log)
    }

    /// Get admin logs with pagination
    pub async fn list_logs(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<AdminLog>> {
        let logs = sqlx::query_as::<_, AdminLog>(
            r#"
            SELECT id, admin_id, action, target_type, target_id, details, created_at
            FROM admin_logs
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }

    // ==================== Resource Management ====================

    /// Adjust village resources (for emergency fixes)
    pub async fn adjust_resources(
        pool: &PgPool,
        village_id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE villages
            SET wood = GREATEST(0, wood + $2),
                clay = GREATEST(0, clay + $3),
                iron = GREATEST(0, iron + $4),
                crop = GREATEST(0, crop + $5),
                last_resource_update = NOW()
            WHERE id = $1
            "#,
        )
        .bind(village_id)
        .bind(wood)
        .bind(clay)
        .bind(iron)
        .bind(crop)
        .execute(pool)
        .await?;

        Ok(())
    }
}

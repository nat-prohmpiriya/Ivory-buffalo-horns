use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::admin::{
    AdminHeroResponse, AdminUserResponse, AdminVillageResponse,
    PlayerDetailResponse, ServerStatsResponse, AdminAllianceInfoResponse,
};
use crate::repositories::admin_repo::AdminRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::repositories::hero_repo::HeroRepository;

pub struct AdminService;

impl AdminService {
    // ==================== User Management ====================

    /// Get all users with pagination
    pub async fn list_users(
        pool: &PgPool,
        page: i64,
        per_page: i64,
    ) -> AppResult<Vec<AdminUserResponse>> {
        let offset = (page - 1) * per_page;
        let users = AdminRepository::list_users(pool, per_page, offset).await?;

        let mut responses = Vec::new();
        for user in users {
            let village_count = AdminRepository::count_user_villages(pool, user.id).await?;
            responses.push(AdminUserResponse {
                id: user.id,
                firebase_uid: user.firebase_uid,
                email: user.email,
                display_name: user.display_name,
                photo_url: user.photo_url,
                provider: user.provider,
                is_admin: user.is_admin,
                banned_at: user.banned_at,
                banned_reason: user.banned_reason,
                created_at: user.created_at,
                last_login_at: user.last_login_at,
                village_count,
            });
        }

        Ok(responses)
    }

    /// Search users
    pub async fn search_users(
        pool: &PgPool,
        query: &str,
    ) -> AppResult<Vec<AdminUserResponse>> {
        let users = AdminRepository::search_users(pool, query, 50).await?;

        let mut responses = Vec::new();
        for user in users {
            let village_count = AdminRepository::count_user_villages(pool, user.id).await?;
            responses.push(AdminUserResponse {
                id: user.id,
                firebase_uid: user.firebase_uid,
                email: user.email,
                display_name: user.display_name,
                photo_url: user.photo_url,
                provider: user.provider,
                is_admin: user.is_admin,
                banned_at: user.banned_at,
                banned_reason: user.banned_reason,
                created_at: user.created_at,
                last_login_at: user.last_login_at,
                village_count,
            });
        }

        Ok(responses)
    }

    /// Get player detail (user + villages + heroes)
    pub async fn get_player_detail(
        pool: &PgPool,
        user_id: Uuid,
    ) -> AppResult<PlayerDetailResponse> {
        let user = AdminRepository::get_user_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let village_count = AdminRepository::count_user_villages(pool, user.id).await?;

        let user_response = AdminUserResponse {
            id: user.id,
            firebase_uid: user.firebase_uid,
            email: user.email,
            display_name: user.display_name,
            photo_url: user.photo_url,
            provider: user.provider,
            is_admin: user.is_admin,
            banned_at: user.banned_at,
            banned_reason: user.banned_reason,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
            village_count,
        };

        // Get villages
        let villages = VillageRepository::find_by_user_id(pool, user_id).await?;
        let village_responses: Vec<AdminVillageResponse> = villages
            .into_iter()
            .map(|v| AdminVillageResponse {
                id: v.id,
                name: v.name,
                x: v.x,
                y: v.y,
                is_capital: v.is_capital,
                wood: v.wood,
                clay: v.clay,
                iron: v.iron,
                crop: v.crop,
                population: v.population,
            })
            .collect();

        // Get heroes
        let heroes = HeroRepository::get_user_heroes(pool, user_id).await?;
        let hero_responses: Vec<AdminHeroResponse> = heroes
            .into_iter()
            .map(|h| AdminHeroResponse {
                id: h.id,
                name: h.name,
                level: h.level,
                health: h.health,
                status: format!("{:?}", h.status),
            })
            .collect();

        // Get alliance info
        let alliance = Self::get_user_alliance(pool, user_id).await?;

        Ok(PlayerDetailResponse {
            user: user_response,
            villages: village_responses,
            heroes: hero_responses,
            alliance,
        })
    }

    /// Get user's alliance info
    async fn get_user_alliance(
        pool: &PgPool,
        user_id: Uuid,
    ) -> AppResult<Option<AdminAllianceInfoResponse>> {
        let result: Option<(Uuid, String, String, String)> = sqlx::query_as(
            r#"
            SELECT a.id, a.name, a.tag, am.role::text
            FROM alliances a
            JOIN alliance_members am ON a.id = am.alliance_id
            WHERE am.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|(id, name, tag, role)| AdminAllianceInfoResponse {
            id,
            name,
            tag,
            role,
        }))
    }

    /// Ban a user
    pub async fn ban_user(
        pool: &PgPool,
        admin_id: Uuid,
        user_id: Uuid,
        reason: Option<String>,
    ) -> AppResult<AdminUserResponse> {
        // Check user exists
        let user = AdminRepository::get_user_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        // Can't ban admins
        if user.is_admin {
            return Err(AppError::BadRequest("Cannot ban an admin".into()));
        }

        // Ban user
        let user = AdminRepository::ban_user(pool, user_id, reason.clone()).await?;

        // Log action
        AdminRepository::create_log(
            pool,
            admin_id,
            "ban_user",
            "user",
            Some(user_id),
            Some(serde_json::json!({ "reason": reason })),
        )
        .await?;

        let village_count = AdminRepository::count_user_villages(pool, user_id).await?;

        Ok(AdminUserResponse {
            id: user.id,
            firebase_uid: user.firebase_uid,
            email: user.email,
            display_name: user.display_name,
            photo_url: user.photo_url,
            provider: user.provider,
            is_admin: user.is_admin,
            banned_at: user.banned_at,
            banned_reason: user.banned_reason,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
            village_count,
        })
    }

    /// Unban a user
    pub async fn unban_user(
        pool: &PgPool,
        admin_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<AdminUserResponse> {
        let user = AdminRepository::unban_user(pool, user_id).await?;

        // Log action
        AdminRepository::create_log(
            pool,
            admin_id,
            "unban_user",
            "user",
            Some(user_id),
            None,
        )
        .await?;

        let village_count = AdminRepository::count_user_villages(pool, user_id).await?;

        Ok(AdminUserResponse {
            id: user.id,
            firebase_uid: user.firebase_uid,
            email: user.email,
            display_name: user.display_name,
            photo_url: user.photo_url,
            provider: user.provider,
            is_admin: user.is_admin,
            banned_at: user.banned_at,
            banned_reason: user.banned_reason,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
            village_count,
        })
    }

    /// Set admin status
    pub async fn set_admin(
        pool: &PgPool,
        admin_id: Uuid,
        user_id: Uuid,
        is_admin: bool,
    ) -> AppResult<AdminUserResponse> {
        let user = AdminRepository::set_admin(pool, user_id, is_admin).await?;

        // Log action
        AdminRepository::create_log(
            pool,
            admin_id,
            if is_admin { "grant_admin" } else { "revoke_admin" },
            "user",
            Some(user_id),
            None,
        )
        .await?;

        let village_count = AdminRepository::count_user_villages(pool, user_id).await?;

        Ok(AdminUserResponse {
            id: user.id,
            firebase_uid: user.firebase_uid,
            email: user.email,
            display_name: user.display_name,
            photo_url: user.photo_url,
            provider: user.provider,
            is_admin: user.is_admin,
            banned_at: user.banned_at,
            banned_reason: user.banned_reason,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
            village_count,
        })
    }

    // ==================== Statistics ====================

    /// Get server stats
    pub async fn get_server_stats(pool: &PgPool) -> AppResult<ServerStatsResponse> {
        let total_users = AdminRepository::count_users(pool).await?;
        let active_users_24h = AdminRepository::count_active_users_24h(pool).await?;
        let banned_users = AdminRepository::count_banned_users(pool).await?;
        let total_villages = AdminRepository::count_villages(pool).await?;
        let total_alliances = AdminRepository::count_alliances(pool).await?;
        let total_battles_today = AdminRepository::count_battles_today(pool).await?;

        Ok(ServerStatsResponse {
            total_users,
            active_users_24h,
            banned_users,
            total_villages,
            total_alliances,
            total_battles_today,
        })
    }

    // ==================== Resource Management ====================

    /// Adjust village resources (emergency fix)
    pub async fn adjust_resources(
        pool: &PgPool,
        admin_id: Uuid,
        village_id: Uuid,
        wood: Option<i32>,
        clay: Option<i32>,
        iron: Option<i32>,
        crop: Option<i32>,
        reason: &str,
    ) -> AppResult<()> {
        AdminRepository::adjust_resources(
            pool,
            village_id,
            wood.unwrap_or(0),
            clay.unwrap_or(0),
            iron.unwrap_or(0),
            crop.unwrap_or(0),
        )
        .await?;

        // Log action
        AdminRepository::create_log(
            pool,
            admin_id,
            "adjust_resources",
            "village",
            Some(village_id),
            Some(serde_json::json!({
                "wood": wood,
                "clay": clay,
                "iron": iron,
                "crop": crop,
                "reason": reason,
            })),
        )
        .await?;

        Ok(())
    }
}

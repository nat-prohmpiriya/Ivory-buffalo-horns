use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::alliance::{
    Alliance, AllianceDiplomacy, AllianceInvitation, AllianceListItem, AllianceMember,
    AllianceMemberResponse, AllianceRole, DiplomacyStatus, InvitationStatus,
};

pub struct AllianceRepository;

impl AllianceRepository {
    // ==================== Alliance CRUD ====================

    pub async fn create(
        pool: &PgPool,
        name: &str,
        tag: &str,
        description: Option<&str>,
        founder_id: Uuid,
    ) -> AppResult<Alliance> {
        let alliance = sqlx::query_as::<_, Alliance>(
            r#"
            INSERT INTO alliances (name, tag, description, founder_id, leader_id)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id, name, tag, description, founder_id, leader_id, max_members, created_at, updated_at
            "#,
        )
        .bind(name)
        .bind(tag)
        .bind(description)
        .bind(founder_id)
        .fetch_one(pool)
        .await?;

        Ok(alliance)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Alliance>> {
        let alliance = sqlx::query_as::<_, Alliance>(
            r#"
            SELECT id, name, tag, description, founder_id, leader_id, max_members, created_at, updated_at
            FROM alliances
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(alliance)
    }

    pub async fn find_by_tag(pool: &PgPool, tag: &str) -> AppResult<Option<Alliance>> {
        let alliance = sqlx::query_as::<_, Alliance>(
            r#"
            SELECT id, name, tag, description, founder_id, leader_id, max_members, created_at, updated_at
            FROM alliances
            WHERE tag = $1
            "#,
        )
        .bind(tag)
        .fetch_optional(pool)
        .await?;

        Ok(alliance)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
    ) -> AppResult<Alliance> {
        let alliance = sqlx::query_as::<_, Alliance>(
            r#"
            UPDATE alliances
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, tag, description, founder_id, leader_id, max_members, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .fetch_one(pool)
        .await?;

        Ok(alliance)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM alliances WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn list_all(pool: &PgPool, limit: i32, offset: i32) -> AppResult<Vec<AllianceListItem>> {
        let alliances = sqlx::query_as::<_, AllianceListItem>(
            r#"
            SELECT
                a.id,
                a.name,
                a.tag,
                COUNT(am.id)::INT as member_count,
                COALESCE(SUM(v.population), 0) as total_population
            FROM alliances a
            LEFT JOIN alliance_members am ON a.id = am.alliance_id
            LEFT JOIN villages v ON am.user_id = v.user_id
            GROUP BY a.id, a.name, a.tag
            ORDER BY total_population DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(alliances)
    }

    pub async fn get_member_count(pool: &PgPool, alliance_id: Uuid) -> AppResult<i32> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM alliance_members WHERE alliance_id = $1",
        )
        .bind(alliance_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0 as i32)
    }

    pub async fn transfer_leadership(pool: &PgPool, alliance_id: Uuid, new_leader_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE alliances SET leader_id = $2, updated_at = NOW() WHERE id = $1
            "#,
        )
        .bind(alliance_id)
        .bind(new_leader_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    // ==================== Members ====================

    pub async fn add_member(
        pool: &PgPool,
        alliance_id: Uuid,
        user_id: Uuid,
        role: AllianceRole,
    ) -> AppResult<AllianceMember> {
        let member = sqlx::query_as::<_, AllianceMember>(
            r#"
            INSERT INTO alliance_members (alliance_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING id, alliance_id, user_id, role, joined_at
            "#,
        )
        .bind(alliance_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn remove_member(pool: &PgPool, alliance_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            "DELETE FROM alliance_members WHERE alliance_id = $1 AND user_id = $2",
        )
        .bind(alliance_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_member(pool: &PgPool, alliance_id: Uuid, user_id: Uuid) -> AppResult<Option<AllianceMember>> {
        let member = sqlx::query_as::<_, AllianceMember>(
            r#"
            SELECT id, alliance_id, user_id, role, joined_at
            FROM alliance_members
            WHERE alliance_id = $1 AND user_id = $2
            "#,
        )
        .bind(alliance_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(member)
    }

    pub async fn get_user_alliance(pool: &PgPool, user_id: Uuid) -> AppResult<Option<AllianceMember>> {
        let member = sqlx::query_as::<_, AllianceMember>(
            r#"
            SELECT id, alliance_id, user_id, role, joined_at
            FROM alliance_members
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(member)
    }

    pub async fn list_members(pool: &PgPool, alliance_id: Uuid) -> AppResult<Vec<AllianceMemberResponse>> {
        let members = sqlx::query_as::<_, AllianceMemberResponse>(
            r#"
            SELECT
                am.id,
                am.user_id,
                u.display_name as player_name,
                am.role,
                COUNT(v.id)::INT as villages_count,
                COALESCE(SUM(v.population), 0)::INT as population,
                am.joined_at
            FROM alliance_members am
            JOIN users u ON am.user_id = u.id
            LEFT JOIN villages v ON am.user_id = v.user_id
            WHERE am.alliance_id = $1
            GROUP BY am.id, am.user_id, u.display_name, am.role, am.joined_at
            ORDER BY am.role, population DESC
            "#,
        )
        .bind(alliance_id)
        .fetch_all(pool)
        .await?;

        Ok(members)
    }

    pub async fn update_member_role(
        pool: &PgPool,
        alliance_id: Uuid,
        user_id: Uuid,
        role: AllianceRole,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE alliance_members SET role = $3 WHERE alliance_id = $1 AND user_id = $2",
        )
        .bind(alliance_id)
        .bind(user_id)
        .bind(role)
        .execute(pool)
        .await?;

        Ok(())
    }

    // ==================== Invitations ====================

    pub async fn create_invitation(
        pool: &PgPool,
        alliance_id: Uuid,
        inviter_id: Uuid,
        invitee_id: Uuid,
        message: Option<&str>,
    ) -> AppResult<AllianceInvitation> {
        let invitation = sqlx::query_as::<_, AllianceInvitation>(
            r#"
            INSERT INTO alliance_invitations (alliance_id, inviter_id, invitee_id, message)
            VALUES ($1, $2, $3, $4)
            RETURNING id, alliance_id, inviter_id, invitee_id, status, message, created_at, expires_at, responded_at
            "#,
        )
        .bind(alliance_id)
        .bind(inviter_id)
        .bind(invitee_id)
        .bind(message)
        .fetch_one(pool)
        .await?;

        Ok(invitation)
    }

    pub async fn get_invitation(pool: &PgPool, id: Uuid) -> AppResult<Option<AllianceInvitation>> {
        let invitation = sqlx::query_as::<_, AllianceInvitation>(
            r#"
            SELECT id, alliance_id, inviter_id, invitee_id, status, message, created_at, expires_at, responded_at
            FROM alliance_invitations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(invitation)
    }

    pub async fn get_pending_invitations_for_user(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<AllianceInvitation>> {
        let invitations = sqlx::query_as::<_, AllianceInvitation>(
            r#"
            SELECT id, alliance_id, inviter_id, invitee_id, status, message, created_at, expires_at, responded_at
            FROM alliance_invitations
            WHERE invitee_id = $1 AND status = 'pending' AND expires_at > NOW()
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(invitations)
    }

    pub async fn update_invitation_status(
        pool: &PgPool,
        id: Uuid,
        status: InvitationStatus,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE alliance_invitations
            SET status = $2, responded_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(status)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn has_pending_invitation(
        pool: &PgPool,
        alliance_id: Uuid,
        invitee_id: Uuid,
    ) -> AppResult<bool> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM alliance_invitations
            WHERE alliance_id = $1 AND invitee_id = $2 AND status = 'pending' AND expires_at > NOW()
            "#,
        )
        .bind(alliance_id)
        .bind(invitee_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0 > 0)
    }

    // ==================== Diplomacy ====================

    pub async fn set_diplomacy(
        pool: &PgPool,
        alliance_id: Uuid,
        target_alliance_id: Uuid,
        status: DiplomacyStatus,
        proposed_by: Uuid,
    ) -> AppResult<AllianceDiplomacy> {
        let diplomacy = sqlx::query_as::<_, AllianceDiplomacy>(
            r#"
            INSERT INTO alliance_diplomacy (alliance_id, target_alliance_id, status, proposed_by)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (alliance_id, target_alliance_id) DO UPDATE
            SET status = $3, proposed_by = $4, updated_at = NOW()
            RETURNING id, alliance_id, target_alliance_id, status, proposed_by, created_at, updated_at
            "#,
        )
        .bind(alliance_id)
        .bind(target_alliance_id)
        .bind(status)
        .bind(proposed_by)
        .fetch_one(pool)
        .await?;

        Ok(diplomacy)
    }

    pub async fn get_diplomacy(
        pool: &PgPool,
        alliance_id: Uuid,
        target_alliance_id: Uuid,
    ) -> AppResult<Option<AllianceDiplomacy>> {
        let diplomacy = sqlx::query_as::<_, AllianceDiplomacy>(
            r#"
            SELECT id, alliance_id, target_alliance_id, status, proposed_by, created_at, updated_at
            FROM alliance_diplomacy
            WHERE alliance_id = $1 AND target_alliance_id = $2
            "#,
        )
        .bind(alliance_id)
        .bind(target_alliance_id)
        .fetch_optional(pool)
        .await?;

        Ok(diplomacy)
    }

    pub async fn list_diplomacy(pool: &PgPool, alliance_id: Uuid) -> AppResult<Vec<AllianceDiplomacy>> {
        let diplomacy = sqlx::query_as::<_, AllianceDiplomacy>(
            r#"
            SELECT id, alliance_id, target_alliance_id, status, proposed_by, created_at, updated_at
            FROM alliance_diplomacy
            WHERE alliance_id = $1
            ORDER BY status, updated_at DESC
            "#,
        )
        .bind(alliance_id)
        .fetch_all(pool)
        .await?;

        Ok(diplomacy)
    }

    pub async fn remove_diplomacy(pool: &PgPool, alliance_id: Uuid, target_alliance_id: Uuid) -> AppResult<()> {
        sqlx::query(
            "DELETE FROM alliance_diplomacy WHERE alliance_id = $1 AND target_alliance_id = $2",
        )
        .bind(alliance_id)
        .bind(target_alliance_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

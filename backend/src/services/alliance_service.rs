use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::alliance::{
    Alliance, AllianceDiplomacy, AllianceInvitation, AllianceListItem, AllianceMemberResponse,
    AllianceResponse, AllianceRole, CreateAllianceRequest, DiplomacyStatus, InvitationStatus,
};
use crate::repositories::alliance_repo::AllianceRepository;

pub struct AllianceService;

impl AllianceService {
    // ==================== Alliance Management ====================

    /// Create a new alliance
    pub async fn create_alliance(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateAllianceRequest,
    ) -> AppResult<AllianceResponse> {
        // Validate tag length (2-4 characters)
        if request.tag.len() < 2 || request.tag.len() > 4 {
            return Err(AppError::BadRequest("Tag must be 2-4 characters".into()));
        }

        // Validate name length
        if request.name.len() < 3 || request.name.len() > 50 {
            return Err(AppError::BadRequest("Name must be 3-50 characters".into()));
        }

        // Check if user is already in an alliance
        if let Some(_) = AllianceRepository::get_user_alliance(pool, user_id).await? {
            return Err(AppError::BadRequest("You are already in an alliance".into()));
        }

        // Check if tag is already taken
        if let Some(_) = AllianceRepository::find_by_tag(pool, &request.tag.to_uppercase()).await? {
            return Err(AppError::BadRequest("This tag is already taken".into()));
        }

        // Create the alliance
        let alliance = AllianceRepository::create(
            pool,
            &request.name,
            &request.tag.to_uppercase(),
            request.description.as_deref(),
            user_id,
        )
        .await?;

        // Add founder as leader
        AllianceRepository::add_member(pool, alliance.id, user_id, AllianceRole::Leader).await?;

        let mut response: AllianceResponse = alliance.into();
        response.member_count = 1;

        Ok(response)
    }

    /// Get alliance details
    pub async fn get_alliance(pool: &PgPool, alliance_id: Uuid) -> AppResult<AllianceResponse> {
        let alliance = AllianceRepository::find_by_id(pool, alliance_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Alliance not found".into()))?;

        let member_count = AllianceRepository::get_member_count(pool, alliance_id).await?;

        let mut response: AllianceResponse = alliance.into();
        response.member_count = member_count;

        Ok(response)
    }

    /// Update alliance (leader/officers only)
    pub async fn update_alliance(
        pool: &PgPool,
        user_id: Uuid,
        alliance_id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> AppResult<AllianceResponse> {
        // Check permission
        Self::check_permission(pool, alliance_id, user_id, &[AllianceRole::Leader, AllianceRole::Officer]).await?;

        let alliance = AllianceRepository::update(
            pool,
            alliance_id,
            name.as_deref(),
            description.as_deref(),
        )
        .await?;

        let member_count = AllianceRepository::get_member_count(pool, alliance_id).await?;
        let mut response: AllianceResponse = alliance.into();
        response.member_count = member_count;

        Ok(response)
    }

    /// Disband alliance (leader only)
    pub async fn disband_alliance(pool: &PgPool, user_id: Uuid, alliance_id: Uuid) -> AppResult<()> {
        Self::check_permission(pool, alliance_id, user_id, &[AllianceRole::Leader]).await?;
        AllianceRepository::delete(pool, alliance_id).await?;
        Ok(())
    }

    /// List all alliances
    pub async fn list_alliances(pool: &PgPool, limit: i32, offset: i32) -> AppResult<Vec<AllianceListItem>> {
        AllianceRepository::list_all(pool, limit, offset).await
    }

    /// Get user's alliance
    pub async fn get_my_alliance(pool: &PgPool, user_id: Uuid) -> AppResult<Option<AllianceResponse>> {
        let member = AllianceRepository::get_user_alliance(pool, user_id).await?;

        match member {
            Some(m) => {
                let response = Self::get_alliance(pool, m.alliance_id).await?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    // ==================== Member Management ====================

    /// List alliance members
    pub async fn list_members(pool: &PgPool, alliance_id: Uuid) -> AppResult<Vec<AllianceMemberResponse>> {
        AllianceRepository::list_members(pool, alliance_id).await
    }

    /// Invite player to alliance
    pub async fn invite_player(
        pool: &PgPool,
        inviter_id: Uuid,
        alliance_id: Uuid,
        invitee_id: Uuid,
        message: Option<String>,
    ) -> AppResult<AllianceInvitation> {
        // Check permission (leader or officer)
        Self::check_permission(pool, alliance_id, inviter_id, &[AllianceRole::Leader, AllianceRole::Officer]).await?;

        // Check if invitee is already in an alliance
        if let Some(_) = AllianceRepository::get_user_alliance(pool, invitee_id).await? {
            return Err(AppError::BadRequest("Player is already in an alliance".into()));
        }

        // Check if there's already a pending invitation
        if AllianceRepository::has_pending_invitation(pool, alliance_id, invitee_id).await? {
            return Err(AppError::BadRequest("Player already has a pending invitation".into()));
        }

        // Check member limit
        let alliance = AllianceRepository::find_by_id(pool, alliance_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Alliance not found".into()))?;

        let member_count = AllianceRepository::get_member_count(pool, alliance_id).await?;
        if member_count >= alliance.max_members {
            return Err(AppError::BadRequest("Alliance is full".into()));
        }

        AllianceRepository::create_invitation(pool, alliance_id, inviter_id, invitee_id, message.as_deref()).await
    }

    /// Accept or reject invitation
    pub async fn respond_invitation(
        pool: &PgPool,
        user_id: Uuid,
        invitation_id: Uuid,
        accept: bool,
    ) -> AppResult<()> {
        let invitation = AllianceRepository::get_invitation(pool, invitation_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".into()))?;

        // Verify this invitation is for this user
        if invitation.invitee_id != user_id {
            return Err(AppError::Forbidden("This invitation is not for you".into()));
        }

        // Check if invitation is still pending
        if invitation.status != InvitationStatus::Pending {
            return Err(AppError::BadRequest("Invitation has already been responded to".into()));
        }

        // Check if expired
        if invitation.expires_at < chrono::Utc::now() {
            AllianceRepository::update_invitation_status(pool, invitation_id, InvitationStatus::Expired).await?;
            return Err(AppError::BadRequest("Invitation has expired".into()));
        }

        if accept {
            // Check if user is already in an alliance
            if let Some(_) = AllianceRepository::get_user_alliance(pool, user_id).await? {
                return Err(AppError::BadRequest("You are already in an alliance".into()));
            }

            // Add to alliance
            AllianceRepository::add_member(pool, invitation.alliance_id, user_id, AllianceRole::Member).await?;
            AllianceRepository::update_invitation_status(pool, invitation_id, InvitationStatus::Accepted).await?;
        } else {
            AllianceRepository::update_invitation_status(pool, invitation_id, InvitationStatus::Rejected).await?;
        }

        Ok(())
    }

    /// Get pending invitations for user
    pub async fn get_pending_invitations(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<AllianceInvitation>> {
        AllianceRepository::get_pending_invitations_for_user(pool, user_id).await
    }

    /// Leave alliance
    pub async fn leave_alliance(pool: &PgPool, user_id: Uuid) -> AppResult<()> {
        let member = AllianceRepository::get_user_alliance(pool, user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("You are not in an alliance".into()))?;

        // Leader cannot leave, must transfer leadership first
        if member.role == AllianceRole::Leader {
            return Err(AppError::BadRequest(
                "Leader cannot leave. Transfer leadership first or disband the alliance.".into(),
            ));
        }

        AllianceRepository::remove_member(pool, member.alliance_id, user_id).await?;

        Ok(())
    }

    /// Kick member from alliance
    pub async fn kick_member(pool: &PgPool, user_id: Uuid, target_user_id: Uuid) -> AppResult<()> {
        let kicker = AllianceRepository::get_user_alliance(pool, user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("You are not in an alliance".into()))?;

        let target = AllianceRepository::get_member(pool, kicker.alliance_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found".into()))?;

        // Permission check: Leader can kick anyone, Officer can kick members
        match kicker.role {
            AllianceRole::Leader => {
                if target.role == AllianceRole::Leader {
                    return Err(AppError::BadRequest("Cannot kick yourself".into()));
                }
            }
            AllianceRole::Officer => {
                if target.role != AllianceRole::Member {
                    return Err(AppError::Forbidden("Officers can only kick members".into()));
                }
            }
            AllianceRole::Member => {
                return Err(AppError::Forbidden("You don't have permission to kick members".into()));
            }
        }

        AllianceRepository::remove_member(pool, kicker.alliance_id, target_user_id).await?;

        Ok(())
    }

    /// Update member role
    pub async fn update_member_role(
        pool: &PgPool,
        user_id: Uuid,
        target_user_id: Uuid,
        new_role: AllianceRole,
    ) -> AppResult<()> {
        let actor = AllianceRepository::get_user_alliance(pool, user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("You are not in an alliance".into()))?;

        // Only leader can change roles
        if actor.role != AllianceRole::Leader {
            return Err(AppError::Forbidden("Only the leader can change roles".into()));
        }

        let target = AllianceRepository::get_member(pool, actor.alliance_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found".into()))?;

        // If promoting to leader, transfer leadership
        if new_role == AllianceRole::Leader {
            AllianceRepository::transfer_leadership(pool, actor.alliance_id, target_user_id).await?;
            AllianceRepository::update_member_role(pool, actor.alliance_id, user_id, AllianceRole::Officer).await?;
        }

        AllianceRepository::update_member_role(pool, actor.alliance_id, target_user_id, new_role).await?;

        Ok(())
    }

    // ==================== Diplomacy ====================

    /// Set diplomacy with another alliance
    pub async fn set_diplomacy(
        pool: &PgPool,
        user_id: Uuid,
        target_alliance_id: Uuid,
        status: DiplomacyStatus,
    ) -> AppResult<AllianceDiplomacy> {
        let member = AllianceRepository::get_user_alliance(pool, user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("You are not in an alliance".into()))?;

        // Only leader can set diplomacy
        if member.role != AllianceRole::Leader {
            return Err(AppError::Forbidden("Only the leader can set diplomacy".into()));
        }

        // Cannot set diplomacy with own alliance
        if member.alliance_id == target_alliance_id {
            return Err(AppError::BadRequest("Cannot set diplomacy with your own alliance".into()));
        }

        // Check target alliance exists
        if AllianceRepository::find_by_id(pool, target_alliance_id).await?.is_none() {
            return Err(AppError::NotFound("Target alliance not found".into()));
        }

        AllianceRepository::set_diplomacy(pool, member.alliance_id, target_alliance_id, status, user_id).await
    }

    /// List diplomacy relations
    pub async fn list_diplomacy(pool: &PgPool, alliance_id: Uuid) -> AppResult<Vec<AllianceDiplomacy>> {
        AllianceRepository::list_diplomacy(pool, alliance_id).await
    }

    // ==================== Helpers ====================

    async fn check_permission(
        pool: &PgPool,
        alliance_id: Uuid,
        user_id: Uuid,
        allowed_roles: &[AllianceRole],
    ) -> AppResult<()> {
        let member = AllianceRepository::get_member(pool, alliance_id, user_id)
            .await?
            .ok_or_else(|| AppError::Forbidden("You are not a member of this alliance".into()))?;

        if !allowed_roles.contains(&member.role) {
            return Err(AppError::Forbidden("You don't have permission for this action".into()));
        }

        Ok(())
    }
}

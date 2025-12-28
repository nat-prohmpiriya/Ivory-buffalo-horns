use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::ranking::{
    AllianceRanking, HeroRanking, PlayerAttackRanking, PlayerDefenseRanking,
    PlayerPopulationRanking,
};

pub struct RankingRepository;

impl RankingRepository {
    // ==================== Player Population Ranking ====================

    /// Get players ranked by total population
    pub async fn get_population_ranking(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<PlayerPopulationRanking>> {
        let rankings = sqlx::query_as::<_, PlayerPopulationRanking>(
            r#"
            WITH player_stats AS (
                SELECT
                    u.id as user_id,
                    u.display_name,
                    COALESCE(SUM(v.population), 0) as population,
                    COUNT(v.id) as village_count
                FROM users u
                LEFT JOIN villages v ON u.id = v.user_id
                WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
                GROUP BY u.id, u.display_name
                HAVING COALESCE(SUM(v.population), 0) > 0
            ),
            ranked AS (
                SELECT
                    ps.*,
                    a.tag as alliance_tag,
                    ROW_NUMBER() OVER (ORDER BY ps.population DESC, ps.village_count DESC) as rank
                FROM player_stats ps
                LEFT JOIN alliance_members am ON ps.user_id = am.user_id
                LEFT JOIN alliances a ON am.alliance_id = a.id
            )
            SELECT rank, user_id, display_name, alliance_tag, population, village_count
            FROM ranked
            ORDER BY rank
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rankings)
    }

    /// Get total count for population ranking
    pub async fn count_population_ranking(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT u.id)
            FROM users u
            JOIN villages v ON u.id = v.user_id
            WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    // ==================== Player Attack Ranking ====================

    /// Get players ranked by attack points (troops killed as attacker)
    pub async fn get_attack_ranking(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<PlayerAttackRanking>> {
        // Calculate attack points from battle reports
        // Attack points = sum of all troops killed (from defender_losses JSONB)
        let rankings = sqlx::query_as::<_, PlayerAttackRanking>(
            r#"
            WITH attack_stats AS (
                SELECT
                    br.attacker_player_id as user_id,
                    COUNT(*) FILTER (WHERE br.winner = 'attacker') as battles_won,
                    SUM(
                        (SELECT COALESCE(SUM((value::text)::int), 0)
                         FROM jsonb_each(COALESCE(br.defender_losses, '{}'::jsonb)))
                    ) as attack_points
                FROM battle_reports br
                WHERE br.attacker_player_id IS NOT NULL
                GROUP BY br.attacker_player_id
                HAVING SUM(
                    (SELECT COALESCE(SUM((value::text)::int), 0)
                     FROM jsonb_each(COALESCE(br.defender_losses, '{}'::jsonb)))
                ) > 0
            ),
            ranked AS (
                SELECT
                    u.id as user_id,
                    u.display_name,
                    a.tag as alliance_tag,
                    COALESCE(s.attack_points, 0) as attack_points,
                    COALESCE(s.battles_won, 0) as battles_won,
                    ROW_NUMBER() OVER (ORDER BY COALESCE(s.attack_points, 0) DESC, COALESCE(s.battles_won, 0) DESC) as rank
                FROM attack_stats s
                JOIN users u ON s.user_id = u.id
                LEFT JOIN alliance_members am ON u.id = am.user_id
                LEFT JOIN alliances a ON am.alliance_id = a.id
                WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
            )
            SELECT rank, user_id, display_name, alliance_tag, attack_points, battles_won
            FROM ranked
            ORDER BY rank
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rankings)
    }

    /// Get total count for attack ranking
    pub async fn count_attack_ranking(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT attacker_player_id)
            FROM battle_reports
            WHERE attacker_player_id IS NOT NULL
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    // ==================== Player Defense Ranking ====================

    /// Get players ranked by defense points (troops killed as defender)
    pub async fn get_defense_ranking(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<PlayerDefenseRanking>> {
        let rankings = sqlx::query_as::<_, PlayerDefenseRanking>(
            r#"
            WITH defense_stats AS (
                SELECT
                    br.defender_player_id as user_id,
                    COUNT(*) FILTER (WHERE br.winner = 'defender') as battles_defended,
                    SUM(
                        (SELECT COALESCE(SUM((value::text)::int), 0)
                         FROM jsonb_each(COALESCE(br.attacker_losses, '{}'::jsonb)))
                    ) as defense_points
                FROM battle_reports br
                WHERE br.defender_player_id IS NOT NULL
                GROUP BY br.defender_player_id
                HAVING SUM(
                    (SELECT COALESCE(SUM((value::text)::int), 0)
                     FROM jsonb_each(COALESCE(br.attacker_losses, '{}'::jsonb)))
                ) > 0
            ),
            ranked AS (
                SELECT
                    u.id as user_id,
                    u.display_name,
                    a.tag as alliance_tag,
                    COALESCE(s.defense_points, 0) as defense_points,
                    COALESCE(s.battles_defended, 0) as battles_defended,
                    ROW_NUMBER() OVER (ORDER BY COALESCE(s.defense_points, 0) DESC, COALESCE(s.battles_defended, 0) DESC) as rank
                FROM defense_stats s
                JOIN users u ON s.user_id = u.id
                LEFT JOIN alliance_members am ON u.id = am.user_id
                LEFT JOIN alliances a ON am.alliance_id = a.id
                WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
            )
            SELECT rank, user_id, display_name, alliance_tag, defense_points, battles_defended
            FROM ranked
            ORDER BY rank
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rankings)
    }

    /// Get total count for defense ranking
    pub async fn count_defense_ranking(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT defender_player_id)
            FROM battle_reports
            WHERE defender_player_id IS NOT NULL
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    // ==================== Hero Ranking ====================

    /// Get heroes ranked by level
    pub async fn get_hero_ranking(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<HeroRanking>> {
        let rankings = sqlx::query_as::<_, HeroRanking>(
            r#"
            WITH ranked AS (
                SELECT
                    h.id as hero_id,
                    h.name as hero_name,
                    h.user_id as owner_id,
                    u.display_name as owner_name,
                    h.level,
                    h.experience,
                    ROW_NUMBER() OVER (ORDER BY h.level DESC, h.experience DESC) as rank
                FROM heroes h
                JOIN users u ON h.user_id = u.id
                WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
                  AND h.status != 'dead'
            )
            SELECT rank, hero_id, hero_name, owner_id, owner_name, level, experience
            FROM ranked
            ORDER BY rank
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rankings)
    }

    /// Get total count for hero ranking
    pub async fn count_hero_ranking(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM heroes h
            JOIN users u ON h.user_id = u.id
            WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
              AND h.status != 'dead'
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    // ==================== Alliance Ranking ====================

    /// Get alliances ranked by total population
    pub async fn get_alliance_ranking(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<AllianceRanking>> {
        let rankings = sqlx::query_as::<_, AllianceRanking>(
            r#"
            WITH alliance_stats AS (
                SELECT
                    a.id as alliance_id,
                    a.name,
                    a.tag,
                    COUNT(DISTINCT am.user_id) as member_count,
                    COALESCE(SUM(v.population), 0) as total_population
                FROM alliances a
                LEFT JOIN alliance_members am ON a.id = am.alliance_id
                LEFT JOIN villages v ON am.user_id = v.user_id
                GROUP BY a.id, a.name, a.tag
            ),
            ranked AS (
                SELECT
                    alliance_id,
                    name,
                    tag,
                    member_count,
                    total_population,
                    ROW_NUMBER() OVER (ORDER BY total_population DESC, member_count DESC) as rank
                FROM alliance_stats
            )
            SELECT rank, alliance_id, name, tag, member_count, total_population
            FROM ranked
            ORDER BY rank
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rankings)
    }

    /// Get total count for alliance ranking
    pub async fn count_alliance_ranking(pool: &PgPool) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM alliances")
            .fetch_one(pool)
            .await?;

        Ok(count.0)
    }

    // ==================== Player Position ====================

    /// Get a specific player's rank by population
    pub async fn get_player_population_rank(pool: &PgPool, user_id: Uuid) -> AppResult<Option<i64>> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            WITH player_stats AS (
                SELECT
                    u.id as user_id,
                    COALESCE(SUM(v.population), 0) as population
                FROM users u
                LEFT JOIN villages v ON u.id = v.user_id
                WHERE u.deleted_at IS NULL AND u.banned_at IS NULL
                GROUP BY u.id
                HAVING COALESCE(SUM(v.population), 0) > 0
            ),
            ranked AS (
                SELECT
                    user_id,
                    ROW_NUMBER() OVER (ORDER BY population DESC) as rank
                FROM player_stats
            )
            SELECT rank FROM ranked WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.0))
    }
}

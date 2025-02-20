use shared::domain::meta::{
    Affiliation, AffiliationId, AgeRange, AgeRangeId, AnimationStyle, AnimationStyleId, Goal,
    GoalId, ImageStyle, ImageStyleId, ImageTag, ImageTagIndex, MetaKind, Subject, SubjectId,
};
use shared::media::MediaGroupKind;
use sqlx::{postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

pub async fn get_image_styles(db: &PgPool) -> sqlx::Result<Vec<ImageStyle>> {
    sqlx::query_as!(
        ImageStyle,
        r#"
with cte as (
    select distinct style_id as id
    from image_style
)
select id as "id: ImageStyleId", display_name, created_at, updated_at
from cte inner join style using (id)
order by index
        "#
    )
    .fetch_all(db)
    .await
}

pub async fn get_animation_styles(db: &PgPool) -> sqlx::Result<Vec<AnimationStyle>> {
    sqlx::query_as!(
        AnimationStyle,
        r#"
with cte as (
    select distinct style_id as id
    from animation_style
)
select id as "id: AnimationStyleId", display_name, created_at, updated_at
from cte inner join style using (id)
order by index
        "#
    )
    .fetch_all(db)
    .await
}

pub async fn get_age_ranges(db: &PgPool) -> sqlx::Result<Vec<AgeRange>> {
    sqlx::query_as!(
        AgeRange,
        r#"
            select id as "id: AgeRangeId", display_name, created_at, updated_at from age_range
            order by index
        "#
    )
    .fetch_all(db)
    .await
}

pub async fn get_affiliations(db: &PgPool) -> sqlx::Result<Vec<Affiliation>> {
    sqlx::query_as!(
        Affiliation,
        r#"
            select id as "id: AffiliationId", display_name, created_at, updated_at from affiliation
            order by index
        "#
    )
    .fetch_all(db)
    .await
}

pub async fn get_subjects(db: &PgPool) -> sqlx::Result<Vec<Subject>> {
    sqlx::query_as!(
        Subject,
        r#"
            select subject_id as "id: SubjectId", display_name, created_at, updated_at from subject
            order by index
        "#
    )
    .fetch_all(db)
    .await
}

pub async fn get_goals(db: &PgPool) -> sqlx::Result<Vec<Goal>> {
    sqlx::query_as!(
        Goal,
        r#"
select id as "id: GoalId", display_name, created_at, updated_at from "goal"
order by index
"#
    )
    .fetch_all(db)
    .await
}

pub async fn get_image_tags(db: &PgPool) -> sqlx::Result<Vec<ImageTag>> {
    sqlx::query_as!(
        ImageTag,
        r#"
        select index as "index: ImageTagIndex", display_name, created_at, updated_at from "image_tag"
        order by index
    "#
    )
    .fetch_all(db)
    .await
}

// attempts to grab a uuid out of a string in the shape:
// Key (<key>)=(<uuid>)<postfix>
fn extract_uuid(s: &str) -> Option<Uuid> {
    // <uuid>)<postfix)
    let s = s.split('(').nth(2)?;
    let s = &s[0..s.find(')')?];
    s.parse().ok()
}

// attempts to grab an i16 out of a string in the shape:
// Key (<key>)=(<integer>)<postfix>
fn extract_index(s: &str) -> Option<i16> {
    // <int>)<postfix)
    let s = s.split('(').nth(2)?;
    i16::from_str_radix(&s[0..s.find(')')?], 10).ok()
}

// "WrapperError isn't a good description."
//
// WARN: unstable. will most likely be updated as
#[allow(clippy::module_name_repetitions)]
pub enum MetaWrapperError {
    Sqlx(sqlx::Error),
    MissingMetadata {
        id: Option<Uuid>,
        kind: MetaKind,
    },
    MissingTag {
        index: Option<i16>,
        media_group_kind: MediaGroupKind,
    },
}

pub fn handle_metadata_err(err: sqlx::Error) -> MetaWrapperError {
    let db_err = match &err {
        sqlx::Error::Database(e) => e.downcast_ref::<PgDatabaseError>(),
        _ => return MetaWrapperError::Sqlx(err),
    };

    let id = db_err.detail().and_then(extract_uuid);

    let kind = match db_err.constraint() {
        Some("image_affiliation_affiliation_id_fkey") => MetaKind::Affiliation,
        Some("image_age_range_age_range_id_fkey") => MetaKind::AgeRange,
        Some("image_style_style_id_fkey") => MetaKind::ImageStyle,
        Some("image_category_category_id_fkey") => MetaKind::Category,
        Some("jig_goal_goal_id_fkey") => MetaKind::Goal,
        Some("image_tag_join_tag_index_fkey") => {
            let index = db_err.detail().and_then(extract_index);
            return MetaWrapperError::MissingTag {
                index,
                media_group_kind: MediaGroupKind::Image,
            };
        }

        _ => return MetaWrapperError::Sqlx(err),
    };

    MetaWrapperError::MissingMetadata { id, kind }
}

mod studies;
mod objects;
mod orgs;
mod lang;

use crate::err::AppError;
use sqlx::{Pool, Postgres};

/* 
async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, AppError> {
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
}
*/

pub async fn create_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // N.B. ALL TO BE REVISED
    
    create_schema_sql(pool).await?;

    //studies::create_contribution_types(pool).await?;
    //studies::create_gender_eligibility_types(pool).await?;
    studies::create_study_identifier_types(pool).await?;
    studies::create_study_feature_categories(pool).await?;
    studies::create_study_feature_types(pool).await?;
    studies::create_study_relationship_types(pool).await?;
    studies::create_study_statuses(pool).await?;
    studies::create_study_types(pool).await?;
    studies::create_iec_level_types(pool).await?;

    studies::create_title_types(pool).await?;
    studies::create_topic_types(pool).await?;
    studies::create_topic_vocabularies(pool).await?;
    studies::create_trial_registries(pool).await?;
    studies::create_time_units(pool).await?;

    //objects::create_dataset_consent_types(pool).await?;
    //objects::create_dataset_deidentification_levels(pool).await?;
    //objects::create_dataset_recordkey_types(pool).await?;
    objects::create_date_types(pool).await?;
    objects::create_description_types(pool).await?;
    objects::create_object_access_types(pool).await?;
    objects::create_object_identifier_types(pool).await?;
    objects::create_object_classes(pool).await?;
    objects::create_object_filter_types(pool).await?;
    objects::create_object_relationship_types(pool).await?;
    objects::create_object_types(pool).await?;
    studies::create_iec_level_types(pool).await?;
    objects::create_resource_types(pool).await?;
    objects::create_size_units(pool).await?;

    orgs::create_org_attribute_types(pool).await?;
    orgs::create_org_classes(pool).await?;
    orgs::create_org_name_qualifier_types(pool).await?;
    orgs::create_org_relationship_types(pool).await?;
    orgs::create_org_types(pool).await?;

    lang::create_language_codes(pool).await?;
    lang::create_language_scripts(pool).await?;

    reset_message_sql(pool).await?;

    Ok(())
}


async fn create_schema_sql(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"SET client_min_messages TO WARNING; 
        create schema if not exists lups;"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}

async fn reset_message_sql(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"SET client_min_messages TO NOTICE;"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}
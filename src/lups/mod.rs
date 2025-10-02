mod create;
mod fill;
mod lang;

use crate::err::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};


async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, AppError> {
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
}


pub async fn create_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    execute_sql(create::create_schema_sql(), pool).await?;

    create::contribution_types(pool).await?;
    create::dataset_consent_types(pool).await?;
    create::dataset_deidentification_levels(pool).await?;
    create::dataset_recordkey_types(pool).await?;
    create::date_types(pool).await?;
    create::description_types(pool).await?;
    create::doi_status_types(pool).await?;
    create::gender_eligibility_types(pool).await?;

    create::object_access_types(pool).await?;
    create::object_classes(pool).await?;
    create::object_filter_types(pool).await?;
    create::object_relationship_types(pool).await?;
    create::object_types(pool).await?;
    create::iec_level_types(pool).await?;
    create::language_usage_types(pool).await?;


    create::org_attribute_types(pool).await?;
    create::org_classes(pool).await?;
    create::org_name_qualifier_types(pool).await?;
    create::org_relationship_types(pool).await?;
    create::org_types(pool).await?;

    create::resource_types(pool).await?;
    create::size_units(pool).await?;

    // create::study_identifier_types(pool).await?;
    create::study_feature_categories(pool).await?;
    create::study_feature_types(pool).await?;
    create::study_relationship_types(pool).await?;
    create::study_statuses(pool).await?;
    create::study_types(pool).await?;
    create::iec_level_types(pool).await?;

    create::title_types(pool).await?;
    create::topic_types(pool).await?;
    create::topic_vocabularies(pool).await?;
    create::trial_registries(pool).await?;

    create::language_codes(pool).await?;
    create::language_scripts(pool).await?;
    create::language_usage_types(pool).await?;
    create::time_units(pool).await?;

    create::reset_message_sql(pool).await?;

    Ok(())
}


pub async fn fill_tables(_pool: &Pool<Postgres>) -> Result<(), AppError> {

    //execute_sql(fill::contribution_types(), pool).await?;
    //execute_sql(fill::dataset_consent_types(), pool).await?;
    //execute_sql(fill::dataset_deidentification_levels(), pool).await?;
    //execute_sql(fill::dataset_recordkey_types(), pool).await?;
    //execute_sql(fill::date_types(), pool).await?;
    //execute_sql(fill::description_types(), pool).await?;
    //execute_sql(fill::doi_status_types(), pool).await?;
    //execute_sql(fill::gender_eligibility_types(), pool).await?;
    //xecute_sql(fill::identifier_types(), pool).await?;
    //execute_sql(fill::iec_level_types(), pool).await?;
    //execute_sql(fill::language_usage_types(), pool).await?;
    //execute_sql(fill::object_access_types(), pool).await?;
    //execute_sql(fill::object_classes(), pool).await?;
    //execute_sql(fill::object_filter_types(), pool).await?;
    //execute_sql(fill::object_relationship_types(), pool).await?;
    //execute_sql(fill::object_types(), pool).await?;
    //execute_sql(fill::org_attribute_types(), pool).await?;
    //execute_sql(fill::org_classes(), pool).await?;
    //execute_sql(fill::org_name_qualifier_types(), pool).await?;
    //execute_sql(fill::org_relationship_types(), pool).await?;
    //execute_sql(fill::org_types(), pool).await?;
    //execute_sql(fill::resource_types(), pool).await?;
    //execute_sql(fill::size_units(), pool).await?;
    /* 
    execute_sql(fill::study_feature_categories(), pool).await?;
    execute_sql(fill::study_feature_types(), pool).await?;
    execute_sql(fill::study_relationship_types(), pool).await?;
    execute_sql(fill::study_statuses(), pool).await?;
    execute_sql(fill::study_types(), pool).await?;
    execute_sql(fill::time_units(), pool).await?;
    execute_sql(fill::title_types(), pool).await?;
    execute_sql(fill::topic_types(), pool).await?;
    execute_sql(fill::topic_vocabularies(), pool).await?;
    execute_sql(fill::trial_registries(), pool).await?;

    execute_sql(lang::language_codes(), pool).await?;
    execute_sql(lang::language_scripts(), pool).await?;
*/
    Ok(())
}

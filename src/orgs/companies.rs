use sqlx::{Pool, Postgres};
use crate::AppError;
use crate::sql::process_companies;
//use log::info;


pub async fn create_companies_table(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = process_companies::get_create_companies_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}


pub async fn correct_bracketed_company_names(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = process_companies::correct_company_brackets_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}


pub async fn remove_company_suffixes(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = process_companies::remove_company_suffixes_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}


pub async fn create_revised_orgs(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = process_companies::create_revised_orgs_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}


pub async fn process_multisite_companies(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = process_companies::process_multisite_companies_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}

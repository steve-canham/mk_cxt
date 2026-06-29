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


pub async fn remove_company_suffices(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = process_companies::remove_company_suffixes_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}

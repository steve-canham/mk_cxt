mod cities;
mod countries;
mod scopes;

use crate::err::AppError;
use sqlx::{Pool, Postgres};
use log::info;


pub async fn create_mdr_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    countries::create_schema_and_set_message_level(pool).await?;
    countries::create_country_names_table(pool).await?;
    countries::transfer_country_names_to_mdr(pool).await?;

    cities::create_city_names_table(pool).await?;
    cities::transfer_city_names_to_mdr(pool).await?;

    scopes::create_scope_tables(pool).await?;

    Ok(())
}

pub async fn create_country_data(pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    countries::add_mdr_names_1(pool).await?;
    countries::add_mdr_names_2(pool).await?; 
    countries::add_mdr_names_3(pool).await?; 
    countries::add_mdr_names_4(pool).await?; 
    countries::add_mdr_names_5(pool).await?;
    countries::update_mdr_names(pool).await?;

    Ok(())
}

pub async fn create_city_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    cities::make_mdr_related_changes_1(pool).await?;
    cities::make_mdr_related_changes_2(pool).await?;
    cities::make_mdr_related_changes_3(pool).await?;

    Ok(())
}

pub async fn create_scope_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    scopes::create_scope_data_1(pool).await?;
    scopes::create_scope_data_2(pool).await?;
    scopes::create_scope_data_3(pool).await?;

    scopes::reset_message_level(pool).await?;

    Ok(())
}



pub async fn create_lang_codes_full_table(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.lang_codes_full;
            create table lups.lang_codes_full
            ( 
                  id                  int PRIMARY KEY GENERATED ALWAYS AS IDENTITY (START WITH 1001 INCREMENT BY 1) 
                , code                varchar
                , name                varchar
                , code_type           varchar
            );
            create index lang_codes_full_code on lups.lang_codes_full(code);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}


pub async fn transfer_lang_codes_to_cxt(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into locs.country_names (code, name, code_type)
        select code, name, code_type
        from ftw_geo.lang_codes cn;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} language code records transferred to lups schema", res.rows_affected());

    Ok(())
}




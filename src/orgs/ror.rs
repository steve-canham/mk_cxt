/*
use sqlx::{Pool, Postgres};
use crate::AppError;
use crate::sql::make_ror_tables;
use log::info;


pub async fn make_ror_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {
    let sql = make_ror_tables::get_sql();
    sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(())
}

pub async fn fill_ror_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into orgs.ror_orgs (id, ror_full_id, ror_name, 
            status, established, location, csubdiv_code, country_code)
            select id, ror_full_id, ror_name, 
            status, established, location, csubdiv_code, country_code
            from ftw_ror.core_data;"#;

    info!("{} ROR organisation records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    // type 1: alias or label that is the ror name, type 2: alias or label that is not the ror name
    // type 3: acronym that is the ror name, type 10  acronym that is not the ror name
    
    let sql = r#"insert into orgs.ror_names (id, name, name_corrected, name_type, 
            lang_code, lang_source, script_code)
            select id, value, value, 
            case 
            when is_ror_name = true and name_type <> 10 then 1
            when is_ror_name = true and name_type = 10 then 3
            when is_ror_name = false and name_type <> 10 then 2
            when is_ror_name = false and name_type = 10 then 10
            end, 
            lang_code, 'ror', script_code
            from ftw_ror.names;"#;

    info!("{} ROR organisation names transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_rels (id, ror_name, rel_type, 
            related_id, related_name)
            select id, ror_name, rel_type, 
            related_id, related_name
            from ftw_ror.relationships;"#;

    info!("{} ROR relationship records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_types(id, ror_name, org_type)
            select id, ror_name, org_type
            from ftw_ror.type;"#;
    
    info!("{} ROR type records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_locs(id, ror_name, 
                 geonames_id, location, lat, lng, cont_code, 
                 cont_name, country_code, country_name, 
                 csubdiv_code, csubdiv_name)
             select id, ror_name, 
                 geonames_id, location, lat, lng, cont_code, 
                 cont_name, country_code, country_name, 
                 csubdiv_code, csubdiv_name
             from ftw_ror.locations;"#;

    info!("{} ROR location records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_countries(id, country_code)
            select distinct id, country_code
            from orgs.ror_locs;"#;

    info!("{} ROR country records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_links(id, ror_name, link_type, link)
            select id, ror_name, link_type, link
            from ftw_ror.links;"#;
    
    info!("{} ROR link records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_ext_ids(id, ror_name, id_type, id_value, is_preferred)
            select id, ror_name, id_type, id_value, is_preferred
            from ftw_ror.external_ids;"#;

    info!("{} ROR external id records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    let sql = r#"insert into orgs.ror_domains(id, ror_name, domain)
            select id, ror_name, domain
            from ftw_ror.domains;"#;

    info!("{} ROR domain records transferred to ror schema", 
          execute_sql(pool, sql).await?);

    Ok(())
}

pub async fn execute_sql(pool: &Pool<Postgres>, sql: &str) -> Result<u64, AppError> {
   let res = sqlx::raw_sql(sql).execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    Ok(res.rows_affected())
}
*/



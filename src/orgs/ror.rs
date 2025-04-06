use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;

pub async fn load_ror_orgs(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists orgs.ror_orgs;
            create table orgs.ror_orgs
    (
          id                varchar     not null primary key
        , ror_full_id       varchar     not null
        , ror_name          varchar     not null	
        , status            int         not null default 1
        , established       int         null
        , location          varchar     null
        , csubdiv_code      varchar     null
        , country_code      varchar     null
    );"#;
    
    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        
    let sql = r#"insert into orgs.ror_orgs (id, ror_full_id, ror_name, 
            status, established, location, csubdiv_code, country_code)
            select id, ror_full_id, ror_name, 
            status, established, location, csubdiv_code, country_code
            from ftw_ror.core_data;"#;
        
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} ROR organisation records transferred to orgs schema", res.rows_affected());

    Ok(())
        
}


pub async fn load_ror_org_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists orgs.ror_names;
            create table orgs.ror_names
    (
          id                varchar     not null
        , name              varchar     not null  
        , name_to_compare   varchar     null  
        , name_type         int         not null 
        , lang_code         varchar     null
        , script_code       varchar     null
    );
    create index names_idx on orgs.ror_names(id);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    let sql = r#"insert into orgs.ror_names (id, name, name_to_compare, name_type, 
            lang_code, script_code)
            select id, value, lower(value), 
            case 
            when is_ror_name = true and name_type <> 10 then 1 
            when is_ror_name = true and name_type = 10 then 3 
            when is_ror_name = false and name_type <> 10 then 2 
            when is_ror_name = false and name_type = 10 then 10
            end, 
            lang_code, script_code
            from ftw_ror.names;"#;
        
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} ROR organisation names transferred to orgs schema", res.rows_affected());

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, '.', '')
    where name_to_compare like '%.%';"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, ',', '')
    where name_to_compare like '%,%';"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, '"', '')
     where name_to_compare like '%"%';"#;
        
    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, ')', '')
    where name_to_compare like '%)%';"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, '(', ' ')
    where name_to_compare like '%(%';"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, '''', '’')
    where name_to_compare like '%''%';"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = replace(name_to_compare, '  ', ' ')
    where name_to_compare like '%  %';"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"update orgs.ror_names 
    set name_to_compare = trim(name_to_compare);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        
    Ok(())
}


pub async fn load_ror_org_rels(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists orgs.ror_rels;
            create table orgs.ror_rels
            (
                  id                varchar     not null
                , ror_name          varchar     not null
                , rel_type          int         not null
                , related_id        varchar     not null
                , related_name      varchar     not null
            );  
            create index relationships_idx on orgs.ror_rels(id);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    let sql = r#"insert into orgs.ror_rels (id, ror_name, rel_type, 
            related_id, related_name)
            select id, ror_name, rel_type, 
            related_id, related_name
            from ftw_ror.relationships;"#;
        
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} ROR relationship records transferred to orgs schema", res.rows_affected());
        
    Ok(())
}


pub async fn load_ror_org_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists orgs.ror_types;
            create table orgs.ror_types
            (
                id                varchar     not null
                , ror_name          varchar     not null
                , org_type          int         not null
            );  
            create index type_idx on orgs.ror_types(id);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    
            let sql = r#"insert into orgs.ror_types(id, ror_name, org_type)
            select id, ror_name, org_type
            from ftw_ror.type;"#;
        
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} ROR type records transferred to orgs schema", res.rows_affected());
        
    Ok(())
}


pub async fn load_ror_org_locs(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists orgs.ror_locs;
            create table orgs.ror_locs
            (
                  id                varchar     not null
                , ror_name          varchar     not null
                , geonames_id       int         null
                , location          varchar     null	
                , lat               real        null
                , lng               real        null
                , cont_code         varchar     null
                , cont_name         varchar     null
                , country_code      varchar     null
                , country_name      varchar     null
                , csubdiv_code      varchar     null  
                , csubdiv_name      varchar     null	
            );
            create index locations_idx on orgs.ror_locs(id);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        
            let sql = r#"insert into orgs.ror_locs(id, ror_name, 
                geonames_id, location, lat, lng, cont_code, 
                cont_name, country_code, country_name, 
                csubdiv_code, csubdiv_name)
            select id, ror_name, 
                geonames_id, location, lat, lng, cont_code, 
                cont_name, country_code, country_name, 
                csubdiv_code, csubdiv_name
            from ftw_ror.locations;"#;
        
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} ROR location records transferred to orgs schema", res.rows_affected());
        
    let sql = r#"drop table if exists orgs.ror_countries;
            create table orgs.ror_countries
            (
                  id                varchar     not null
                , country_code      varchar     null
            );
            create index countries_idx on orgs.ror_countries(id);"#;

    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        
            let sql = r#"insert into orgs.ror_countries(id, country_code)
            select distinct id, country_code
            from orgs.ror_locs;"#;
        
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} ROR country records created", res.rows_affected());

    Ok(())
}


pub async fn add_names_without_thes(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into orgs.ror_names (id, name, name_to_compare, name_type, lang_code, script_code)
        select n.* from 
            (select id, 
                substring(name, 5, length(name) - 4) as name,
                substring(name_to_compare, 5, length(name_to_compare) - 4) as name_to_compare,
                2 as name_type, lang_code, script_code
            from orgs.ror_names
            where name_to_compare like 'the %'
            and array_length(string_to_array(name_to_compare, ' '), 1) > 2) as n
        left join 
        orgs.ror_names r
        on n.id = r.id
        and n.name_to_compare = r.name_to_compare
        where r.id is null;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} additional ROR name records added with initial 'the ' removed", res.rows_affected());
  
    Ok(())
}


pub async fn add_cm_lang_code_to_comm_orgs(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"update orgs.ror_names n
                set lang_code = 'cm'
                from orgs.ror_types t
                where n.id = t.id
                and t.org_type = 400"#;

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} names of commercial organisations given 'cm' language code", res.rows_affected());
  
    Ok(())
}
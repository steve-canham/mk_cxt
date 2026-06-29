use crate::err::AppError;
use crate::setup::config_reader::DB_PARS;
use std::time::Duration;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use sqlx::ConnectOptions;

// Only a single database and database pool is used within the sytem

pub async fn get_db_pool() -> Result<PgPool, AppError> {

    // Use the static DB_PARS objecu to get the connection string
    // Use the string to set up a connection options object and change
    // the time threshold for warnings. Set up a DB pool option and
    // connect using the connection options object.

    let db_pars = DB_PARS.get()
        .ok_or_else(|| AppError::MissingDBParameters())?;
    let db_conn_string = format!("postgres://{}:{}@{}:{}/{}",
        db_pars.db_user, db_pars.db_password, db_pars.db_host, db_pars.db_port, db_pars.cxt_db_name);
    let mut conn_object: PgConnectOptions = db_conn_string.parse()
        .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    conn_object = conn_object.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(conn_object).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_pars.cxt_db_name), e))
}



pub async fn set_up_foreign_tables(pool: &PgPool, data_type: &str) -> Result<(), AppError> {

    let db_pars = DB_PARS.get()
        .ok_or_else(|| AppError::MissingDBParameters())?;

    let source = match data_type {                     // name of source database, and ftw server name
        "locs" => db_pars.locs_db_name.clone(), 
        "orgs" => db_pars.orgs_db_name.clone(),  
        "umls" => db_pars.umls_db_name.clone(),  
        _ => "".to_string(),
    };

    let schema = match data_type {                    // name of source schema
        "locs" => "ppr", 
        "orgs" => "ppr", 
        "umls" => "icd", 
        _ => "",
    };
    
    // Operating in the cxt database. The lups schema can be guaranteed to exist
    // (it holds the look up tables independent of FTW data)
    // so use lups to hold the postgres_fdw if necessary
    // N.B. 'WITH SCHEMA <schema>' required the first time in DB
    
    let sql = "SET client_min_messages TO WARNING; 
            CREATE EXTENSION IF NOT EXISTS postgres_fdw WITH SCHEMA lups;";  
    sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = format!(r#"CREATE SERVER IF NOT EXISTS {source} 
           FOREIGN DATA WRAPPER postgres_fdw
           OPTIONS (host '{}', dbname '{source}', port '{}');"#, db_pars.db_host, db_pars.db_port);
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = format!(r#"CREATE USER MAPPING IF NOT EXISTS FOR CURRENT_USER
           SERVER {source}
           OPTIONS (user '{}', password '{}')"#, db_pars.db_user, db_pars.db_password);
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
                    
    let sql = format!(r#"SET client_min_messages TO WARNING;
           DROP SCHEMA IF EXISTS ftw_{source} cascade;
           CREATE SCHEMA ftw_{source};
           IMPORT FOREIGN SCHEMA {schema}
           FROM SERVER {source}
           INTO ftw_{source};
           SET client_min_messages TO NOTICE;"#);
    sqlx::raw_sql(&sql).execute(pool)
       .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    Ok(())
}


pub async fn drop_foreign_tables(pool: &PgPool, data_type: &str) -> Result<(), AppError> {

    let db_pars = DB_PARS.get()
        .ok_or_else(|| AppError::MissingDBParameters())?;

    let source = match data_type {
        "locs" => db_pars.locs_db_name.clone(),  
        "orgs" => db_pars.orgs_db_name.clone(),  
        "umls" => db_pars.umls_db_name.clone(),  
        _ => "".to_string(),
    };
    
    let sql = format!(r#"SET client_min_messages TO WARNING; 
        DROP SCHEMA IF EXISTS ftw_{source} cascade;
        DROP USER MAPPING IF EXISTS FOR CURRENT_USER SERVER {source}; 
        DROP SERVER IF EXISTS {source};
        SET client_min_messages TO NOTICE;"#);

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}



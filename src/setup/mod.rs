pub mod config_reader;
pub mod log_helper;
pub mod cli_reader;

use crate::err::AppError;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use std::path::PathBuf;
use cli_reader::{CliPars, Flags};
use config_reader::DBPars;
use std::fs;
use std::time::Duration;
use sqlx::ConnectOptions;
use config_reader::Config;
use std::sync::OnceLock;

pub struct InitParams {
    pub log_folder: PathBuf,
    pub flags: Flags,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_params(cli_pars: CliPars, config_string: &String) -> Result<InitParams, AppError> {

    // Called from lib::run as the initial task of the program.
    // Returns a struct that contains the program's parameters.
      
    // Normal import and / or processing and / or outputting
    // If folder name also given in CL args the CL version takes precedence
    
    let config_file: Config = config_reader::populate_config_vars(&config_string)?; 
    let folder_pars = config_file.folders;  

    let log_folder = folder_pars.log_folder_path;  // guaranteed to exist as essential parameter
    if !folder_exists (&log_folder) { 
            fs::create_dir_all(&log_folder)?;
    }
   
    // For execution flags read from the environment variables
    
    Ok(InitParams {
        log_folder,
        flags: cli_pars.flags,
    })

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let xres = folder_name.try_exists();
    let res = match xres {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
        

pub async fn get_cxt_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_cxt_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}


pub fn fetch_db_pars() -> Result<DBPars, AppError> {
    config_reader::fetch_db_pars()
}


pub fn establish_log(params: &InitParams) -> Result<(), AppError> {

    if !log_set_up() {  // can be called more than once in context of integration tests
        log_helper::setup_log(&params.log_folder)?;
        LOG_RUNNING.set(true).unwrap(); // should always work
        log_helper::log_startup_params(&params);
    }
    Ok(())
}

pub fn log_set_up() -> bool {
    match LOG_RUNNING.get() {
        Some(_) => true,
        None => false,
    }
}


pub async fn set_up_foreign_tables(pool: &PgPool, data_type: &str) -> Result<(), AppError> {

    let dbp = config_reader::fetch_db_pars()?;

    let source = match data_type {
        "locs" => dbp.locs_db_name, 
        "orgs" => dbp.orgs_db_name, 
        "umls" => dbp.umls_db_name, 
        "pubs" => dbp.pubs_db_name, 
        _ => "".to_string(),
    };
    
    // Operating in the cxt database. The lup schema can be guaranteed to exist
    // (it holds the look up tables independent of FTW data)
    // so use lup to hold the postgres_fdw if necessary
    
    let sql = "CREATE EXTENSION IF NOT EXISTS postgres_fdw WITH SCHEMA lkup;";  // WITH SCHEMA <schema> required the first time in DB
    sqlx::raw_sql(sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = format!("CREATE SERVER IF NOT EXISTS {} ", source)
    + " FOREIGN DATA WRAPPER postgres_fdw "
    + &format!("OPTIONS (host '{}', dbname '{}', port '{}')", dbp.db_host, source, dbp.db_port);
    sqlx::raw_sql(&sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = "CREATE USER MAPPING IF NOT EXISTS FOR CURRENT_USER".to_string()
    + &format!(" SERVER {} ", source)
    + &format!(" OPTIONS (user '{}', password '{}')", dbp.db_user, dbp.db_password);
    sqlx::raw_sql(&sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
                    
    let sql = format!("DROP SCHEMA IF EXISTS ftw_{} cascade;", source)
    + &format!(" CREATE SCHEMA ftw_{};", source)
    + &format!(" IMPORT FOREIGN SCHEMA {}", source)
    + &format!(" FROM SERVER {}", source)
    + &format!(" INTO ftw_{};", source);
    sqlx::raw_sql(&sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    Ok(())

}


pub async fn drop_foreign_tables(pool: &PgPool, data_type: &str) -> Result<(), AppError> {

    let dbp = config_reader::fetch_db_pars()?;

    let source = match data_type {
        "locs" => dbp.locs_db_name, 
        "orgs" => dbp.orgs_db_name, 
        "umls" => dbp.umls_db_name, 
        "pubs" => dbp.pubs_db_name, 
        _ => "".to_string(),
    };
    
    let sql = format!("DROP SCHEMA IF EXISTS ftw_{} cascade;", source)
        + &format!("DROP USER MAPPING IF EXISTS FOR CURRENT_USER SERVER {} ;", source)
        + &format!("DROP SERVER IF EXISTS {} ;", source);

    sqlx::raw_sql(&sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}




// Tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn check_min_pars_read_correctly() {

        let config = r#"
[folders]
log_folder_path="E:\\MDR source data\\cxt\\logs1"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"

"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        assert_eq!(res.flags.create_lups, true);
        assert_eq!(res.flags.import_locs, false);
        assert_eq!(res.flags.import_orgs, false);
        assert_eq!(res.flags.import_umls, false);
        assert_eq!(res.flags.import_pubs, false);
        assert_eq!(res.log_folder, PathBuf::from("E:\\MDR source data\\cxt\\logs1"));

    }


    #[test]
    fn check_config_vars_read_correctly() {

        let config = r#"
[folders]
log_folder_path="E:\\MDR source data\\cxt\\logs1"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"

cnxt_db_name="cxt"
orgs_db_name="ror"
locs_db_name="geo"
umls_db_name="uml"
pubs_db_name="pub"

"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-c", "-p"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        assert_eq!(res.flags.create_lups, false);
        assert_eq!(res.flags.import_locs, true);
        assert_eq!(res.flags.import_orgs, false);
        assert_eq!(res.flags.import_umls, false);
        assert_eq!(res.flags.import_pubs, true);
        assert_eq!(res.log_folder, PathBuf::from("E:\\MDR source data\\cxt\\logs1"));

    }
   
    
    #[test]
    fn check_all_flags_read() {

        let config = r#"
[folders]
log_folder_path="E:\\MDR source data\\cxt\\logs2"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"

cnxt_db_name="cxt"
orgs_db_name="ror"
locs_db_name="geo"
umls_db_name="uml"
pubs_db_name="pub"

"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();
        
        let args : Vec<&str> = vec!["dummy target", "-c", "-u", "-g", "-p", "-k"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        assert_eq!(res.flags.create_lups, true);
        assert_eq!(res.flags.import_locs, true);
        assert_eq!(res.flags.import_orgs, true);
        assert_eq!(res.flags.import_umls, true);
        assert_eq!(res.flags.import_pubs, true);
        assert_eq!(res.log_folder, PathBuf::from("E:\\MDR source data\\cxt\\logs2"));
    }
}


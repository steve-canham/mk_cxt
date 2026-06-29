pub mod config_reader;
pub mod log_helper;
pub mod cli_reader;
pub mod db_pars;

use crate::err::AppError;
use std::path::PathBuf;
use cli_reader::{Flags};
use std::fs;
use config_reader::Config;
use std::sync::OnceLock;
use directories::ProjectDirs;
use std::ffi::OsString;

pub struct InitParams {
    pub log_folder: PathBuf,
    pub flags: Flags,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn combine_params(args: Vec<OsString>) -> Result<InitParams, AppError>{

    // Called from lib::run as the initial task of the program.
    // Returns a struct that contains the program's parameters.
    
    let cli_pars = cli_reader::fetch_valid_arguments(args)?;
    let config_path = obtain_config_file_path()?;
    let config_string = fs::read_to_string(&config_path)
        .map_err(|e| AppError::IoReadErrorWithPath(e, config_path.to_owned()))?;

    let config_file: Config = config_reader::populate_config_vars(&config_string)?; 
    let folder_pars = config_file.folders;  
    let log_folder = folder_pars.log_folder_path;  // guaranteed to exist as essential parameter
    if !folder_exists (&log_folder) { 
            fs::create_dir_all(&log_folder)?;
    }

    Ok(InitParams {
        log_folder,
        flags: cli_pars.flags,
    })
}

fn obtain_config_file_path() -> Result<PathBuf, AppError> {

     if let Some(config) = ProjectDirs::from("eu", "canhamis", "imp_ror") {
         let config_folder = config.config_dir().to_path_buf();
         let file_name = "config.toml";
         Ok(config_folder.join(file_name))    // Linux:   /home/<user name>/.config/mk_cxt/config.toml
     }   
     else {
         println!("Odd! - Unable to identify an OS-specific location for the configuration file");
         Err(AppError::ConfigurationError(
             "No folder for config file found".to_string(), 
             "Fatal error - unable to proceed".to_string()))
     }
}


fn folder_exists(folder_name: &PathBuf) -> bool {
    match folder_name.try_exists() {
        Ok(true) => true,
        _ => false,   // includes Ok(false) as well as Err
    }
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


// Tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn check_min_pars_read_correctly() {

        let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/cxt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"

cnxt_db_name="cxt"
orgs_db_name="ror"
locs_db_name="geo"
umls_db_name="uml"
"#;
        let config_string = config.to_string();
        let config = config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res =  InitParams {
            log_folder: config.folders.log_folder_path,
            flags: cli_pars.flags,
        };

        assert_eq!(res.log_folder, PathBuf::from("/home/steve/Data/MDR logs/cxt"));
        assert_eq!(res.flags.create_lups, true);
        assert_eq!(res.flags.import_locs, false);
        assert_eq!(res.flags.import_orgs, false);
        assert_eq!(res.flags.import_umls, false);
    }


    #[test]
    fn check_config_vars_read_correctly() {

        let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/cxt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"

cnxt_db_name="cxt"
orgs_db_name="ror"
locs_db_name="geo"
umls_db_name="uml"
"#;
        let config_string = config.to_string();
        let config = config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-c", "-g"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res =  InitParams {
            log_folder: config.folders.log_folder_path,
            flags: cli_pars.flags,
        };

        assert_eq!(res.log_folder, PathBuf::from("/home/steve/Data/MDR logs/cxt"));
        assert_eq!(res.flags.create_lups, false);
        assert_eq!(res.flags.import_locs, true);
        assert_eq!(res.flags.import_orgs, true);
        assert_eq!(res.flags.import_umls, false);
    }
   
    
    #[test]
    fn check_all_flags_read() {

        let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/cxt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"

cnxt_db_name="cxt"
orgs_db_name="ror"
locs_db_name="geo"
umls_db_name="uml"
"#;
        let config_string = config.to_string();
        let config = config_reader::populate_config_vars(&config_string).unwrap();
        
        let args : Vec<&str> = vec!["dummy target", "-c", "-u", "-g", "-k"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res =  InitParams {
            log_folder: config.folders.log_folder_path,
            flags: cli_pars.flags,
        };
        
        assert_eq!(res.log_folder, PathBuf::from("/home/steve/Data/MDR logs/cxt"));
        assert_eq!(res.flags.create_lups, true);
        assert_eq!(res.flags.import_locs, true);
        assert_eq!(res.flags.import_orgs, true);
        assert_eq!(res.flags.import_umls, true);
    }
}


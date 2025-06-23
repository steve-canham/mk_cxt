
pub mod setup;
pub mod err;
mod lkup;
mod locs;
mod orgs;
mod umls;
mod pubs;

use setup::{cli_reader, set_up_foreign_tables, drop_foreign_tables};
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let cli_pars: cli_reader::CliPars;
    cli_pars = cli_reader::fetch_valid_arguments(args)?;
    let flags = cli_pars.flags;

    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
                              
    let params = setup::get_params(cli_pars, &config_string)?;
    setup::establish_log(&params)?;
    let pool = setup::get_cxt_db_pool().await?;

    if flags.create_lups {

        // (re)creates the MDR based lookup tables

        lkup::create_tables(&pool).await?;
        lkup::fill_tables(&pool).await?;
    }
         
    if flags.import_locs {
        
        // Set up the location data within the context database, as a foreign table schema

        set_up_foreign_tables(&pool, "locs").await?;

        // Transfer the data to the relevant context schema

        locs::create_mdr_tables(&pool).await?;
        
        // Further process that data, if and as necessary

        locs::create_city_data(&pool).await?;
        locs::create_country_data(&pool).await?;
        locs::create_scope_data(&pool).await?;
      
        // remove the foreign tables from the context database

        drop_foreign_tables(&pool, "locs").await?;
    }


    if flags.import_orgs {

        // Set up the organisation data within the context database, as a foreign table schema

        set_up_foreign_tables(&pool, "orgs").await?;

        // Transfer the data to the relevant context schema

        orgs::load_ror_data(&pool).await?;

        orgs::process_ror_data(&pool).await?;
        

        // Further process that data, if and as necessary

        // remove the foreign tables from the context database

        drop_foreign_tables(&pool, "orgs").await?;
        
    }


    if flags.import_umls {

        // Set up the terminology data within the context database, as a foreign table schema

        // set_up_foreign_tables(&pool, "umls").await?;

        // Transfer the data to the relevant context schema

        // Further process that data, if and as necessary

        // remove the foreign tables from the context database
        
        //drop_foreign_tables(&pool, "umls").await?;
        
    }


    if flags.import_pubs {

        // Set up the publishing data within the context database, as a foreign table schema

        // set_up_foreign_tables(&pool, "pubs").await?;

        // Transfer the data to the relevant context schema

        // Further process that data, if and as necessary

        // remove the foreign tables from the context database
        
        //drop_foreign_tables(&pool, "pubs").await?;
        
    }

    Ok(())  
}

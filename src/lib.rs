
pub mod setup;
pub mod err;
mod lups;
mod locs;
mod orgs;
mod umls;
mod sql;

use setup::db_pars::{get_db_pool, set_up_foreign_tables, drop_foreign_tables};
use err::AppError;
use std::ffi::OsString;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {
           
    let params = setup::combine_params(args)?;
    setup::establish_log(&params)?;
    let pool = get_db_pool().await?;
    let flags = params.flags;
    
    if flags.create_lups {
        lups::create_tables(&pool).await?;                // (re)creates the MDR based lookup tables in cxt.lup
    }

    if flags.import_locs {

        set_up_foreign_tables(&pool, "locs").await?;      // Set up the location data as a foreign table schema

        locs::create_mdr_tables(&pool).await?;            // Transfer the data to the relevant context schema

        locs::create_city_data(&pool).await?;             // Further process that data, if and as necessary
        locs::create_country_data(&pool).await?;
        locs::create_scope_data(&pool).await?;
        
        locs::create_lang_codes_full_table(&pool).await?;
        locs::transfer_lang_codes_to_cxt(&pool).await?;

        drop_foreign_tables(&pool, "locs").await?;        // remove the foreign tables from the context database
    }


    if flags.import_orgs {

        set_up_foreign_tables(&pool, "orgs").await?;  // Set up the ror data within the context database, as a foreign table schema
        //orgs::load_ror_data(&pool).await?;            // Transfer the data to the relevant context schema

        orgs::process_ror_data(&pool).await?;      // Further process that data, if and as necessary

        drop_foreign_tables(&pool, "orgs").await?;    // remove the foreign tables from the context database
    }


    if flags.import_umls {

        // set_up_foreign_tables(&pool, "uml").await?;    // Set up the terminology data within the context database, as a foreign table schema

        // Transfer the data to the relevant context schema

        // Further process that data, if and as necessary

        //drop_foreign_tables(&pool, "uml").await?;      // remove the foreign tables from the context database
    }

    Ok(())  
}

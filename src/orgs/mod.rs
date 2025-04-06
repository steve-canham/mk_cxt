mod ror;

use crate::err::AppError;
use sqlx::{Pool, Postgres};


// need a process for taking latest version of ror data and using it to update the 
// organisation table, and related: org names, attributes, locations, types, relationships, etc.

// First may need to modify the ror data as obtained rom the src schema in the ror db
// For example modify the company data to make a single entry from multiple national subsidiaries - 
// get companies in a parent child relationship - remove the children but possibly keep a name if it is 
// different as an alt name. Keep the parent entry as 'the' company ROR entry.

// Look at orgs names without a language code
// If the org is a commercial company change the lang code to 'cm' (?) or just leave blank
// If the former easier to see the gaps, though 'cm' needs to be added to the lang codes
// in the language_codes lkup table.

// Add languages if possible, using location of org and key words or word parts
// Do language of acronyms where all other names have the same language
// See what are left

// Also make sure apostrophe's are regularised
// Names beginning with 'the' also need to be modified 
// Only those with three words or less, in total, can keep the 'The', i.e. 'the X Y' is allowed,
// or 'the X', but nothing longer - 'the university of... becomes 'university of...
// so 'The Royal Free' is an allowed alt name, but the actual name is 'Royal Free Hospital'

// 

pub async fn load_ror_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    ror::load_ror_orgs(pool).await?;
    ror::load_ror_org_names(pool).await?;
    ror::load_ror_org_rels(pool).await?;
    ror::load_ror_org_types(pool).await?;

    Ok(())
}


pub async fn process_ror_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    ror::add_names_without_thes(pool).await?;
    


    Ok(())
}
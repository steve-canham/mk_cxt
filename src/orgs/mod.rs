mod ror;
mod english;
mod japanese;
mod chinese;
mod french;

use crate::err::AppError;
use sqlx::{Pool, Postgres};


pub async fn load_ror_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // These simply load the ROR data into matching tables in the orgs schema
    // of the cxt DB. The ror names are loaded in two forms, the original
    // and a 'name_to_compare' form, which is lower-cased, shorn of full stops, 
    // commas and brackets, and has apostrophes replaced by single right quotes.

    ror::load_ror_orgs(pool).await?;
    ror::load_ror_org_names(pool).await?;
    ror::load_ror_org_rels(pool).await?;
    ror::load_ror_org_types(pool).await?;
    ror::load_ror_org_locs(pool).await?;

    Ok(())
}


pub async fn process_ror_data(pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    // Look at orgs names without a language code
    // If the org is a commercial company change the lang code to 'cm' (?) or just leave blank
    // If the former easier to see the gaps, though 'cm' needs to be added to the lang codes
    // in the language_codes lkup table.

    ror::add_cm_lang_code_to_comm_orgs(pool).await?;

    // Add languages if possible, using location of org and key words or word parts
    // Do language of acronyms where all other names have the same language
    // See what are left

    english::update_english_names(pool).await?;
    japanese::update_japanese_names(pool).await?;
    chinese::update_chinese_names(pool).await?;
    french::update_french_names(pool).await?;

    // There are about 1600 names that begin with 'The '
    // These are often prtesented in source material without the 'The '.
    // 400 of them already include a name variant without the 'The., but
    // this call results in the remaining 1200+ also having a 'the-less'
    // version of the name added.

    ror::add_names_without_thes(pool).await?;
    
    


    // Do acronym language codes....


    // Need to modify the company data to make a single entry from multiple national 
    // subsidiaries - get companies in a parent child relationship - 
    // remove the children but possibly keep a name if it is different as an alt name. 
    // Keep the parent entry as 'the' company ROR entry.


    


    Ok(())
}
mod ror;
mod names;
mod langs;
mod acros;

use crate::err::AppError;
use sqlx::{Pool, Postgres};


pub async fn load_ror_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // These simply load the ROR data into matching tables in the orgs schema
    // of the cxt DB. The ror names are loaded in two forms, the original
    // and a 'name_to_match' form, which is lower-cased, shorn of full stops, 
    // commas and brackets, and has apostrophes replaced by single right quotes.

    ror::load_ror_orgs(pool).await?;
    ror::load_ror_org_names(pool).await?;
    ror::load_ror_org_rels(pool).await?;
    ror::load_ror_org_types(pool).await?;
    ror::load_ror_org_locs(pool).await?;

    Ok(())
}


pub async fn process_ror_data(pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    names::remove_no_width_chars(pool).await?;
    names::remove_peoples_space_in_names(pool).await?;
    names::prepare_names_to_match(pool).await?;

    // Look at orgs names without a language code
    // If the org is a commercial company change the lang code to 'cm' (?) or just leave blank
    // If the former easier to see the gaps, though 'cm' needs to be added to the lang codes
    // in the language_codes lkup table.

    langs::add_cm_lang_code_to_comm_orgs(pool).await?;
    
    // Add languages if possible, using location of org and key words or word parts
    // Do language of acronyms where all other names have the same language
    // See what are left
  
    langs::update_english_names(pool).await?;
    langs::update_japanese_names(pool).await?;
    langs::update_chinese_names(pool).await?;
    langs::update_french_names(pool).await?;
    langs::update_indian_names(pool).await?;
    langs::update_iranian_names(pool).await?;
    langs::update_russian_names(pool).await?;
    langs::update_ukrainian_names(pool).await?;
    langs::update_norwegian_names(pool).await?;
    langs::update_serbian_names(pool).await?;
    langs::update_bulgarian_names(pool).await?;
    langs::update_israeli_names(pool).await?;
    langs::update_korean_names(pool).await?;
    langs::update_greek_names(pool).await?;

    langs::update_lang_code_source("lex_auto", pool).await?;

        // israel
        // greece ?
        // korea
        // taiwan +
        // india +
        // russia +
        
    // Do acronym language codes....

    langs::obtain_manual_coding_list(pool).await?;
    langs::apply_manual_coding_list(pool).await?;

    langs::update_lang_code_source("manual", pool).await?;


    // Need to consider acronyms (type = 10)
    // get all the acronyms into a table with the id, acronym
    // ???, and space for a matching name and language code

    // For each id where there is an acronym, get each name, name_to_match (?), language code, 
    // name minus 'the ', name minus ' of ', 
    // string with first letter of each word (minus 'the '), 
    // remove entries with just one word / initial letter
    // update table above with first letter of each word minus ' of ', where the name contains ' of '

    // match the atual acronyms with the derived acronyms, and create a table with the result.
    // Add rto the table additional ercords where the acronym matches the 'of-less' records.
    // (might be some similar additions, e.g. removing ' and ')
    


    // There are about 1600 names that begin with 'The '
    // These are often presented in source material without the 'The '.
    // 400 of them already include a name variant without the 'The., but
    // this call results in the remaining 1200+ also having a 'the-less'
    // version of the name added.

    names::add_names_without_thes(pool).await?;

    // Need to modify the company data to make a single entry from multiple national 
    // subsidiaries - get companies in a parent child relationship - 
    // remove the children but possibly keep a name if it is different as an alt name. 
    // Keep the parent entry as 'the' company ROR entry.

    Ok(())
}
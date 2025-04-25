use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;


pub async fn remove_no_width_chars (pool: &Pool<Postgres>) -> Result<(), AppError> {

    // remove any of the set of zero width characters 

    let mut no_width_chars = 0;
    no_width_chars += remove_unicode_char_from_names("200B", pool).await?;  // zero width space
    no_width_chars += remove_unicode_char_from_names("200C", pool).await?;  // zero width no join
    no_width_chars += remove_unicode_char_from_names("200D", pool).await?;  // zero width join
    no_width_chars += remove_unicode_char_from_names("200E", pool).await?;  // left-to-right mark
    no_width_chars += remove_unicode_char_from_names("200F", pool).await?;  // right-to-left mark
    no_width_chars += remove_unicode_char_from_names("2060", pool).await?;  // word joiner
    no_width_chars += remove_unicode_char_from_names("FEFF", pool).await?;  // zero width no-break space / BOM
    info!("{} no width characters removed from names", no_width_chars);
   
    Ok(())
}

// Some corrections of chines people 's required


pub async fn prepare_names_to_match  (pool: &Pool<Postgres>) -> Result<(), AppError> {

    // punctuation

    info!("{} periods removed from names", remove_char_from_names(".", pool).await?);
    info!("{} commas removed from names", remove_char_from_names(",", pool).await?);
    info!("{} colons removed from names", remove_char_from_names(":", pool).await?);
    info!("{} semi-colons removed from names", remove_char_from_names(";", pool).await?);
    
    // brackets

    info!("{} left parantheses replaced by spaces", replace_char_in_names("(", " ", pool).await?);
    info!("{} right parantheses removed from names", remove_char_from_names(")", pool).await?);
    info!("{} left brackets replaced by spaces", replace_char_in_names("[", " ", pool).await?);
    info!("{} right brackets removed from names", remove_char_from_names("]", pool).await?);

    // double quotes
    
    info!("{} straight double quotes removed from names", remove_unicode_char_from_names("0022", pool).await?);
    info!("{} left curved double quotes removed from names", remove_unicode_char_from_names("201C", pool).await?);
    info!("{} right curved quotes removed from names", remove_unicode_char_from_names("201D", pool).await?);
    info!("{} left bottom quotes removed from names", remove_unicode_char_from_names("201E", pool).await?);
    info!("{} left upper reversed quotes removed from names", remove_unicode_char_from_names("201F", pool).await?);
    info!("{} left guillemets removed from names", remove_unicode_char_from_names("00AB", pool).await?);
    info!("{} right guillemets removed from names", remove_unicode_char_from_names("00BB", pool).await?);
    info!("{} twin single apostrophes removed from names", remove_char_from_names("''''", pool).await?);

    // single quotes

    info!("{} low single quotes changed to left single quotes", replace_unicode_char_in_names("201A", "‘", pool).await?);
    info!("{} reverse single quotes changed to left single quotes", replace_unicode_char_in_names("201B", "‘", pool).await?);
    info!("{} full width apostrophes changed to apostrophes", replace_unicode_char_in_names("FF01", "''", pool).await?);
    info!("{} apostrophes after spaces changed to left single quotes", replace_char_in_names(" ''", "‘", pool).await?);

    // apostrophe at beginning of name changed to left single quotes

    let sql  = r#"update orgs.ror_names
            set name_to_match = '‘'||substring(name_to_match, 2)
            where name_to_match like '''%'; "#;

    let res = sqlx::query(&sql).execute(pool).await
    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} apostrophes at beginning of name changed to left single quotes", res.rows_affected());
  
    // remaining apostrophes changed to right single quotes

    info!("{} apostrophes changed to right single quotes", replace_char_in_names("''", "’", pool).await?);
    
    // bullet points

    info!("{} bullets changed to spaces", replace_unicode_char_in_names("2022", " ", pool).await?);
    info!("{} hyphen bullets changed to spaces", replace_unicode_char_in_names("2043", " ", pool).await?);
    info!("{} raised dots changed to spaces", replace_unicode_char_in_names("2219", " ", pool).await?);
    info!("{} small square changes to spaces", replace_unicode_char_in_names("25AA", " ", pool).await?);

    // standardise hyphens

    info!("{} hyphens changed to ascii hyphens", replace_unicode_char_in_names("2010", "-", pool).await?);
    info!("{} non-breaking hyphens changed to hyphens", replace_unicode_char_in_names("2011", "-", pool).await?);
    info!("{} figure dashes changed to hyphens", replace_unicode_char_in_names("2012", "-", pool).await?);
    info!("{} n dashes changed to hyphens", replace_unicode_char_in_names("2013", "-", pool).await?);
    info!("{} m dashes changed to hyphens", replace_unicode_char_in_names("2014", "-", pool).await?);
    info!("{} horizontal bars changed to hyphens", replace_unicode_char_in_names("2015", "-", pool).await?);

    // standardise hyphen spacing

    info!("{} left spaces removed from hyphens", replace_char_in_names(" -", "-", pool).await?);
    info!("{} right spaces removed from hyphens", replace_char_in_names("- ", "-", pool).await?);

    // standardise spaces

    info!("{} non breaking spaces changed to spaces", replace_unicode_char_in_names("00A0", " ", pool).await?);
    info!("{}  m quad spaces changed to spaces", replace_unicode_char_in_names("2001", " ", pool).await?);
    info!("{}  m spaces changed to spaces", replace_unicode_char_in_names("2002", " ", pool).await?);
    info!("{}  n spaces changed to spaces", replace_unicode_char_in_names("2003", " ", pool).await?);
    // info!("{}  3 per m spaces changed to spaces", replace_unicode_char_in_names("2004", " ", pool).await?);
    // info!("{}  4 per m spaces changed to spaces", replace_unicode_char_in_names("2005", " ", pool).await?);
    // info!("{}  6 per m spaces changed to spaces", replace_unicode_char_in_names("2006", " ", pool).await?);
    // info!("{}  figure spaces changed to spaces", replace_unicode_char_in_names("2007", " ", pool).await?);
    info!("{}  punctuation spaces changed to spaces", replace_unicode_char_in_names("2008", " ", pool).await?);
    // info!("{}  thin spaces changed to spaces", replace_unicode_char_in_names("2009", " ", pool).await?);
    // info!("{}  hair spaces changed to spaces", replace_unicode_char_in_names("200A", " ", pool).await?);
    // info!("{}  narrow non breaking spaces changed to spaces", replace_unicode_char_in_names("202F", " ", pool).await?);
    // info!("{}  medium mathematical spaces changed to spaces", replace_unicode_char_in_names("205F", " ", pool).await?);
    info!("{}  ideographic spaces changed to spaces", replace_unicode_char_in_names("3000", " ", pool).await?);

    info!("{} double spaces replaced by single", replace_char_in_names("  ", " ", pool).await?);

    // final trim of name 

    let sql = r#"update orgs.ror_names 
    set name_to_match = trim(name_to_match);"#;
    sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    Ok(())
}



async fn remove_char_from_names(char: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql  = format!(r#"update orgs.ror_names
            set name_to_match = replace(name_to_match, '{}', '')
            where name_to_match like '%{}%'; "#, char, char);

    let res = sqlx::query(&sql).execute(pool).await
    .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(res.rows_affected())
}


async fn replace_char_in_names(char: &str, rep_str: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql  = format!(r#"update orgs.ror_names
            set name_to_match = replace(name_to_match, '{}', '{}')
            where name_to_match like '%{}%'; "#, char, rep_str, char);

    let res = sqlx::query(&sql).execute(pool).await
    .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(res.rows_affected())
}


async fn remove_unicode_char_from_names(unicode: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql  = format!(r#"update orgs.ror_names
            set name_to_match = replace(name_to_match, U&'\{}', '')
            where name_to_match like U&'%\{}%'; "#, unicode, unicode);

    let res = sqlx::query(&sql).execute(pool).await
    .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(res.rows_affected())
}


async fn replace_unicode_char_in_names(unicode: &str, rep_str: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql  = format!(r#"update orgs.ror_names
            set name_to_match = replace(name_to_match, U&'\{}', '{}')
            where name_to_match like U&'%\{}%'; "#, unicode, rep_str, unicode);

    let res = sqlx::query(&sql).execute(pool).await
    .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(res.rows_affected())
}


pub async fn add_names_without_thes(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into orgs.ror_names (id, name, name_to_match, name_type, lang_code, lang_source, script_code)
        select n.* from 
            (select id, 
                substring(name, 5, length(name) - 4) as name,
                substring(name_to_match, 5, length(name_to_match) - 4) as name_to_match,
                2 as name_type, lang_code, lang_source, script_code
            from orgs.ror_names
            where name_to_match like 'the %'
            and array_length(string_to_array(name_to_match, ' '), 1) > 2) as n
        left join 
        orgs.ror_names r
        on n.id = r.id
        and n.name_to_match = r.name_to_match
        where r.id is null;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    info!("{} additional ROR name records added with initial 'the ' removed", res.rows_affected());
  
    Ok(())
}


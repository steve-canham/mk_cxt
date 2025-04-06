use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;

pub async fn update_english_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
        set lang_code = 'en'
        where n.lang_code is null
        and n.name_type <> 10
        and (name_to_compare like '%university%' 
        or name_to_compare like '%college%'
        or name_to_compare like '%museum%'
        or name_to_compare like '%center%'
        or name_to_compare like '%centre%'
        or name_to_compare like '%institute%'
        or name_to_compare like '%clinic%'
        or name_to_compare like '%library%'
        or name_to_compare like '%foundation%'
        or name_to_compare like '% trust%'
        or name_to_compare like '%laborator%');"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
            set lang_code = 'en'
            where n.lang_code is null
            and n.name_type <> 10
            and n.name_to_compare like '%school%'
            and n.name_to_compare not like '%hochshule%'
        "#;
    
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();


    let sql = r#"update orgs.ror_names n
    set lang_code = 'en'
    from orgs.ror_countries c
    where n.id = c.id
    and n.lang_code is null
    and n.name_type <> 10
    and name_to_compare like '%hospital%'
    and c.country_code not in ('AR', 'BO', 'CL', 'CO', 'CR', 'CU', 'DO', 'EC', 
    'ES', 'GQ', 'GT', 'HN', 'MX', 'NI', 'PE', 'PY', 'UY', 'VE', 
    'PT', 'BR', 'CV', 'AO', 'MZ', 'GW', 'ST', 'TL' );"#;
    
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();


    let sql = r#"update orgs.ror_names n
    set lang_code = 'en'
    where n.lang_code is null
    and n.name_type <> 10
    and name_to_compare like '%national%'  
    and name_to_compare not like '%nationale%'"#;
    
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language code added to english names", total_records_affected);

    Ok(())
}
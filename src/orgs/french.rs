use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;
 

pub async fn update_french_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'fr'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'FR'
            and 
            (name like 'CH %' 
            or name like 'CHU %'
            or name like 'CIC %'
            or name like 'EA %' 
            or name like 'ERL %' 
            or name like 'U %'
            or name like 'UAR%'
            or name like 'UMR%'
            or name like 'UMRS %' 
            or name like 'UMR_S %' 
            or name like 'UMS %'
            or name like 'UR%'
            or name like 'URP %'
            or name like 'US%');"#;


        // CH    Centre Hospitalier
        // CHU   Centre Hospitalier Universitaire
        // CIC   centres d’investigation clinique
        // EA    ?
        // ERL   ?
        // U 9999  unité ...
        // UAR   unités d'appui et de recherche
        // UMR   unité mixte de recherche
        // UMRS  unité mixte de recherche et service
        // UMR_S unité mixte de recherche et service
        // UMS   unité mixte de service
        // UR    unité de recherche
        // URP   unité de recherche ?
        // US    unité ?

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to french records", total_records_affected);
    
    Ok(())

}


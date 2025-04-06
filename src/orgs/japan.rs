
use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;

pub async fn update_japanese_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
                set lang_code = 'ja'
                from orgs.ror_countries c
                where n.id = c.id
                and n.lang_code is null
                and n.name_type <> 10
                and c.country_code = 'JP'
                and n.script_code <> 'Latn'"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
            set lang_code = 'ja'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'JP'
            and 
            (name_to_compare ilike '%daigaku%' 
            or name_to_compare ilike '%daigakkō%'  
            or name_to_compare ilike '%kabushiki%'
            or name_to_compare ilike '%nippon%'
            or name_to_compare ilike '%kaihatsu%'
            or name_to_compare ilike '%bijutsukan%');"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();
    
    let sql = r#"update orgs.ror_names n
            set lang_code = 'ja'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'JP'
            and 
            (name_to_compare ilike '%kenritsu%' 
            or name_to_compare ilike '%dokuritsu%'  
            or name_to_compare ilike '% kikō%'
            or name_to_compare ilike '% gakkō%'
            or name_to_compare ilike '%kaihatsu%'
            or name_to_compare ilike '%-shō%'
            or name_to_compare ilike '%bunka senta%' 
            or name_to_compare ilike '%denryoku%'  
            or name_to_compare ilike '%gakuen%'
            or name_to_compare ilike '%kagaku-kan%'
            or name_to_compare ilike '%bungaku-kan%'
            or name_to_compare ilike '%-chō%'
            or name_to_compare ilike '%kotogakko%'
            or name_to_compare ilike '%mongakkō%'
            or name_to_compare ilike '%mongakkou%');"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
            set lang_code = 'ja'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'JP'
            and (name_to_compare ilike '%fukui%' 
            or name_to_compare ilike '%hayashibara%'  
            or name_to_compare ilike '%inuyamachuobyoin%'
            or name_to_compare ilike '%kahoku%'
            or name_to_compare ilike '%kinikyochuobyoin%'
            or name_to_compare ilike '%shiritsu%'  
            or name_to_compare ilike '%kenkyūjo%'
            or name_to_compare ilike '%kenkei%'
            or name_to_compare ilike '%kyōdō%'
            or name_to_compare ilike '%kenkyūsho%');"#;
            
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
        set lang_code = 'ja'
        from orgs.ror_countries c
        where n.id = c.id
        and n.lang_code is null
        and n.name_type <> 10
        and c.country_code = 'JP'
        and (name_to_compare ilike '%kenkyujo%' 
        or name_to_compare ilike '%tankyu%'
        or name_to_compare ilike '%kenkyusho%'
        or name_to_compare ilike '%kenkyuu%'
        or name_to_compare ilike '%ritsumeikan%'
        or name_to_compare ilike '%kokusai%');"#;


    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
        set lang_code = 'ja'
        from orgs.ror_countries c
        where n.id = c.id
        and n.lang_code is null
        and n.name_type <> 10
        and c.country_code = 'JP'
        and (name_to_compare ilike '%nihon%' 
        or name_to_compare ilike '%kinzoku%'  
        or name_to_compare ilike '%kyorindo%'
        or name_to_compare ilike '%kenkyū%'
        or name_to_compare ilike '%kokudo%'
        or name_to_compare ilike '%jitsugyo%'
        or name_to_compare ilike '%fukusei%' 
        or name_to_compare ilike '%shiryokan%'  
        or name_to_compare ilike '%Gurūpu%'
        or name_to_compare ilike '%shimonosekishiritsuchuobyoin%'
        or name_to_compare ilike '%kenkyuukikou%'
        or name_to_compare ilike '%kōtōsenmongakkō%'
        or name_to_compare ilike '%toyokawashiminbyoin%'
        or name_to_compare ilike '%tsuyama%');"#;


    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes  added to japanese records", total_records_affected);

    Ok(())

}

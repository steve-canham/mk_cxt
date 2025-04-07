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
            
            // university
            // college
            // corporation
            // Japan
            // development
            // art museum
                        
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

            // prefectural
            // independent
            // organization
            // school
            // development
            // -prize
            // cultural center
            // electric power
            // academy
            // science building
            // literature building
            // district
            // senior high school
            // specialized school
            // specialized school

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
            and (name_to_compare like '%fukui%' 
            or name_to_compare like '%hayashibara%'  
            or name_to_compare like '%inuyamachuobyoin%'
            or name_to_compare like '%kahoku%'
            or name_to_compare like '%kinikyochuobyoin%'
            or name_to_compare like '%shiritsu%'  
            or name_to_compare like '%kenkyūjo%'
            or name_to_compare like '%kenkei%'
            or name_to_compare like '%kyōdō%'
            or name_to_compare like '%kenkyūsho%');"#;
            
            // Fukui
            // Hayashibara
            // Inuyama city hospital
            // Kahoku
            // Kinki Medical Center
            // municipal
            // research institute
            // survey
            // collaboration
            
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
        and (name_to_compare like '%kenkyujo%' 
        or name_to_compare like '%tankyu%'
        or name_to_compare like '%kenkyusho%'
        or name_to_compare like '%kenkyuu%'
        or name_to_compare like '%ritsumeikan%'
        or name_to_compare like '%kokusai%');"#;
        
        // research facility
        // research institute
        // research laboratory
        // research
        // Ritsumeikan
        // international
        
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
        and (name_to_compare like '%nihon%' 
        or name_to_compare like '%kinzoku%'  
        or name_to_compare like '%kyorindo%'
        or name_to_compare like '%kenkyū%'
        or name_to_compare like '%kokudo%'
        or name_to_compare like '%jitsugyo%'
        or name_to_compare like '%fukusei%' 
        or name_to_compare like '%shiryokan%'  
        or name_to_compare like '%gurūpu%'
        or name_to_compare like '%shimonosekishiritsuchuobyoin%'
        or name_to_compare like '%kenkyuukikou%'
        or name_to_compare like '%kōtōsenmongakkō%'
        or name_to_compare like '%toyokawashiminbyoin%'
        or name_to_compare like '%tsuyama%');"#;

        // Japan
        // metal
        // Kyorindo
        // research
        // national land
        // practical business
        // integrated
        // information center
        // group
        // Shimonoseki municipal hospital
        // research organization
        // high school for advanced study
        // toyokawa municipal hospital
        // Tsuyama

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to japanese records", total_records_affected);

    Ok(())

}

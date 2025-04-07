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
        or name_to_compare like '%polytechnic%'
        or name_to_compare like '%museum%'
        or name_to_compare like '%center%'
        or name_to_compare like '%institute%'
        or name_to_compare like '%clinic%'
        or name_to_compare like '%library%'
        or name_to_compare like '%society%');"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();


    let sql = r#"update orgs.ror_names n
        set lang_code = 'en'
        where n.lang_code is null
        and n.name_type <> 10
        and (name_to_compare like '%foundation%'
        or name_to_compare like '% trust%'
        or name_to_compare like '%laboratory%'
        or name_to_compare like '%laboratories%'
        or name_to_compare like '%bureau%'
        or name_to_compare like '%academy%'
        or name_to_compare like '% zoo%'
        or name_to_compare like '% park%'
         or name_to_compare like '% garden%'
        or name_to_compare like '%wikimedia%');"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();


    let sql = r#"update orgs.ror_names n
        set lang_code = 'en'
        where n.lang_code is null
        and n.name_type <> 10
        and n.name_to_compare like '%observatory%'
        and n.name_to_compare like '%observatories%'
        and n.name not like '%ПМФ%'
    "#;

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
    and name_to_compare like '%network%'  
    and name_to_compare not like '%researcherenye%'"#;
    
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language code added to english names", total_records_affected);

    Ok(())
}


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
            (name_to_compare like '%daigaku%'
            or name_to_compare like '%daigakkō%'
            or name_to_compare like '%kabushiki%'
            or name_to_compare like '%nippon%' 
            or name_to_compare like '%kaihatsu%' 
            or name_to_compare like '%bijutsukan%');"#;   
            
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
            (name_to_compare like '%kenritsu%' 
            or name_to_compare like '%dokuritsu%'  
            or name_to_compare like '% kikō%'
            or name_to_compare like '% gakkō%'
            or name_to_compare like '%kaihatsu%'
            or name_to_compare like '%-shō%'
            or name_to_compare like '%bunka senta%' 
            or name_to_compare like '%denryoku%'  
            or name_to_compare like '%gakuen%'
            or name_to_compare like '%kagaku-kan%'
            or name_to_compare like '%bungaku-kan%'
            or name_to_compare like '%-chō%'
            or name_to_compare like '%kotogakko%'
            or name_to_compare like '%mongakkō%'
            or name_to_compare like '%mongakkou%');"#;

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
            and (name_to_compare like '%chuobyoin%'
            or name_to_compare like '%shiritsu%'  
            or name_to_compare like '%kenkyūjo%'
            or name_to_compare like '%kenkei%'
            or name_to_compare like '%kyōdō%'
            or name_to_compare like '%kenkyūsho%');"#;
            
            // medical center
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
        or name_to_compare like '%kokusai%');"#;
        
        // research facility
        // research institute
        // research laboratory
        // research
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
        or name_to_compare like '%kenkyū%'
        or name_to_compare like '%kokudo%'
        or name_to_compare like '%jitsugyo%'
        or name_to_compare like '%fukusei%' 
        or name_to_compare like '%shiryokan%'  
        or name_to_compare like '%gurūpu%'
        or name_to_compare like '%shimonosekishiritsuchuobyoin%'
        or name_to_compare like '%kenkyuukikou%'
        or name_to_compare like '%kōtōsenmongakkō%'
        or name_to_compare like '%toyokawashiminbyoin%');"#;

        // Japan
        // metal
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

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to japanese records", total_records_affected);

    Ok(())

}


pub async fn update_chinese_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
                set lang_code = 'zh'
                from orgs.ror_countries c
                where n.id = c.id
                and n.lang_code is null
                and n.name_type <> 10
                and c.country_code in ('CN', 'TW')
                and n.script_code <> 'Latn'"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
                set lang_code = 'zh'
                from orgs.ror_countries c
                where n.id = c.id
                and n.lang_code is null
                and n.name_type <> 10
                and c.country_code in ('CN', 'TW')
                and (name_to_compare like '%dàxué%'
                or name_to_compare like '%daxue%'
                or name_to_compare like '%dàxúe%'
                or name_to_compare like '%zhōngyī%'
                or name_to_compare like '%xuéyuàn%'
                or name_to_compare like '%yīyuàn%'
                or name_to_compare like '%jīgòu%'
                or name_to_compare like '%yánjiū%'
                or name_to_compare like '%mínguó%');"#;
                 

        // dàxué, dàxúe, daxue   University
        // zhōngyī     (traditional) Chinese medicine
        // xuéyuàn     Educational institute (school - conservatory - academy)
        // yīyuàn      hospital
        // jīgòu       Mechanism (body - agency)
        // yánjiū      Study (Research)
        // mínguó       ??

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to chinese records", total_records_affected);

    Ok(())
}


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


pub async fn update_indian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'en'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'IN'
            and 
            (name like 'AIIMS%'
            or name like 'GCE%'
            or name like 'GMC%'
            or name like 'IIIT%'
            or name like 'IIM%'
            or name like 'IISER%'
            or name like 'IIT%' 
            or name like 'NIPER%'
            or name like 'NIT%'
            or name like 'RDC%'
            or name like 'REC%'
            or name like 'SKUAST%');"#;

       // 'AIIMS%'  All India Institute of Medical Sciences
       // 'GCE%'    Government College of Engineering
       // 'GMC%'    Government Medical College
       //'IIIT%'   International Institute of Information Technology
       //         Indian Institute of Information Technology Design & Manufacturing
       // 'IIM %'   Indian Institute of Management 
       // 'IISER%'  Indian Institute of Science Education and Research
       // 'IIT %'   Indian Institute of Technology
       // 'NIPER%'  National Institute of Pharmaceutical Education and Research
       // 'NIT %'   National Institute of Technology
       // 'RDC %'   Dental College & Hospital
       // 'REC %'   Regional / Rajkiya Engineering College 
       // 'SKUAST%' Sher-e-Kashmir University of Agricultural Sciences and Technology

    let res = sqlx::raw_sql(sql).execute(pool)
       .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();


    let sql = r#"update orgs.ror_names n
    set lang_code = 'hi'
    from orgs.ror_countries c
    where n.id = c.id
    and n.lang_code is null
    and n.name_type <> 10
    and c.country_code = 'IN'
    and 
    (name like 'KVK %'
    or name_to_compare like 'GCE%'
    or name_to_compare like '% vigyan%'
    or name_to_compare like '% vishwavidyalaya%'
    or name_to_compare like '% sanstha%'
    or name_to_compare like '% sansthā%'
    or name_to_compare like '% vidyālaya%');"#;

       // 'KVK %'    (Krishi Vigyan Kendra)  Farm Science Center
       // '% vigyan%'           science
       // '% vishwavidyalaya%'  university school
       // '% sanstha%'          organization
       // '% sansthā%'
       // '% vidyālaya%'        school
       
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to indian records", total_records_affected);
    
    Ok(())
}


pub async fn update_iranian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'fa'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'IR'
            and name_to_compare like '%dāneshgāh%';"#;

        // dāneshgāh    university

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to iranian records", total_records_affected);
    
    Ok(())
}


pub async fn update_russian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'ru'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'RU'
            and n.script_code <> 'Latn';"#;
       
    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
            set lang_code = 'ru'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'RU'
            and (name_to_compare like '%institut %'
            or name_to_compare like '%universitet%'
            or name_to_compare like '%akademiya%'
            or name_to_compare like '%akadémiya%'
            or name_to_compare like '%oblastnoy%'
            or name like 'JSC %');"#;

            // JSC  Scientific research institute

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();


    let sql = r#"update orgs.ror_names n
            set lang_code = 'ru'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'RU'
            and (name_to_compare like '%federalnyy%'
            or name_to_compare like '%patologii%'
            or name_to_compare like '%khirurgii%'
            or name_to_compare like '%shkola%'
            or name_to_compare like '%kombinat%'
            or name_to_compare like '%tsentr%');"#;

     let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to russian records", total_records_affected);
    
    Ok(())
}


pub async fn update_ukrainian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'uk'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'UA'
            and n.script_code <> 'Latn';"#;          
                  

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    let sql = r#"update orgs.ror_names n
            set lang_code = 'uk'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'UA'
            and (name_to_compare like '%universitét %'
            or name_to_compare like '%universytet%'
            or name_to_compare like '%ukrainsky%'
            or name_to_compare like '%ukrayinska%'
            or name_to_compare like '%ukrayiny%');"#;
 
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to ukranian records", total_records_affected);
    
    Ok(())
}

pub async fn update_norwegian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'no'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'NO'
            and (name_to_compare like '%sykehus%' 
            or name_to_compare like '%skole%' 
            or name_to_compare like '%skule%' 
            or name_to_compare like '%universitet%' 
            or name_to_compare like '% i %'
            or name_to_compare like '%ø%'
            or name_to_compare like '%direktoratet%'
            or name_to_compare like '%registeret%'
            or name_to_compare like '%kommune%'
            or name_to_compare like '%instituut%');"#;
 
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to norwegian records", total_records_affected);
    
    Ok(())
}


pub async fn update_serbian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'sr'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'RS'
            and (name_to_compare like '%institut%' 
            or name_to_compare like '%univerzitet%' 
            or name_to_compare like '%zvezdara%');"#;
 
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to serbian records", total_records_affected);
    
    Ok(())
}

pub async fn update_bulgarian_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let mut total_records_affected = 0;

    let sql = r#"update orgs.ror_names n
            set lang_code = 'bg'
            from orgs.ror_countries c
            where n.id = c.id
            and n.lang_code is null
            and n.name_type <> 10
            and c.country_code = 'BG'
            and (name_to_compare like '%institut%' 
            or name_to_compare like '%akademiya%' 
            or name_to_compare like '%universitet%'
            or name_to_compare like '%ministerstvo%' 
            or name_to_compare like '%obshtina%'
            or name_to_compare like '%muzei%'
            or name_to_compare like '%medicinska%');"#;
 
    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    total_records_affected += res.rows_affected();

    info!("{} language codes added to bulgarian records", total_records_affected);
    
    Ok(())
}



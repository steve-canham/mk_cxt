use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;


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
                 

        // dàxué, dàxúe  University
        // daxue       University
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

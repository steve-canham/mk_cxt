use crate::err::AppError;
use sqlx::{Pool, Postgres};


pub async fn create_org_attribute_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.org_attribute_types;
    CREATE TABLE lups.org_attribute_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        data_type          varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL

    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.org_attribute_types (id, name, data_type, 
          description, list_order)
       values 
        (11, 'URL', 'url', 'URL of the landing page of the organisation – may be in various language versions', 10),
        (12, 'Wikipedia entry', 'url', 'URL of main Wikipedia page, if one exists', 20),
        (13, 'Wikidata entry', 'url', 'URL of a Wikidata link, if one exists', 30),
        (1001, 'NLM Databank Id', 'string', 'Abbreviation used within PubMed to indicate a data repository', 40),
        (1004, 'BBMRI identifier', 'string', 'PID applieed by BBMRI to biobank organisations', 50),
        (1002, 'ISNI id', 'string', 'String identifier from the International Standard Name Identifier scheme', 150),
        (1003, 'GRID id', 'string', 'String identifier from the Global Research Identifier Database', 160);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_org_classes(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.org_classes;
    CREATE TABLE lups.org_classes (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.org_classes (id, name, description, list_order)
       values 
       (100, 'Government', 'An organization that is part of or operated by a national or regional government and that conducts or supports research.', 10), 
       (200, 'Education', 'A university or similar institution involved in providing education and educating/employing researchers.', 20), 
       (300, 'Healthcare', 'A medical care facility such as hospital or medical clinic. Excludes medical schools, which should be categorized as ‘Education”.', 30), 
       (400, 'Company', 'A private for-profit corporate entity involved in conducting or sponsoring research.', 40), 
       (500, 'Nonprofit', 'A non-profit and non-governmental organization involved in conducting or funding research.', 50), 
       (600, 'Funder', 'An organization that awards research funds or provides in-kind support. Organisations linked to a Funder ID will be assigned this type, usually in conjunction with another type.', 60),
       (700, 'Facility', 'A specialized facility where research takes place, such as a laboratory or telescope or dedicated research area.', 70), 
       (800, 'Archive', 'An organization involved in stewarding research and cultural heritage materials. Includes libraries, museums, and zoos.', 80),  
       (900, 'Other', 'A category for any organization that does not fit the other named categories.', 90);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_org_name_qualifier_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.org_name_qualifier_types;
    CREATE TABLE lups.org_name_qualifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.org_name_qualifier_types (id, name, description, list_order)
       values 
        (1, 'Default', 'The default (and usual display) name. Should normally be in the dominant local language.', 10),
        (2, 'Also listed as', 'Alternative name, in any language, including English versions of the default name if different.', 20),
        (3, 'Abbreviated name', 'Abbreviation or acronym that is distinct enough to be listed / matched as a valid name option', 30),
        (10, 'Abbreviated as', 'Abbreviation or acronym', 50),
        (18, 'Previously known as', 'Historic name, no longer in current use', 60),
        (19, 'External Foreign name', 'Name in a foreign language NOT generally used in local area (e.g. as found in a foreign listing)', 70),
        (20, 'Minor variation', 'Minor variation or mis-spelt version of a name, or one with a location suffix or different punctuation, that exists in source data but is not in normal usage.', 80);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_org_relationship_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.org_relationship_types;
    CREATE TABLE lups.org_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.org_relationship_types (id, name, description, list_order)
       values 
        (1001, 'is a department within', 'Subject organisation is a departement, division, faculty etc. within the larger object organisation, institution or company, without indepenmdent legal standing itself', 20),
        (1012, 'contains', 'Subject organisation contains the object organisations as a sub-division, department, faculty, labnoratory (etc.). Object organisations are presented as an array', 25),
        (1002, 'is part of', 'A stand-alone subject organisation that is part of a larger formal grouping, e.g. one of several national subsidaries, one of several regional government offices, etc.', 30),
        (1004, 'includes', 'Subject organisation includes the object organisations as stand-alone organisational components (presented as an array of Ids).', 35),
        (1003, 'is a member of ', 'Subject organisation is a member of the informal collaboration or grouping that is the object organisation.', 40),
        (1011, 'involves collaboration of', 'Subject organisation brings together several independent object organisations (presented as an array of Ids) within a collaboration or common working group.', 45),
        (1005, 'was formed from the split of ', 'Subject organisation created when object organisation split or divested itself of part of its structure', 60),
        (1008, 'was acquired by', 'Subject organisation ceased to exist when it was acquired / became part of the object organisation', 70),
        (1006, 'was formed from the merger of', 'Subject organisation was created from the merger of the two or more object organisations (provided as an array of Ids)', 80),
        (1009, 'was merged into', 'Subject organisation ceased to exist when it ws part of the merger that created the object organisation.', 90),
        (1007, 'is funded by', 'Subject organisation receives all or part of its funding from the object organisation', 100),
        (1010, 'is affiliated with / related to', 'Subject organisation is related to and / or loosely affiliated to the object organisation (details should be prpovided in comments)', 110),
        (1013, 'other', 'Relationship not falling within any of the other categories.', 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_org_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.org_types;
    CREATE TABLE lups.org_types (
        id                 int4       NOT NULL PRIMARY KEY,
        class_id           int4       NULL,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.org_types (id, class_id, name, 
          description, list_order)
       values
        (1044, 11, 'National research institute / centre', '', 101),
        (1026, 11, 'University research institute', '', 105),
        (1048, 11, 'Hospital research institute', '', 110),
        (1021, 11, 'Other government research institution', '', 112),
        (1006, 11, 'Independent research organisation (not for profit)', '', 115),
        (1001, 11, 'Clinical trials unit', '', 120),
        (1004, 11, 'Genetics / molecular biology laboratory', '', 125),
        (1043, 11, 'Specialist cancer research centre', '', 130),
        (1003, 11, 'Biochemistry / cell biology laboratory', '', 130),
        (1002, 11, 'Epidemiology research unit', '', 135),
        (1014, 14, 'University', '', 160),
        (1025, 14, 'University faculty', '', 165),
        (1039, 14, 'Medical university', '', 170),
        (1060, 14, 'University specialising in traditional medicine', '', 175),
        (1053, 14, 'Postgraduate medical education department ', '', 180),
        (1042, 12, 'University hospital', '', 201),
        (1041, 12, 'Hospital group', '', 203),
        (1011, 12, 'General hospital ', '', 205),
        (1012, 12, 'Children’s hospital or children and women’s hospital', '', 210),
        (1054, 12, 'Hospital specialising in traditional medicine', '', 215),
        (1013, 12, 'Specialist cancer hospital or centre', '', 220),
        (1062, 12, 'Community based health organisation', '', 225),
        (1064, 14, 'Other HE Institution', '', 225),
        (1015, 13, 'Pharmaceutical company', '', 301),
        (1016, 13, 'Biotech company', '', 303),
        (1017, 13, 'Medical device company', '', 305),
        (1063, 13, 'Supplier of biological materials, reagents', '', 310),
        (1050, 13, 'Journal publishers', '', 315),
        (1052, 13, 'Multi-sector company', '', 318),
        (1018, 13, 'Commercial research organisation', '', 320),
        (1019, 13, 'Consultancy (pharma industry)', '', 330),
        (1020, 13, 'Consultancy (IT)', '', 333),
        (1022, 13, 'Software vendor', '', 336),
        (1023, 13, 'IT hosting service', '', 341),
        (1036, 15, 'United Nations agency', '', 401),
        (1045, 15, 'Supra-national government agency', '', 405),
        (1055, 15, 'Health funding programme', '', 410),
        (1056, 15, 'Regional healthcare co-ordination / funding agency', '', 415),
        (1037, 15, 'Government funding agency', '', 420),
        (1029, 15, 'Government health related department', '', 425),
        (1038, 15, 'Other government department', '', 430),
        (1027, 16, 'National drug and / or device regulatory authority', '', 460),
        (1028, 16, 'National regulator for data protection', '', 465),
        (1051, 16, 'Review Board or Ethics Committee', '', 470),
        (1008, 16, 'International research infrastructure', '', 475),
        (1007, 16, 'National research infrastructure', '', 480),
        (1065, 16, 'Research collaborative group', '', 485),
        (1030, 16, 'Non governmental research funder', '', 495),
        (1033, 17, 'Trial registry (WHO Source)', '', 501),
        (1069, 17, 'Studies registry (MDR source, not a WHO source)', '', 503),
        (1061, 17, 'Studies registry (neither an MDR or WHO source)', '', 505),
        (1035, 17, 'Data repository', '', 510),
        (1032, 17, 'Bibliographic database', '', 515),
        (1034, 17, 'Metadata repository', '', 520),
        (1066, 17, 'Biobank or samples collection', '', 525),
        (1031, 18, 'Charity', '', 601),
        (1040, 18, 'General professional association', '', 605),
        (1067, 18, 'Condition specific professional Association', '', 610),
        (1068, 18, 'Patient / advocacy group', '', 615);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}

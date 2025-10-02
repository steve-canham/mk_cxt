use crate::err::AppError;
use sqlx::{Pool, Postgres};




pub async fn create_contribution_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.contribution_types;
    CREATE TABLE lups.contribution_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        applies_to         varchar    NULL,
        description        varchar    NULL,
        use_in_data_entry  bool       NULL,
        list_order         int4       DEFAULT 10 NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.contribution_types (id, name, applies_to, description, use_in_data_entry, list_order)
       values
        (0, 'Not yet known', 'both', 'Dummy value supplied by default on entity creation.', false, 999),
        (11, 'Creator / Author', 'both', 'The main researchers involved in producing the data, or the authors of the publication, in priority order. May be a corporate, research group or personal name.', true, 45), 
        (12, 'Resource contact', 'individual', 'From DC, person with knowledge of how to access, troubleshoot, or otherwise field issues related to the resource. May also be “Point of Contact” in organisation that controls access to the resource.', true, 25),
        (13, 'Data collector', 'both', 'Person/institution responsible for finding, gathering/collecting data under the guidelines of the author(s) or Principal Investigator (PI).', true, 320),
        (14, 'Data curator', 'both', 'Person/institution tasked with reviewing, enhancing, cleaning, or standardizing metadata and the associated data submitted for storage, use, and maintenance within a data centre or repository.', true, 325),
        (15, 'Data manager', 'both', 'Person (or organisation with a staff of data managers, such as a data centre) responsible for maintaining the finished resource, including ensuring that the resource is periodically “refreshed” in terms of software/hardware support, is kept available and is protected from unauthorized access.', true, 330),
        (16, 'Distributor', 'organisation', 'Institution tasked with responsibility to generate/disseminate copies of the resource in either electronic or print form. Works stored in more than one archive/repository may credit each as a distributor.', false, 235),
        (17, 'Editor', 'individual', 'A person who oversees the details related to the publication format of the resource. If the Editor is to be credited in place of multiple creators, the Editor’s name may be supplied as Creator, with “(Ed.)” appended to the name.', false, 50),
        (18, 'Hosting institution', 'organisation', 'Typically, the organisation allowing the resource to be available on the internet through the provision of its hardware/software/operating support. May also be used for an organisation that stores the data offline. Often a data centre (if that data centre is not the “publisher” of the resource).', false, 245),
        (19, 'Producer (DC)', 'both', 'Typically a person or organisation responsible for the artistry and form of a media product. In the data industry, this may be a company “producing” DVDs that package data for future dissemination by a distributor. ', false, 250),
        (26, 'Researcher', 'individual', 'A person involved in analyzing data or the results of an experiment or formal study. May indicate an intern or assistant to one of the authors who helped with research but who was not so “key” as to be listed as an author. Should be a person, not an institution.', true, 85),
        (27, 'Research group', 'organisation', 'Typically refers to a group of individuals with a lab, department, or division; the group has a particular, defined focus of activity. May operate at a narrower level of scope; may or may not hold less administrative responsibility than a project team.', true, 120),
        (28, 'Resource rights holder ', 'both', 'Person or institution owning or managing property rights, including intellectual property rights over the resource.', true, 395),
     
        (50, 'Study lead', 'individual', 'The individual who leads and co-ordinates the scientific and clinical activity within a clinical study, including co-ordinating the work of principal investigators at clinical sites. May be known as the Co-ordinating Investigator, the Study Chair, Study Director, the Scientific Contact or similar terms.', true, 10),
        (51, 'Principal investigator', 'individual', 'The individual responsible for the safe conduct of a clinical trial at a particular clinical site.', true, 15),
        (52, 'Clinical study manager', 'individual', 'An individual responsible for the operational management of a clinical study. Similar to a Project Manager though a study manager is often involved in the management of data and data collection.', true, 20),
        (53, 'Independent monitoring committee member', 'individual', 'A member of a safety monitoring committee for a clinical trial, independent of the researchers and research activity.', true, 160),
        (54, 'Statistician', 'individual', 'A member of the research team specifically identified as provifing statistical analysis for the study.', true, 160),
        (55, 'Research group member', 'individual', 'From PubMed, an individual (e.g., collaborator or investigator) who is not an author of a paper but is listed as a member of a collective/corporate group that is an author of the paper.', true, 220),
      
        (60, 'Sponsor-investigator', 'individual', 'An individual with the role of sponsor as well as being the co-ordinating investigator for the study.', true, 210),
       
        (70, 'Scientific contact', 'both', 'An individual or office, e.g. the study lead or a representative of the sponsor, acting as an initial contact point.', true, 27),
        (71, 'Public contact', 'both', 'An individual or office designated as dealing with non-scientific queries from the public or press.', true, 29),
        (72, 'Recruitment contact', 'both', 'An individual or office designated as providing periodic updates on recruitment information or status, at all sites, usually for monitoring purposes.', false, 31),
        (73, 'Funder contact', 'both', 'An individual or office representing the funder and acting as an initial contact point.', false, 33),
        (74, 'Results contact', 'both', 'The individual, occasionally organisation, to be contacted for further information on the study results.', true, 35),
        (75, 'Ethics contact', 'both', 'An individual or office designated as dealing with queries about the study’s ethical approval.', true, 129),
        
        (90, 'Ethics approval organisation', 'organisation', 'The organisation that provided ethical approval for the study', true, 128),
        (91, 'Clinical organisation', 'organisation', 'Organisation, usually a primary or secondary health care organisation, that manages one or more of the sites where a clinical study takes place.', true, 195),
        (92, 'Clinical site', 'organisation', 'Organisation or location, usually in a primary or secondary health care organisation, that is one of the sites where a clinical study takes place.', false, 200),

        (100, 'Sponsor', 'organisation', 'The organisation or individual that for a clinical trial has the formal, legal role of a clinical trial sponsor, and for observational studies has an analogous responsibility for the overall organisation and conduct of the study.', true, 130),
        (110, 'Collaborating organisation', 'organisation', 'May be listed as a secondary sponsor, an organisation other than the lead sponsor involved in supporting a study, but not identified in a specific role.', true, 205),
        (111, 'Study funder', 'organisation', 'An organisation providing some or all of the additional funds required for the study.', true, 150),
        (112, 'Medicinal product supplier', 'organisation', 'Organisation that provides one or more of the medicines investigated in a clinical study.', true, 165),
        (113, 'Medical device supplier', 'organisation', 'Organisation that provides one or more of the medical devices in a clinical study.', true, 170),
        (114, 'Logistics support organisation', 'organisation', 'Organisation that provided logistical input, e.g. provides a drug distribution service.', true, 175),
        (115, 'Research infratructure', 'organisation', 'Organisation that provided scientific infrastructure support within the study, e.g. a national or international research network.', true, 180),
        (116, 'Central laboratory', 'organisation', 'Organisation that provided a central specialist laboratory testing facility within the study.', true, 185),
        (117, 'Central imaging facility', 'organisation', 'Organisation that provided a central specialist imaging or scanning facility within the study.', true, 190),
        (118, 'Commercial research organisation', 'organisation', 'A commercial organisation used by the sponsor to carry out study and data management, usually referred to as a CRO', true, 127),
        (120, 'Study sponsor and funder', 'organisation', 'The organisation that for a clinical trial has the formal, legal role of a clinical trial sponsor, and which is also listed as a study funder.', true, 155),
        
        (990, 'Other', 'both', 'Any person or institution making a significant contribution to the development and/or maintenance of the resource, but whose contribution does not “fit” other controlled vocabulary for contributorType.', true, 900);"#;
  

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())

}


pub async fn create_gender_eligibility_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.gender_eligibility_types;
    CREATE TABLE lups.gender_eligibility_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.gender_eligibility_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
       (1, 'Female', 'Study recruits only female participants', true, 10),
       (2, 'Male', 'Study recruits only male participants', true, 20),
       (3, 'Both', 'Study open to both male and female participants', true, 30);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


/* 
pub async fn identifier_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.identifier_types;
    CREATE TABLE lups.identifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    applies_to         varchar    NULL,
	    description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.identifier_types (id, name, applies_to, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (11, 'Trial registry ID', 'Study', 'Org id should identify the registry.', true, 10, 'ECRIN', '2019-01-14'),
        (41, 'Regulatory body ID', 'Study', 'Number generated by a national authority when a study applies for authorisation / ethical approval, e.g. IRAS number in the UK, ANSM number in France', true, 12, 'ECRIN', '2019-11-20'),
        (12, 'Ethics review ID', 'Study', 'Org id should identify the review body.', true, 15, 'ECRIN', '2019-01-14'),
        (13, 'Funder / Contract ID', 'Study', 'May be a grant id or contract, org id should identify the funder.', true, 20, 'ECRIN', '2019-01-14'),
        (14, 'Sponsor’s ID', 'Study', 'Usually a protocol ID. Org id should identify the sponsor.', true, 25, 'ECRIN', '2019-01-14'),
        (39, 'NCI CTRP ID', 'Study', 'ID generated by the US NCI for supported cancer trials (in the Clinical Trial Reporting Program), usually NCI- followed by year followed by -5 digits', false, 27, 'NIH', '2019-11-20'),
        (40, 'DAIDS-ES registry ID', 'Study', 'ID used within the US NIH NAID Division of AIDS Enterprise System (DAIDS-ES).', false, 28, 'NIH', '2019-11-20'),
        (42, 'NHLBI BiioLINCC ID', 'Study', 'Accession number, from NHLBI BioLINCC study database', false, 36, 'NIH', '2019-12-14'),
        (43, 'CTMS ID', 'Study', 'ID as used by a clinical trial management or data management system', true, 40, 'ECRIN', '2019-11-20'),
        (46, 'CTEP Number', 'Study', 'ID allocated by the US Cancer Therapy Evaluation program', false, 41, 'ECRIN', '2023-04-20'),
        (47, 'Additional Sponsor ID', 'Study', 'Additional Id listed by sponsor', false, 42, 'ECRIN', '2023-04-20'),
        (48, 'Agency Reference ID', 'Study', 'Reference or Protocol ID assigned by a Government agency (e.g. NIH institute), not covered elsewhere', false, 43, 'ECRIN', '2023-04-20'),
        (50, 'Research Collaboration ID', 'Study', 'ID provided by a research collaborative group, usualluy a trials group', true, 44, 'ECRIN', '2023-04-20'),
        (44, 'Obsolete NCT number', 'Study', 'Obsolete NCT number or NCT alias', false, 45, 'CTG', '2022-12-24'),
        (45, 'Obsolete NTR number', 'Study', 'Obsolete NTR number (Dutch registry only)', false, 47, 'ECRIN', '2023-04-15'),
        (49, 'NCI PDQ ID', 'Study', 'Id used in NCI’s Physician Data Query system (normally CDR followed by 10 digits)', false, 50, 'ECRIN', '2023-04-20'),
        (51, 'AHRQ ID', 'Study', 'ID assigned by the US Healthcare Research and Quality Agency', false, 51, 'CTG', '2023-08-05'),
        (52, 'NIAID CRMS Id', 'Study', 'ID used for the study within the US NIAID Clinical Research Management System', true, 52, 'NIH', '2023-08-05'),
        (90, 'Other', 'All', 'None of the listed identifier types.', true, 990, 'ECRIN', '2019-01-14'),
        (0, 'Not yet known', 'All', 'Dummy value supplied by default on entity creation.', false, 990, 'ECRIN', '2019-01-14'),
        (1, 'Type not provided', 'All', 'Missing type data in data source.', false, 998, 'ECRIN', '2019-11-03');"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}
*/


pub async fn create_iec_level_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.iec_level_types;
    CREATE TABLE lups.iec_level_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.iec_level_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (0, 'None', 'No inclusion / exclusion material in source material', true, 10),
        (1, 'Single statement', 'A single statement, without carriage returns. May not be headed. If headed, often as ‘eligibility’, rather than ‘inclusion’', true, 20),
        (2, 'Single paragraph', 'With internal carriage returns, implying multiple statements. The paragraph often labelled ‘eligibility’ or equivalent, rather than ‘inclusion’.', true, 30),
        (4, 'Single inclusion statement', 'Includes the word inclusion but without carriage returns. May be structured into separate criteria or may be a sigle statement.', true, 40),
        (8, 'Inclusion paragraph', 'Includes the word inclusion and contains carriage returns. Assumed that different lines represent different criteria.', true, 50),
        (16, 'Single exclusion statement', 'Includes the word exclusion but without carriage returns. May be structured into separate criteria or may be a sigle statement.', true, 60),
        (32, 'Exclusion paragraph', 'Includes the word exclusion and contains carriage returns. Assumed that different lines represent different criteria.', true, 70),
        (20, 'Single inclusion + Single exclusion', 'A single statement for each of inclusion and exclusion', true, 80),
        (36, 'Single inclusion + Multiple exclusion', 'A single statement for inclusion criteria but multiple exclusion criteria', true, 90),
        (24, 'Multiple inclusion + Single exclusion', 'Multiple inclusion criteria but with a single statement covering exclusion', true, 100),
        (40, 'Multiple inclusion + exclusion', 'Sets of both inclusion and exclusion criteria statements', true, 110);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_study_feature_categories(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.study_feature_categories;
    CREATE TABLE lups.study_feature_categories (
        id                 int4       NOT NULL PRIMARY KEY,
        feature_type_id    int4       NOT NULL,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.study_feature_categories (id, feature_type_id, name, 
          description, list_order)
       values 
        (105, 20, 'Early phase 1','Exploratory trials, involving very limited human exposure, with no therapeutic or diagnostic intent (e.g., screening studies, microdose studies).', 10),
        (110, 20, 'Phase 1','Initial studies to determine the metabolism and pharmacologic actions of drugs in humans, side effects, and to gain early evidence of effectiveness; may include healthy participants and/or patients.', 20),
        (115, 20, 'Phase 1/2','Trials that are a combination of phases 1 and 2.', 30),
        (120, 20, 'Phase 2','Controlled clinical studies conducted to evaluate the effectiveness of the intervention for a particular indication in participants with the disease or condition under study, and to determine the common short-term side effects and risks.', 40),
        (125, 20, 'Phase 2/3','Trials that are a combination of phases 2 and 3.', 50),
        (130, 20, 'Phase 3','Trials conducted after preliminary evidence suggesting effectiveness of the drug has been obtained, and are intended to gather additional information to evaluate the overall benefit-risk relationship of the drug.', 60),
        (135, 20, 'Phase 4','Studies of approved drugs to delineate additional information including the drug’s risks, benefits, and optimal use.', 70),
        (100, 20, 'Not applicable','Trials without phases (for example, studies of devices or behavioral interventions).', 80),
        (140, 20, 'Not provided','No data was provided in the source record.', 90),
        (400, 21, 'Treatment','One or more interventions are being evaluated for treating a disease, syndrome, or condition.', 10),
        (405, 21, 'Prevention','One or more interventions are being assessed for preventing the development of a specific disease or health condition.', 20),
        (410, 21, 'Diagnostic','One or more interventions are being evaluated for identifying a disease or health condition.', 30),
        (415, 21, 'Supportive care','One or more interventions are evaluated for maximizing comfort, minimizing side effects, or mitigating against a decline in the participant’s health or function.', 40),
        (420, 21, 'Screening','One or more interventions are assessed or examined for identifying a condition, or risk factors for a condition, in people who are not yet known to have the condition or risk factor.', 50),
        (425, 21, 'Health services research','One or more interventions for evaluating the delivery, processes, management, organization, or financing of healthcare.', 60),
        (430, 21, 'Basic science','One or more interventions for examining the basic mechanism of action (for example, physiology or biomechanics of an intervention).', 70),
        (435, 21, 'Device feasibility','An intervention of a device product is being evaluated in a small clinical trial (generally fewer than 10 participants) to determine the feasibility of the product; or a clinical trial to test a prototype device for feasibility and not health outcomes. Such studies are conducted to confirm the design and operating specifications of a device before beginning a full clinical trial.', 80),
        (450, 21, 'Educational / counselling / training','An intervention involving psychosocial or educational input', 90),
        (440, 21, 'Other','None of the other options applies.', 100),
        (445, 21, 'Not provided','No data was provided in the source record.', 900),
        (205, 22, 'Randomised','Participants are assigned to intervention groups by chance', 10),
        (210, 22, 'Nonrandomised','Participants are expressly assigned to intervention groups through a non-random method, such as physician choice', 20),
        (200, 22, 'Not applicable','For a single-arm trial', 30),
        (215, 22, 'Not provided','No data was provided in the source record.', 40),
        (300, 23, 'Single group assignment','Clinical trials with a single arm', 10),
        (305, 23, 'Parallel assignment','Participants are assigned to one of two or more groups in parallel for the duration of the study', 20),
        (310, 23, 'Crossover assignment','Participants receive one of two (or more) alternative interventions during the initial phase of the study and receive the other intervention during the second phase of the study', 30),
        (315, 23, 'Factorial assignment','Two or more interventions, each alone and in combination, are evaluated in parallel against a control group', 40),
        (320, 23, 'Sequential assignment','Groups of participants are assigned to receive interventions based on prior milestones being reached in the study, such as in some dose escalation and adaptive design studies', 50),
        (325, 23, 'Not provided','No data was provided in the source record.', 60),
        (500, 24, 'None (Open Label)','', 10),
        (502, 24, 'Blinded (no details)','From a statement that says the study was blinded, but where the degree of blinding was not provided', 15),
        (505, 24, 'Single','', 20),
        (510, 24, 'Double','', 30),
        (515, 24, 'Triple','', 40),
        (520, 24, 'Quadruple','', 50),
        (599, 24, 'Not applicable','Explicitly labelled as not applicable - usually because the study is non-interventional	', 60),
        (525, 24, 'Not provided','No data was provided in the source record.', 90),
        (600, 30, 'Cohort','Group of individuals, initially defined and composed, with common characteristics (for example, condition, birth year), who are examined or traced over a given time period.', 10),
        (605, 30, 'Case-control','Group of individuals with specific characteristics (for example, conditions or exposures) compared to group(s) with different characteristics, but otherwise similar.', 20),
        (610, 30, 'Case-only','Single group of individuals with specific characteristics.', 30),
        (615, 30, 'Case-crossover','Characteristics of case immediately prior to disease onset (sometimes called the hazard period) compared to characteristics of same case at a prior time (that is, control period).', 40),
        (620, 30, 'Ecologic or community study','Geographically defined populations, such as countries or regions within a country, compared on a variety of environmental (for example, air pollution intensity, hours of sunlight) and/or global measures not reducible to individual level characteristics (for example, healthcare system, laws or policies median income, average fat intake, disease rate).', 50),
        (625, 30, 'Family-based','Studies conducted among family members, such as genetic studies within families or twin studies and studies of family environment.', 60),
        (640, 30, 'Defined population','', 70),
        (645, 30, 'Natural history',' ', 80),
        (630, 30, 'Other','None of the othjer categories provided.', 100),
        (635, 30, 'Not provided','No data was provided in the source record.', 900),
        (700, 31, 'Retrospective','Look back using observations collected predominantly prior to subject selection and enrollment', 10),
        (705, 31, 'Prospective','Look forward using periodic observations collected predominantly following subject enrollment', 20),
        (710, 31, 'Cross-sectional','Observations or measurements made at a single point in time, usually at subject enrollment', 30),
        (725, 31, 'Retrospective / prospective','', 40),
        (730, 31, 'Longitudinal','', 50),
        (715, 31, 'Other','Explain in Detailed Description', 100),
        (720, 31, 'Not provided','No data was provided in the source record.', 900),
        (800, 32, 'None retained','', 10),
        (805, 32, 'Samples with DNA ','', 20),
        (810, 32, 'Samples without DNA','', 30),
        (815, 32, 'Not provided','No data was provided in the source record.', 40);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_study_feature_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.study_feature_types;
    CREATE TABLE lups.study_feature_types (
        id                 int4       NOT NULL PRIMARY KEY,
        context            varchar    NULL,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.study_feature_types (id, context, name, 
          description, list_order)
       values 
        (20, 'interventional', 'Phase','For a clinical trial of a drug product (including a biological product), the numerical phase of the clinical trial.', 10),
        (21, 'interventional', 'Primary purpose','In very broad terms, the clinical or scientific area the trial is intended to contribute towards.', 20),
        (22, 'interventional', 'Allocation type','The method by which participants are assigned to arms in a clinical trial.', 30),
        (23, 'interventional', 'Intervention model','The strategy for assigning interventions to participants.', 40),
        (24, 'interventional', 'Masking','The party or parties involved in the clinical trial who are prevented from having knowledge of the interventions assigned to individual participants.', 50),
        (30, 'observational', 'Observational model','The Primary strategy for participant identification and follow-up.', 60),
        (31, 'observational', 'Time perspective','For observational studies, describes the temporal relationship of observation period to time of participant enrollment.', 70),
        (32, 'observational', 'Biospecimens retained','Indicates whether samples of material from research participants are retained in a biorepository.', 80);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_study_relationship_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.study_relationship_types;
    CREATE TABLE lups.study_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.study_relationship_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (11, 'Is a sub-study of', 'This study is a sub-study or sub-protocol undertaken at the same time as <the target study>.', true, 10),
        (12, 'Includes as a sub-study', 'Has <the target study> as a sub-study or sub-protocol, undertaken at the same time as this study.', true, 15),
        (13, 'Is in the same series as', 'This study is in a sequence that began with <the target study>.', true, 20),
        (14, 'Is the first of a sequence including', 'This study began a sequence that includes <the target study>.', true, 25),
        (15, 'Is a feasibility study for', 'This study is a feasibility or pilot study for <the target study>.', true, 30),
        (16, 'Is preceded by the feasibility study', 'This study was preceded by <the target study> as a feasibility or pilot study.', true, 35),
        (17, 'Is a later phase variant of', 'For trials using the phase I to IV classification, this study is a later phase continuation of <the target study>.', true, 40),
        (18, 'Is an earlier phase variant of', 'For trials using the phase I to IV classification, this study is an earlier phase precursor to <the target study>.', true, 45),
        (19, 'Is a continuation of', 'This study uses some or all of the same subject population as <the target study>.', true, 50),
        (20, 'Is continued by', 'This study has some or all of the same subject population targeted by <the target study>.', true, 55),
        (21, 'Is a repeat of', 'This study uses a different population but the same or similar protocol as <the target study>.', true, 60),
        (22, 'Is repeated by', 'This study is repeated by <the target study>, with the same or similar protocol but using a different population.', true, 65),
        (23, 'has an expanded access version', 'This study has <the target study> as an expanded access version, (for people who cannot enrol in the trial but who may benefit from the product under investigation).', true, 70),
        (24, 'is an expanded access version of', 'This study is an expanded access version of <the target study>, catering for people who cannot enrol in the trial but who may benefit from the product under investigation.', true, 75),
        (25, 'Includes target as one of a group of non-registered studies', 'This study includes <the target study>. That study is not registered independently, but instead shares this registry entry with one or more other non-registered studies.', false, 80),
        (26, 'Non registered but included within a registered study group', 'This study is registered as <the target study>, along with one or more other studies that share the same registry entry and id.', false, 85),
        (27, 'Has link listed in registry but nature of link unclear', 'This study is linked to <the target study> within the registry entry, but the nature of the linkage is not clear.', false, 90),
        (28, 'Includes target as one of a group of registered studies', 'This study includes <the target study>, which is registered elsewhere along with one or more other registered studies, forming a group that collectively equates to this study.', false, 95),
        (29, 'Registered and is included elsewhere in group', 'This study is also registered, along with one or more other studies that together form an equivalent group, as <the target study>.', false, 100),
        (30, 'One of a related group of 3 or more studies', 'One of a derived group of related studies - exact relationship between them is unclear', false, 105),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_study_statuses(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.study_statuses;
    CREATE TABLE lups.study_statuses (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
        source_terms       varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.study_statuses (id, name, description, source_terms,
          use_in_data_entry, list_order)
       values
        (10, 'Not yet recruiting', 'Participants are not yet being recruited.', '‘NOT_YET_RECRUITING’, ‘Not yet recruiting’, ‘Pending’, ‘Without startig enrollment’ (sic), ‘Preinitiation’', true, 10),
        (12, 'Withdrawn', 'Study halted prior to enrolment of first participant.', '‘WITHDRAWN’, ‘Withdrawn’', false, 70),
        (14, 'Recruiting', 'Participants are currently being recruited, whether or not any participants have yet been enrolled.', '‘RECRUITING’, ‘Recruiting’, ‘Open public recruiting’, ‘Open to recruitment’, ‘In enrollment’', true, 20),
        (16, 'Enrolling by invitation', 'Participants are being (or will be) selected from a predetermined population. Largely limited to ClinicalTrials.gov', '‘ENROLLING_BY_INVITATION’', false, 50),
        (18, 'Ongoing, no longer recruiting', 'Participants are receiving interventions or being assessed, but new participants are not being recruited or enrolled.', '‘ACTIVE_NOT_RECRUITING’', true, 30),
        (20, 'Ongoing, recruitment unclear', 'Trial is ongoing but recruitment status is unclear. Applies to many EUCTR studies', '‘AVAILABLE’, ‘Ongoing’, ‘Authorised-recruitment may be ongoing or finished’', false, 60),
        (25, 'Suspended', 'Study halted prematurely but potentially will resume.', '‘SUSPENDED’, ‘Suspended’, ‘Temporarily closed’, ‘Temporary halt’', true, 80),
        (30, 'Completed', 'The study has concluded normally; participants are no longer receiving an intervention or being examined (that is, last participant’s last visit has occurred).', '‘COMPLETED’, ‘Completed’, ‘APPROVED_FOR_MARKETING’)', true, 40),
        (32, 'Terminated', 'Study halted prematurely and will not resume; participants are no longer being examined or receiving intervention.', '‘TERMINATED’, ‘Terminated’, ‘Stopped early’, ‘Stopped’', true, 90),
        (98, 'Not applicable', 'Explicitly listed as Not Applicable. Found in some WHO source registries', '‘Not applicable’', true, 110),
        (99, 'Other', 'Status given as "other" or does not fit within other available options - would normally require further investigation.', null, true, 100),
        (0, 'Not provided', 'Status information not provided.', 'A null status value, ‘WITHHELD’, ‘NO_LONGER_AVAILABLE’, ‘TEMPORARILY_NOT_AVAILABLE’', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_study_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.study_types;
    CREATE TABLE lups.study_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
        source_terms       varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.study_types (id, name, description, source_terms,
          use_in_data_entry, list_order)
       values 
       (11, 'Interventional', 'Participants are assigned to receive an intervention so that researchers can study the effects.', 'From any type beginning with ‘intervent’ / ‘INTERVENT’, or BA/BE (= bio-availabilty / bio-equivalence, Indian registry only)', true, 10),
       (12, 'Observational', 'Participants do not receive an assigned intervention. They are observed over time for health-related outcomes.', 'From any type beginning with ‘observ’ / ‘OBSERV’ or ‘epidem’, or ‘PMS’ (= post market surveillence), or ‘Relative factors’', true, 20),
       (13, 'Patient registry', 'A type of observational study that collects information about patients’ medical conditions and/or treatments to better understand how a condition or treatment affects patients in the real world.', '‘Patient registry’ or ‘Observational patient registry’', true, 30),
       (14, 'Expanded access', 'A way for patients with serious conditions to receive a drug or treatment outside of a clinical trial. Used by Clinicaltrials.gov only', '‘EXPANDED_ACCESS’', true, 40),
       (15, 'Funded programme', 'Two or more studies funded and registered as a unit. Used in some WHO source registries.', '‘Funded programme’', true, 50),
       (16, 'Diagnostic test', 'Studies which are explictly labelled as being of a diagnostic test rather than a particular condition or intervention - only found in Chinese registry data.', '‘Diagnostic test’', true, 60),
       (98, 'Not applicable', 'Studies where the type is explicitly given as ‘not applicable’ or equivalent - only found in Chinese registry data.', '‘not applicable’, ‘N/A’', true, 70),
       (99, 'Other', 'Studies not falling within the other categories or ambiguous in terms of interventional / observational status.', null, true, 80),
       (0, 'Not provided', 'Studies where there is no provided value or a value inidcating type is unknown.', 'Study type is null, or ‘unknown’, ‘Not provided’, ‘Not specified’ or similar.', true, 90);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_time_units(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.time_units;
    CREATE TABLE lups.time_units (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.time_units(id, name, use_in_data_entry, list_order)
       values
        (11, 'Seconds', false, 10),
        (12, 'Minutes', false, 20),
        (13, 'Hours', false, 30),
        (14, 'Days', true, 40),
        (15, 'Weeks', true, 50),
        (16, 'Months', true, 55),
        (17, 'Years', true, 60),
        (0, 'Not provided', false, 99);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_title_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.title_types;
    CREATE TABLE lups.title_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        applies_to         varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.title_types (id, name, applies_to, description, 
          use_in_data_entry, list_order)
       values 
        (15, 'Public title', 'Study', 'In contrast to the full scientific title, usually from a trial registry - the default display name.', true, 10),
        (16, 'Registry scientific title', 'Study', 'Full scientific title, as quoted in a trial registry.', true, 20),
        (17, 'Protocol title', 'Study', 'Full scientific title, from a published protocol or similar source (e.g. the researchers themselves).', true, 30),
        (18, 'Other scientific title', 'Study', 'Full scientific title, not from the protocol or a registry entry. May be, for example, from a data repository catalogue.', true, 40),
        (14, 'Acronym or abbreviation', 'Study', 'As provided by study sponsors.', true, 50),
        (12, 'Subtitle', 'All', 'A subtitle provided by object creators or study sponsors.', true, 60),
        (13, 'Translated title', 'All', 'Used in conjunction with language code to indicate language translated into.', true, 70),
        (19, 'Journal article title', 'Data Object', 'Full journal title, as listed in citation.', true, 80),
        (20, 'Unique title, from sponsor name and doc ID', 'Data Object', 'Combination of sponsor abbreviation or name and a supplied document Id or label', true, 90),
        (21, 'Object title as provided in source', 'Data Object', 'A name provided for the data object, e.g  in a list - will often refer to its type and not be unique, but should be unique within the parent study', true, 100),
        (22, 'Object file name (without extension)', 'Data Object', 'The name provided for a named file, without the file extension.', true, 110),
        (23, 'Object name provided as type', 'Data Object', 'In some cases an object ‘type’ can be provided that appears more like a title than a generic type', false, 120),
        (24, 'Study scientific name :: object type', 'Data Object', 'Constructed using study full name to prefix object’s type. If obvious from context study name can be omitted.', false, 130),
        (25, 'Study registry ID :: object name', 'Data Object', 'Constructed using registry id to prefix a non unique name.  If obvious from context study ID can be omitted.', false, 140),
        (26, 'Study registry ID :: object type', 'Data Object', 'Constructed using registry id to prefix object’s type. If obvious from context study ID can be omitted.', false, 150),
        (27, 'Study registry ID :: object type :: Id', 'Data Object', 'Constructed using registry id to prefix object’s type and an Id. Used if multiple instances of object type exist (e.g. with sample collections)', false, 160),
        (28, 'Additional title / label (may not be unique)', 'Data Object', 'Any title or label applied, e.g. by the object’s creator or a storage infrastructure. Not necessarily unique.', true, 170),
        (90, 'Other alternative title', 'All', 'Any alternative title not described elsewhere.', true, 900),
        (0, 'Not yet known', 'All', 'Dummy value supplied by default on entity creation.', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_topic_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.topic_types;
    CREATE TABLE lups.topic_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
        value_type         varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.topic_types (id, name, description, value_type, 
          use_in_data_entry, list_order)
       values 
        (11, 'Keyword', 'Topic that was not categorised or does not fit into one of the categories listed. Often written by study or object creators.', 'Free-text or controlled vocabulary', false, 10),
        (12, 'Chemical / agent', 'One or more chemicals or biological agents, relevant to the study, including as interventions under test.', 'Free-text or controlled vocabulary', true, 15),
        (21, 'Device', 'Name of a medical  device', 'Free-text or controlled vocabulary', true, 17),
        (13, 'Condition', 'Illness or condition that is being targeted within study.', 'Free-text or controlled vocabulary', true, 20),
        (14, 'Design', 'Aspect of study design methodology.', 'Free-text or controlled vocabulary', true, 30),
        (15, 'Outcome', 'Outcome measure or outcome produced within the study.', 'Free-text or controlled vocabulary', true, 35),
        (16, 'Geographic', 'A geographical entity that was the particular focus or limit of the study.', 'Free-text or controlled vocabulary', true, 40),
        (17, 'Organism', 'Organism, e.g. particular bacterium, that was targeted during the study.', 'Free-text or controlled vocabulary', true, 45),
        (18, 'Treatment protocol', 'The name of a particular treatment regime / protocol, e.g. a chemtherapy regime', 'Free-text', true, 48),
        (19, 'Subject characteristics', 'Descriptive term pertaining to the subject group of the study', 'Free-text', true, 49),
        (20, 'Other', 'Cannot be easily categorised using the available topic types', 'Free-text or controlled vocabulary', true, 998),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', 'Not Applicable', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_topic_vocabularies(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.topic_vocabularies;
    CREATE TABLE lups.topic_vocabularies (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.topic_vocabularies (id, name, description, 
          use_in_data_entry, list_order)
       values
        (11, 'Authors', 'Free text, not intentionally part of any controlled vocabulary.', false, 10),
        (12, 'ICD 10', 'See https://icd.who.int/browse10/2016/en', true, 15),
        (13, 'ICD 11', 'See https://icd.who.int/browse11/l-m/en', true, 20),
        (14, 'MESH', 'See https://www.nlm.nih.gov/mesh/meshhome.html', true, 25),
        (15, 'SnoMed CT', 'See http://www.snomed.org/', true, 30),
        (16, 'MedDRA', 'See https://www.meddra.org/', true, 35),
        (17, 'NCI thesaurus', 'See https://ncit.nci.nih.gov/ncitbrowser/', true, 40),
        (18, 'Cochrane PICO terminology', 'See https://linkeddata.cochrane.org/pico-ontology', true, 45),
        (19, 'CDISC controlled terminology', 'See https://www.cdisc.org/standards/terminology', true, 50),
        (20, 'LOINC', 'See https://loinc.org/', true, 55),
        (21, 'ATC drug classification', 'See https://www.whocc.no/atc/structure_and_principles/', true, 60),
        (22, 'WHO Drug', 'See https://www.who-umc.org/whodrug/whodrug-portfolio/', true, 65),
        (29, 'CAS', 'CAS chemical registry identifier, see https://www.cas.org/cas-data/cas-registry', true, 68),
        (23, 'IUPAC chemical names', 'Includes biochemical names. See https://iupac.org/what-we-do/nomenclature/', true, 70),
        (24, 'InChI chemical identifier', 'See https://iupac.org/who-we-are/divisions/division-details/inchi/', true, 75),
        (25, 'Enzyme Commission numbers / names', 'See https://www.qmul.ac.uk/sbcs/iubmb/', true, 80),
        (26, 'HGNC human genome codes / names', 'Includes related proteins. See https://www.genenames.org/', true, 85),
        (27, 'Taxonomic names', 'For example, Linnaean binominals. See http://www.iczn.org/iczn/index.jsp', true, 90),
        (30, 'MESH Tree', 'The full tree code chains from the mesh system. See https://www.nlm.nih.gov/mesh/meshhome.html', true, 100),
        (28, 'None', 'No  controlled terminology used', true, 998),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_trial_registries(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.trial_registries;
    CREATE TABLE lups.trial_registries (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.trial_registries (id, name, description, list_order)
       values 
        (100120, 'ClinicalTrials.gov', '', 10),
        (100123, 'EU Clinical Trials Register', '', 20),
        (100126, 'ISRCTN', '', 30),
        (100118, 'Chinese Clinical Trial Register', '', 50),
        (100127, 'Japan Primary Registries Network', '', 60),
        (100116, 'Australian New Zealand Clinical Trials Registry', '', 70),
        (100124, 'Deutschen Register Klinischer Studien', '', 80),
        (100132, 'The Netherlands National Trial Register', '', 90),
        (100121, 'Clinical Trials Registry - India', '', 100),
        (100119, 'Clinical Research Information Service (South Korea)', '', 110),
        (100125, 'Iranian Registry of Clinical Trials', '', 120),
        (100117, 'Registro Brasileiro de Ensaios Clínicos', '', 130),
        (100129, 'Registro Peruano de Ensayos Clínicos', '', 140),
        (100122, 'Registro Público Cubano de Ensayos Clínicos', '', 150),
        (100128, 'Pan African Clinical Trial Registry', '', 160),
        (100130, 'Sri Lanka Clinical Trials Registry', '', 170),
        (100131, 'Thai Clinical Trials Register', '', 180),
        (101989, 'Lebenon Clinical Trial Registry', '', 190);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}
    

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

 
pub async fn create_study_identifier_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.study_identifier_types;
    CREATE TABLE lups.study_identifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        old_id             int4       NULL,
        name               varchar    NULL,
	    description        varchar    NULL,
        comments            varchar   NULL,
        examples           varchar    NULL,
        regexps            varchar    NULL,
        source_org_id      int4       NULL,
        source_org         varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.study_identifier_types (id, old_id, name, description, 
          comments, examples, regexps, source_org_id, source_org, use_in_data_entry, list_order)
       values 
        (100,11,'Trial registry ID (registry unspecified)','Used in the original MDR as a blanket identifier, simply indicating that the id was from a trial registry',
        'Use now DEPRECATED. In most cases a trial registry ID’s format and context should allow the specific registry to be identified.','n/a','n/a',0,'n/a',false, 990),
        (115,0,'WHO universal trial ID','Number provided by WHO as the Universal Trial Number or UTN.  Unfortunately only a relatively small proportion of studies have it.',
        'The initial U is often missing in source data, so regexp to identify the Id better without the U','U1111-1131-4384, U1111-1130-8248, U1111-1132-3386','1111-[0-9]{4}-[0-9]{4}',100114,'World Health Organisation',true,115),
        (116,0,'Australian New Zealand CTR ID','Number provided by the ANZCTR.','','ACTRN12605000057684, ACTRN12622001013752','ACTRN[0-9]{14}',100690,'National Health and Medical Research Council, Australia',true,116),
        (117,0,'Brazilian CTR ID','Number assigned by ReBEC (Registro Brasileiro de Ensaios Clínicos)','Appears to be run by part of the Instituto Oswaldo Cruz, the Information Institute of the Oswaldo Cruz Foundation (ICICT/Fiocruz) ','RBR-2b9qf7h, RBR-8g2q8q5, RBR-972497','RBR-[0-9a-z]{6, 8}',109251,'Instituto Oswaldo Cruz',true,117),
        (118,0,'Chinese CTR ID','Number applied by the Chinese trial registry. N.B. Older numbers have a different format (with additional 3 or 4 letters).','Registry originated and is still based at West China Hospital. Not clear if it is till run by them, on behalf of central authorities.','ChiCTR1800014249, ChiCTR2300076215. Older numbers have a different format (with additional 3 or 4 letters): ChiCTR-OCS-13003526, ChiCTR-ROC-17011223, ChiCTR-TTRCC-14004829','ChiCTR[0-9]{10}, ChiCTR-([A-Z]{3}|[A-Z]{4})-[0-9]{8}',100494,'West China Hospital, Sichuan University',true,118),
        (119,0,'Korean CTR ID','KCT number applied by the Korean registry, the Clinical Research Information Service (CRIS).','** source org needs to be added to system **','KCT0004176, KCT0008933','KCT[0-9]{7}',0,'Korea Disease Control and Prevention Agency',true,119),
        (120,0,'NCT ID','NCT number assigned by the US ClinicalTrials.gov system',null,'NCT00470054, NCT04970069','NCT[0-9]{8}',100133,'National Library of Medicine',true,120),
        (121,0,'Indian CTR ID','CTRI number assigned by the Indian Clinical Trial Registry','To avoid issues with the ‘/’ character in the IDs (especially on Windows machines) these are replaced by ‘-’ within the MDR system.','CTRI-2007-091-000012, CTRI-2020-12-029986, CTRI-2024-07-070268','CTRI/20[0-9]{2}/[0-9]{2,3}/[0-9]{6} ',102044,'Indian Council of Medical Research',true,121),
        (122,0,'Cuban CTR ID','Assigned by the Registro Público Cubano de Ensayos Clínicos (RPCEC)','** source org needs to be added to system **','RPCEC00000139, RPCEC00000446','RPCEC[0-9]{8}',0,'The National Coordinating Center of Clinical Trials (CENCEC)',true,122),
        (123,0,'EMA Eudract ID','EU-CTR number assigned by EMA. Ceased to provide new numbers January 2023.',null,'2004-000119-24, EUCTR2015-003324-32, 2023-000946-41','20[0-9]{2}-0[0-9]{5}-[0-9]{2}, often preceded with ‘EU’ or ‘EUCTR’',100159,'European Medicines Agency',true,123),
        (124,0,'German CTR ID','DRKS number assigned by German CTR',null,'DRKS00018960, DRKS00031743','DRKS[0-9]{8}',105875,'BfArM (Federal Institute for Drugs and Medical Devices)',true,124),
        (125,0,'Iranian CTR ID','IRCT number assigned by Iranian CTR','** source org needs to be added to system ** (it is in ROR)','IRCT138706211241N1, IRCT20111224008505N47, IRCT20220212054002N3, IRCT201307145362N6','IRCT[0-9]{11,14}N[0-9]{1,2}',0,'Ministry of Health and Medical Education (MOHME)',true,125),
        (126,0,'ISRCTN ID','Number provided by ISRCTN. Run as a non-profit department within Springer Nature. High proportion of UK originated trials.',null,'ISRCTN17973604, ISRCTN59618689, ISRCTN98252311','ISRCTN[0-9]{8}',101421,'Springer Nature',true,126),
        (127,0,'Japanese CTR ID','Number provided by one of: Japan Registry of Clinical Trials (JRCT); University Hospital Medical Information Network Center  (UMIN-CTR), Japan Pharmaceutical Information Center (JAPIC), Japan Medical Association Center for Clinical Trials (JMACCT), (may be differently formatted or prefixed to reflect origin)','** source org needs to be added to system ** (it is in ROR)','JPRN-C000000079, JPRN-jRCT1040240030, JPRN-jRCT2080223938, JPRN-UMIN000004891, JPRN-UMIN000047941','JPRN-C[0-9]{9}, JPRN-jRCT[0-9]{9}, JPRN-UMIN[0-9]{9}. The C[0-9]{9} format also appears to be UMIN. The initial JPRN- is often omitted in data sources.',0,'National Institute of Public Health',true,127),
        (128,0,'Pan African CTR ID','PACTR number assigned by Pan African registry','** source org needs to be added to system **','PACTR201006000222401, PACTR202108841661192, PACTR2010030001971409','PACTR[0-9]{15, 16}',0,'Cochrane South Africa',true,128),
        (129,0,'Peruvian CTR ID','REPEC ID assigned by Registro Peruano de Ensayos Clínicos','** source org needs to be added to system **','PER-002-21, PER-040-07, PER-120-07','PER-[0-9]{3}-[0-9]{2}',0,'National Institute of Health',true,129),
        (130,0,'Sri Lankan CTR ID','Sri Lankan SLCTR number assigned by Sri Lankan registry.','To avoid issues with the ‘/’ character in the IDs (especially on Windows machines) these are replaced by ‘-’ within the MDR system.','SLCTR/2010/008, SLCTR/2018/002, SLCTR/2024/037','SLCTR/20[0-9]{2}/[0-9]{3}',0,'Sri Lanka Clinical Trials Registry',true,130),
        (131,0,'Thai CTR ID','TCTR number assigned by Thai Clinical Trials Registry','** source org needs to be added to system **','TCTR20110000022, TCTR20210709003, TCTR20241115007','TCTR20[0-9]{9}',0,'Thai Clinical Trials Register',true,131),
        (132,0,'Dutch CTR ID','As supplied by OMON (Overview of Medical Research in the Netherlands), formerly (pre-2025) by the Nederlands National Trial Register. Number has also changed format as a result','** source org needs to be added to system as new version of CCM0 **','NL-OMON36374, NL-OMON54196, NL-OMON36845 Previously NL numbers. Previously to that NTR numbers.','NL-OMON[0-9]{5}. Previously NL[0-9]{1,5}. Previously to that NTR[0-9]{1,5}',0,'Netherlands National Trial Register',true,132),
        (133,0,'Lebanese CTR ID','LBCTR number assigned bu Lebanese Clinical Trial Registry','** source org needs to be added to system ** (it is in ROR)','LBCTR2019020191, LBCTR2022074978, LBCTR2024045330','LBCTR20[0-9]{8}',0,'Lebanon Clinical Trial Registry',true,133),
        (134,0,'ITM CTR ID','ITMCTR ID provided by the International Traditional Medicine Clinical Trials Registry (in fact almost entirely Chinese Traditional Medicine)','** source org needs to be added to system **','ITMCTR1900002281, ITMCTR2022000094, ITMCTR2200006530','ITMCTR[0-9]{10}',102245,'China Academy of Chinese Medical Sciences',true,134),
        (135,0,'EMA CTIS ID','EU-CTIS number assigned by EMA. Went live in 2024 after extensive testing period.',null,'CTIS2022-500024-30-00, 2022-500024-30-00, CTIS2025-520964-16-00','20[2|3][0-9]-5[0-9]{5}-[0-9]{2}, often preceded by CTIS',100159,'European Medicines Agency',true,135),
        (136,0,'Chinese CCRBCTR ID','ID that was assigned by the Centre for Clinical Research and Biostatistics (Chinese University of Hong Kong) - now subsumed withion the main Chinese Registry.',null,null,null,0,null,true,0), 
        (137,0,'Chinese Acupuncture Registry ID','ID that was assigned by the The Acupuncture-Moxibustion Clinical Trial Registry - now subsumed within the ITMCTR',null,null,null,0,null,true,0), 
        (138,0,'JMA CCT ID','ID that was assigned by the Japan Medical Association Center for Clinical Trials - now incorporated into the Japan Registry of Clinical Trials (JRCT)',null,null,null,0,null,true,0), 
        (139,0,'Japic ID','ID that was assigned by the Japan Pharmaceutical Information Center - now incorporated into the Japan Registry of Clinical Trials (JRCT)',null,null,null,0,null,true,0), 
        (140,0,'jRCT ID','ID assigned by the Japan Registry of Clinical Trials (JRCT)',null,null,null,0,null,true,0), 
        (141,0,'UMIN ID','ID assigned by the University Hospital Medical Information Network (UMIN) in Japan',null,null,null,0,null,true,0), 
        (142,0,'RENIS ID','ID assigned by the Argentinian Registro Nacional de Investigaciones en Salud',null,null,null,0,null,true,0), 
        (143,0,'Indonesian CTR ID','ID assigned by the Indonesian Clinical Research registry',null,null,null,0,null,true,0), 
        (144,0,'Jordanian CTR ID','ID assigned by the Jordanian Clinical Research registry',null,null,null,0,null,true,0), 
        (145,0,'Malaysian CTR ID','ID assigned by the Malaysian Clinical Research registry',null,null,null,0,null,true,0), 
        (146,0,'Nepalese CTR ID','ID assigned by the Nepalese Clinical Research registry',null,null,null,0,null,true,0), 
        (147,0,'Nigerian CTR ID ','ID assigned by the Nigerian Clinical Research registry - Placeholder - Registry does not appear to be active',null,null,null,0,null,true,0), 
        (148,0,'Pakistani CTR ID','ID assigned by the Pakistani Clinical Research registry - Placeholder - Registry does not appear to be active',null,null,null,0,null,true,0), 
        (149,0,'Philipino PHRR ID','ID assigned by the Philipino Clinical Research registry',null,null,null,0,null,true,0), 
        (150,0,'Russian CTR ID','ID assigned by the Russian State Register of Medicines',null,null,null,0,null,true,0), 
        (151,0,'Saudi CTR ID','ID assigned by the Saudi Arabian Clinical Research registry - Placeholder - Registry does not appear to be active',null,null,null,0,null,true,0), 
        (152,0,'Singaporean CTR ID','ID assigned by the Singaporean Clinical Research registry',null,null,null,0,null,true,0), 
        (153,0,'South African CTR ID','ID assigned by the South African National Clinical Research registry',null,null,null,0,null,true,0), 
        (154,0,'Spanish Registry ID','ID assigned by the Registro Español de Estudios Clínicos',null,null,null,0,null,true,0), 
        (155,0,'Tanzanian CTR ID','ID assigned by the Tanzanian Clinical Research registry',null,null,null,0,null,true,0), 
        (156,0,'HKU CTR ID','ID assigned by the Hong Kong University  Clinical Research registry',null,null,null,0,null,true,0), 
        (160,0,'BioLINCC ID','A system derived ID, based on the study''s acronym, for studies within the BioLINCC DB.',null,null,null,100167,'National Heart Lung and Blood Institute',true,150),
        (161,0,'Yoda ID','A system derived ID, based on the corresponding NCT ID, or occasionally the protocol ID, for studies within the Yoda DB.','The Yoda DB does not have a persistent and unique ID for studies',null,null,100197,'Yale University',true,151),
        (162,42,'NHLBI BioLINCC ID','Accession number, from NHLBI BioLINCC study database','Does not appear to be consistent over time, therefore cannot be used as an PID.',null,null,100167,'National Heart Lung and Blood Institute',true,160),
        (170,49,'NCI PDQ ID','Id used in NCI’s Physician Data Query system (normally CDR followed by 10 digits)',null,null,null,0,null,false,170),
        (171,51,'AHRQ ID','ID assigned by the US Healthcare Research and Quality Agency',null,null,null,0,null,false,171),
        (172,52,'NIAID CRMS Id','ID used for the study within the US NIAID Clinical Research Management System',null,null,null,0,null,true,172),
        (173,40,'DAIDS-ES registry ID','ID used within the US NIH NAID Division of AIDS Enterprise System (DAIDS-ES).',null,null,null,0,null,false,173),
        (174,39,'NCI CTRP ID','ID generated by the US NCI for supported cancer trials (in the Clinical Trial Reporting Program), usually NCI- followed by year followed by -5 digits',null,null,null,0,null,false,174),
        (175,46,'CTEP Number','ID allocated by the US Cancer Therapy Evaluation program',null,null,null,0,null,false,175),
        (176,0,'EORTC number','Identifier assigned by the European Organisation for Research into Cancer',null,null,null,0,null,true,0), 
        (177,0,'CDC identifier','Identifier assigned by the US Centers for Disease Control and Prevention (context / purpose not always clear)',null,null,null,0,null,true,0), 
        (178,0,'NCI identifier','Identifier assigned by the US National Cancer Institute (context / purpose not always clear)',null,null,null,0,null,true,0), 
        (179,0,'Malformed registry Id','A claimed trial registry identifier that has been wrongly formatted (e.g. too few or too many digits).',null,null,null,0,null,true,0), 
        (180,44,'Obsolete NCT ID','Obsolete NCT number, sometimes referred to as ’NCT alias’','A few thousand NCT numbers were replaced by new numbers in the early years of the system - reason is unclear.',null,null,100133,'National Library of Medicine',false,180),
        (181,45,'Obsolete NTR ID','Obsolete NTR number (Dutch registry only)',null,null,null,0,null,false,181),
        (182,0,'Obsolete NL ID','Obsolete NL number (Dutch registry only)',null,null,null,0,null,true,0), 
        (183,0,'FDA Orphan Drug ID','ID of the trial within the US Food and Drug Administration Orphan Drug programme.',null,null,null,0,null,true,0), 
        (184,0,'FDA IND / IDE number','IND ID, as provided by the CDER department within the  US Food and Drug Administration.',null,null,null,0,null,true,0), 
        (185,0,'FDA identifier','ID labelled as an FDA identifier, exact context or purpose unclear.',null,null,null,0,null,true,0), 
        (186,0,'Eudamed ID',null,null,null,null,0,null,true,0), 
        (187,0,'HMA-EMA RWD (EUPAS) ID',null,null,null,null,0,null,true,0), 
        (188,0,'University of Basel CTU ID',null,null,null,null,0,null,true,0), 
        (189,0,'National Taiwan University Hospital ID',null,null,null,null,0,null,true,0), 
        (190,0,'Cancer & Leukemia GrpB ID (US)','Cancer and Leukemia Group B  (US)',null,null,null,0,null,true,0), 
        (191,0,'Radiation Onc. Group ID (US)','Radiation Therapy Oncology Group (US)',null,null,null,0,null,true,0), 
        (192,0,'Gynae Onc. Group ID (US)','Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (193,0,'South West Onc. Group ID (US)','South West Oncology Group',null,null,null,0,null,true,0), 
        (194,0,'Korean SW Onc. Group ID','Korean South West Oncology Group',null,null,null,0,null,true,0), 
        (195,0,'China Southern Assoc. for Clinical Oncology ID','China Southern Association for Clinical Oncology',null,null,null,0,null,true,0), 
        (196,0,'Central European Cooperative Onc. Group ID','Central European Cooperative Oncology Group',null,null,null,0,null,true,0), 
        (197,0,'American College of Surgeons Onc. Group ID','American College of Surgeons Oncology Group ',null,null,null,0,null,true,0), 
        (198,0,'North Central Cancer Treatment Group  ID (US)','North Central Cancer Treatment Group',null,null,null,0,null,true,0), 
        (199,0,'Eastern Cooperative Onc. Group ID (US)','Eastern Cooperative Oncology Group',null,null,null,0,null,true,0), 
        (200,0,'American College of Radiology Imaging Network ID','American College of Radiology Imaging Network',null,null,null,0,null,true,0), 
        (201,0,'ECOG-ACRIN ID (US)','Eastern Cooperative Oncology Group / American College of Radiology Imaging Network',null,null,null,0,null,true,0), 
        (202,0,'New Approaches to Brain Tumor Therapy ID (US)','New Approaches to Brain Tumor Therapy',null,null,null,0,null,true,0), 
        (203,0,'North American Brain Tumor Consortium ID','North American Brain Tumor Consortium ',null,null,null,0,null,true,0), 
        (204,0,'National Cancer Institute of Canada ID','NCIC Clinical Trials Group (Canada)',null,null,null,0,null,true,0), 
        (205,0,'Chinese Radiation Therapy Onc. Group ID','Chinese Radiation Therapy Oncology Group',null,null,null,0,null,true,0), 
        (206,0,'Borstkanker Onderzoek Groep ID','Borstkanker Onderzoek Groep (Dutch Breast Cancer Research Group)',null,null,null,0,null,true,0), 
        (207,0,'Trans Tasman Radiation Onc. Group ID','Trans Tasman Radiation Oncology Group ',null,null,null,0,null,true,0), 
        (208,0,'Latin American Cooperative Onc. Group  ID','Latin American Cooperative Oncology Group (formerly GLICO)',null,null,null,0,null,true,0), 
        (209,0,'Australia NZ Gynaecological Onc. Group ID','Australia New Zealand Gynaecological Oncology Group',null,null,null,0,null,true,0), 
        (210,0,'National Surgical Adjuvant Breast and Bowel Project ID','National Surgical Adjuvant Breast and Bowel Project',null,null,null,0,null,true,0), 
        (211,0,'John Hopkins Cancer Center ID','Sidney Kimmel Comprehensive Cancer Center at Johns Hopkins',null,null,null,0,null,true,0), 
        (212,0,'John Hopkins - NABTT  collab. ID','John Hopkins Cancer Center / New Approaches to Brain Tumor Therapy project collaboration',null,null,null,0,null,true,0), 
        (213,0,'City of Hope NMC ID','City of Hope National Medical Center',null,null,null,0,null,true,0), 
        (214,0,'Japanese Gynae Group ID','Japanese Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (215,0,'Korean Gynae Group ID','Korean Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (216,0,'Shanghai Gynae Group ID','Shanghai Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (217,0,'Taiwanese Gynae Group ID','Taiwanese Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (218,0,'New York Gynae Group ID','New York Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (219,0,'Chinese GI Onc. Group ID','Chinese Gastrointestinal Oncology Group ',null,null,null,0,null,true,0), 
        (220,0,'Ad Hoc Collaboration ID','Identifier indicating ad hoc collaboration of research organisations',null,null,null,0,null,true,0), 
        (221,0,'Belgium and Luxembourg Gynael Onc. Group ID','Belgium and Luxembourg Gynaecological Oncology Group',null,null,null,0,null,true,0), 
        (222,0,'Central and E European Gynae Onc. Group ID','Central and Eastern European Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (223,0,'European Network of Gynaec Onco. Trial Groups ID','European Network of Gynaecological Oncological Trial Groups',null,null,null,0,null,true,0), 
        (224,0,'Chongqing Gynae Group ID','Chongqing Gynecologic Oncology Group',null,null,null,0,null,true,0), 
        (225,0,'NRG Oncology ID','NRG Oncology (US)',null,null,null,0,null,true,0), 
        (300,41,'Regulatory body ID (unspecified)','Number generated by a national authority when a study applies for authorisation / ethical approval, e.g. IRAS number in the UK, ANSM number in France',null,null,null,0,null,true,300),
        (301,0,'ANSM (ID-RCB number)','Clinical Trial Authorisation Number provided by the French Regulatory Authority',null,null,null,0,null,true,0), 
        (302,0,'ChinaDrugTrial ID','Appears to be an authorisation number provided by the ChinaDrugTrials government organisation and web site',null,null,null,0,null,true,0), 
        (399,48,'Agency Reference ID (agency unspecified)','Reference or Protocol ID assigned by a Government agency (e.g. NIH institute), not covered elsewhere',null,null,null,0,null,false,399),
        (400,13,'Funder / Contract ID (unspecified)','May be a grant id or contract, org id should identify the funder.',null,null,null,0,null,true,400),
        (401,0,'NIH grant ID','Funding ID from the US National Institutes of Health, normally reported on NIH web page',null,null,null,100134,'National Institutes of Health',true,401),
        (402,0,'AHRQ Grant ID','Funding ID ostensibly from the US Agency for Health Research and Quality, though normally reported on NIH web page',null,null,null,0,null,true,0), 
        (403,0,'FDA Grant ID','Funding ID ostensibly from the US Food and Drug Administration, though normally reported on NIH web page',null,null,null,0,null,true,0), 
        (404,0,'SAMHSA Grant ID','Funding ID ostensibly from the US Substance Abuse and Mental Health Services Administration though normally reported on NIH web page',null,null,null,0,null,true,0), 
        (405,0,'US Department of Defense Grant ID','Funding ID from the US Department of Defense',null,null,null,0,'US Department of Defense',true,0), 
        (406,0,'CDC Grant ID','Grant id of award from the US Centers for Disease Control and Prevention',null,null,null,0,'Centers for Disease Control and Prevention',true,0), 
        (407,0,'NCI Grant ID','Grant id of award from the US National Cancer Institute (part of NIH).',null,null,null,0,'National Cancer Institute',true,0), 
        (410,0,'CRUK ID','Funder''s ID from Cancer Research UK',null,null,null,0,null,true,0), 
        (411,0,'Dutch ZonMW Grant ID','Funding ID from the Netherlands Organisation for Health Research and Development',null,null,null,0,null,true,0), 
        (412,0,'NSTC (Taiwan) Grant ID','National Science and Technology Council Taiwan Grant ID',null,null,null,0,'National Science and Technology Council Taiwan',true,0), 
        (413,0,'Ministry of Health and Welfare Taiwan Grant ID',null,null,null,null,0,null,true,0), 
        (414,0,'Ministry of Science and Technology Taiwan Grant ID',null,null,null,null,0,null,true,0), 
        (415,0,'Department of Health Taiwan Grant ID',null,null,null,null,0,null,true,0), 
        (500,14,'Sponsor’s ID','Usually a protocol ID. Org id should identify the sponsor.',null,null,null,0,null,true,450),
        (502,0,'Sponsor''s id (presumed)',null,null,null,null,0,null,true,452),
        (600,50,'Research Collaboration ID','ID provided by a research collaborative group, usualluy a trials group',null,null,null,0,null,true,600),
        (605,43,'CTMS ID','ID as used by a clinical trial management or data management system',null,null,null,0,null,true,605),
        (701,0,'Eli Lilly Study ID','Eli Lilley identifier (protocol ID usually)',null,null,null,0,null,true,0), 
        (702,0,'InCyte Study ID','InCyte identifier - often includes reference to the code of the drug being studies',null,null,null,0,null,true,0), 
        (703,0,'Novartis study ID','Novartis identifier - in most cases ’C’ followed by 3 letters and then an alphanumeric string representing the drug code.',null,null,null,0,null,true,0), 
        (704,0,'Alcon Research study ID','Alcon research identifier - quite varied in form.','At one time Alcon was part of Novartis',null,null,0,'Alcon Research',true,0), 
        (705,0,'Pfizer ID',null,null,null,null,0,'Pfizer',true,0), 
        (706,0,'GSK ID',null,null,null,null,0,'Glaxo Smith Kline',true,0), 
        (707,0,'Roche ID',null,null,null,null,0,'Roche',true,0), 
        (708,0,'Astra Zeneca ID',null,null,null,null,0,'Astra Zeneca',true,0), 
        (709,0,'Takeda ID',null,null,null,null,0,'Takeda',true,0), 
        (710,0,'Johnson and Johnson ID',null,null,null,null,0,'Johnson and Johnson',true,0), 
        (711,0,'Jannsen ID',null,null,null,null,0,'Jannsen',true,0), 
        (712,0,'Sanofi-Aventis ID',null,null,null,null,0,'Sanofi-Aventis',true,0), 
        (713,0,'BMS ID',null,null,null,null,0,'Bristol Myers Squib',true,0), 
        (714,0,'AbbVie ID',null,null,null,null,0,'AbbVie Pharmaceuticals',true,0), 
        (715,0,'Bayer ID',null,null,null,null,0,'Bayer',true,0), 
        (800,12,'Ethics review ID (unspecified)','Org id should identify the review body.',null,null,null,0,null,true,350),
        (801,0,'CCMO ID','Dutch Ethics approval ID, as issued by CCMO',null,null,null,0,'CCMO',true,0), 
        (802,0,'Swiss BASEC ID','Swiss ethics ID, from the SwissEthics system ',null,null,null,0,null,true,0), 
        (990,0,'Other Id (provenance not supplied)','ID supplied as an additional Id but with no information as to type or source. Often so short or simple as to make matching to known ID types too ambiguous to be useful.',null,null,null,0,null,true,0);"#;
            
  

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}



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
    

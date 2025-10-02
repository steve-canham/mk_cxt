use crate::err::AppError;
use sqlx::{Pool, Postgres};



pub async fn create_dataset_consent_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.dataset_consent_types;
    CREATE TABLE lups.dataset_consent_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.dataset_consent_types (id, name, description, list_order)
       values 
         (0, 'Not known', 'Information about consent for secondary use unavailable', 10),
         (1, 'No explicit consent', 'No specific consent was given for the sharing of data or its re-use beyond the study in which it was originally collected.', 20),
         (2, 'No restriction', 'No restriction explicitly stated in the consent documents, OR a broad consent to re-use is present without qualification.', 30),
         (3, 'General research use', 'Consent indicates that use is allowed for general research use for any research purpose. From the DUO.', 40),
         (4, 'Health / medical / biomedical research', 'Consent indicates that use is allowed for health/medical/biomedical purposes; does not include the study of population origins or ancestry, or the development of methods / algorithms (e.g. for ML). From the DUO.', 50),
         (5, 'Disease-specific research', 'Consent indicates that use, for health/medical/biomedical research is allowed provided it is related to a specified disease (area). The disease (area) must be named or coded in the associated comments field. From the DUO.', 60),
         (6, 'Consent specified, not elsewhere categorised', 'A descriptive statement regarding consent is available, but it does not fit into categories 1 - 5', 70);"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_dataset_deidentification_levels(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.dataset_deidentification_levels;
    CREATE TABLE lups.dataset_deidentification_levels (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
        
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.dataset_deidentification_levels (id, name, description, list_order)
       values 
        (0, 'Not known', 'No clear information available about the de-identification, if any, applied to the data', 10),
        (1, 'No de-identification', 'Confirmed that no de-identification measures have been applied to the data set.', 20),
        (2, 'De-identification applied', 'Some de-identification measures have been applied. Details should be described in comments and / or indicated in the linked boolean fields, or in separate documents.', 30),
        (3, 'De-identification applied, primary outcomes re-assessed', 'Some de-identification measures have been applied and are described. In addition the data has been re-analysed against the primary outcomes and the results described.', 40);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_dataset_recordkey_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.dataset_recordkey_types;
    CREATE TABLE lups.dataset_recordkey_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.dataset_recordkey_types (id, name, description, list_order)
       values 
        (0, 'Not known', 'No clear information available about the record keys in use.', 10),
        (2, 'Anonymised', 'Data controller or manager describes dataset as ‘anonymised’, in their interpretation of the term.', 20),
        (3, 'Pseudonymised', 'Data controller or manager describes dataset as ‘pseudonymised’, in their interpretation of the term.', 30),
        (4, 'Identifiable', 'Data controller or manager describes dataset as ‘identifiable’, in their interpretation of the term.', 40);"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())

}


pub async fn create_date_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.date_types;
    CREATE TABLE lups.date_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    on_papers_only     bool       NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.date_types (id, name, description, on_papers_only,
          use_in_data_entry, list_order)
       values 
        (15, 'Created', 'The date the resource itself was put together; this could be a date range or a single date for a final component, e.g., the finalised file with all of the data (from DataCite).', false, true, 10),
        (11, 'Accepted', 'The date that the publisher accepted the resource into their system. To indicate the start of an embargo period, use Submitted or Accepted, as appropriate (from DataCite)', false, false, 12),
        (12, 'Available', 'The date the resource is made publicly available. May be a range. To indicate the end of an embargo period, use Available (from DataCite).', false, true, 15),
        (51, 'Controlled access in force', 'The date the resource is made available under controlled access of some form. May be a range. To indicate the end of such a period, use Available.', false, false, 18),
        (13, 'Copyrighted', 'The specific, documented date at which the resource receives a copyrighted status, if applicable (from DataCite).', false, false, 20),
        (14, 'Collected', 'The date or date range in which the resource content was collected. To indicate precise or particular timeframes in which research was conducted (from DataCite).', false, true, 25),
        (16, 'Issued', 'The date that the resource is published or distributed e.g. to a data centre (from DataCite).', false, false, 35),
        (17, 'Submitted', 'The date the creator submits the resource to a publisher. This could be different from the Accepted date if the publisher then applies a selection process. Recommended for discovery (from DataCite).', false, true, 40),
        (18, 'Updated', 'The date of the last update to the resource, when the resource is being added to. May be a range (from DataCite).', false, true, 45),
        (19, 'Valid', 'The date or date range during which the dataset or resource is accurate (from DataCite).', false, false, 50),
        (52, 'Pubmed citation created', 'The date in the Pubmed XML, in the element ‘DateCreated’.', true, false, 52),
        (53, 'Pubmed citation revised', 'The date in the Pubmed XML, in the element ‘DateRevised’.', true, false, 53),
        (54, 'Pubmed citation completed', 'The date in the Pubmed XML, in the element ‘DateCompleted’.', true, false, 54),
        (55, 'E-published', 'Date of electronic publication, from Pubmed.', true, false, 55),
        (56, 'P-published', 'Date of print publication, from Pubmed.', true, false, 56),
        (57, 'Revised', 'Date an article was revised in publication by the authors, from Pubmed.', true, false, 57),
        (58, 'E-published ahead of print', 'Date an article was published electronically, ahead of print publication (PubMed date category).', true, false, 58),
        (59, 'Retracted', 'Date the publisher retracted an article or a resource, from Pubmed..', true, false, 59),
        (60, 'Added to e-collection', 'Date an article was included in an electronic collection (similar to an issue), from Pubmed.', true, false, 60),
        (61, 'Added to PMC', 'Date article was added to PMC, from Pubmed.', true, false, 61),
        (62, 'Added to Pubmed', 'Date the citation was added to PubMed, unless the citation is added to PubMed more than twelve months since the date of publication. In that case, the PubMed date is set to the date of publication.', true, false, 62),
        (63, 'Added to Medline', 'Date the citation completed Medline processing. Up until the citation has been indexed for Medline the medline date is the same as the entrez date.', true, false, 63),
        (64, 'PMC embargo release', 'Date a full-text article was released from embargo in PubMed Central (PMC).', true, false, 64),
        (65, 'Added to entrez', 'Date when pubmed entry entered into the e-utils Entrez system.', true, false, 65),
        (90, 'Other', 'Date type not defined elsewhere.', false, true, 90),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, false, 99);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())

}


pub async fn create_description_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.description_types;
    CREATE TABLE lups.description_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.description_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (20, 'Summary description', 'A textual summary of the resource (data, document or AV) and its contents', true, 5),
        (11, 'Abstract', 'A complete abstract or summary from a document.', true, 10),
        (16, 'Abstract section', 'A labelled section in a structured abstract (from Pubmed).', true, 12),
        (17, 'External abstract', 'Abstract not written by the authors, e.g. in translation (from Pubmed).', false, 14),
        (12, 'Methods', 'A description of how the data or other object type was constructed (from DataCite).', true, 15),
        (13, 'Series information', 'Information about a repeating series, such as volume, issue, number (from DataCite).', false, 20),
        (14, 'Table of contents', 'A listing of the Table of Contents (from DataCite).', true, 30),
        (15, 'Technical information', 'Detailed information that may be associated with design, implementation, operation, use, and/or maintenance of a process or system. Includes population information for a clinical research dataset (from DataCite).', true, 35),
        (18, 'Journal source string', 'The bibliographic reference to a journal article - the string that normally follows authors and title.', true, 40),
        (19, 'Data availability description', 'A description of the location of a Dryad dataset (from Pubmed)', false, 45),
        (90, 'Other', 'A description not falling into any of the other categories.', true, 90),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 99);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}
 

 
pub async fn create_object_access_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.object_access_types;
    CREATE TABLE lups.object_access_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.object_access_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (12, 'Public on-screen access', 'Completely open access on the web but content not available in any other format', false, 10),
        (20, 'Public on-screen and API access', 'Completely open access, material viewable and also available through an API', false, 12),
        (11, 'Public on-screen access and download', 'Completely open access, material viewable and also directly available as a file download', true, 14),
        (13, 'Public download (self-attestation required)', 'Public and downloadable (only) once the user identifies and / or describes themselves.', false, 20),
        (14, 'Public on-screen access (self-attestation required)', 'Public and viewable (only) once the user identifies and / or describes themselves.', false, 25),
        (15, 'Restricted download', 'The user is an authenticated member of a defined group and can view / download the material (includes pay-walled journal articles).', true, 30),
        (16, 'Restricted on-screen access', 'The user is an authenticated member of a defined group but can only view the material. Analysis tools may be available.', true, 35),
        (17, 'Case by case access', 'Based on a review of an individual request, often also requiring supporting documentation.', true, 40),
        (18, 'Case by case on-screen access', 'Based on a review of an individual request, often also requiring supporting documentation. Analysis tools may be available.', true, 45),
        (19, 'Non public access - no details', 'The site asserts that the resource is not publicly available but provides no further details.', false, 50),
        (90, 'Other', 'None of the listed options.', true, 90),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 99);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_object_classes(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.object_classes;
    CREATE TABLE lups.object_classes (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.object_classes (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (23, 'Text', 'A resource consisting primarily of words for reading, includes grey literature, lab notes, accompanying materials as well as published articles.', true, 5),
        (12, 'Collection', 'An aggregation of resources, which may encompass collections of one resource type as well as those of mixed types. A collection is described as a group; its parts may also be separately described, e.g. a collection of samples, or various files making up a report.', false, 15),
        (14, 'Dataset', 'Data encoded in a defined structure. A data file or files.', true, 25),
        (16, 'Image', 'A visual representation other than text, e.g. digitised or born digital images, drawings or photographs. ', true, 30),
        (15, 'Event', 'A non-persistent, time based occurrence. Descriptive information and/or content that is the basis for discovery of the purpose, location, duration, and responsible agents associated with an event such as a webcast or convention.', false, 30),
        (11, 'Audiovisual', 'A series of visual representations imparting an impression of motion when shown in succession. May or may not include sound. May be used for films, video, etc.', true, 35),
        (22, 'Sound', 'A resource primarily intended to be heard, e.g. an audio recording.', true, 38),
        (21, 'Software', 'A computer program in source code (text) or compiled form. Use this type for all software components supporting scholarly research.', true, 40),
        (18, 'Model', 'An abstract, conceptual, graphical, mathematical or visualization model that represents empirical objects, phenomena, or physical processes, e.g. modelled descriptions of different aspects of languages or a molecular biology reaction chain.', true, 45),
        (19, 'Physical object', 'An inanimate, three dimensional object or substance, e.g. artefacts, specimens.', false, 50),
        (20, 'Service', 'An organized system of apparatus, appliances, staff, etc., for supplying some function(s) required by end users, e.g. a data management service, or long term preservation service.', false, 55),
        (24, 'Workflow', 'A structured series of steps which can be executed to produce a final outcome, allowing users a means to specify and enact their work in a more reproducible manner.', false, 75),
        (17, 'Interactive resource', 'A resource requiring interaction from the user to be understood, executed, or experienced, e.g. training modules, files that require use of a viewer, or query/response portals.', true, 90),
        (13, 'Data paper', 'A factual and objective publication with a focused intent to identify and describe specific data, sets of data, or data collections to facilitate discoverability. A data paper describes data provenance and methodologies used in the gathering, processing, organizing, and representing the data. ', false, 120),
        (90, 'Other', 'Object class not listed elsewhere.', true, 998),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_object_identifier_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.identifier_types;
    CREATE TABLE lups.identifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.identifier_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (53, 'BBMRI Collection Id', 'The Id assigned by BBMRI to a collection of samples', false, 60),
        (15, 'Repository accession number', 'Org id should identify the repository manager.', false, 105),
        (16, 'PMID', 'Citation ID assigned by PubMed.', true, 110),
        (31, 'PMCID', 'Pub med central ID, of the manuscript itself.', true, 115),
        (32, 'NIH manuscript ID', 'A Manuscript ID, an identifier assigned to an author manuscript submitted to the NIH Manuscript Submission System.', false, 120),
        (36, 'Medline UID', 'Medline identifier.', false, 125),
        (34, 'Publisher article ID', 'Internal reference of article publisher for journal article.', true, 130),
        (37, 'PMC publisher ID', 'Publisher Id supplied to PubMed Central.', false, 132),
        (38, 'PM publisher ID', 'Publisher Id supplied to PubMed.', false, 134),
        (17, 'bioRxiv ID', 'Pre-prints, ID assigned by Cold Spring Harbour Laboratory.', true, 140),
        (18, 'arXiv ID', 'Pre-prints, ID assigned by Cornell university.', true, 145),
        (19, 'psyXiv ID', 'Pre-prints, ID assigned by the Centre for Open Science.', true, 150),
        (20, 'socXiv ID', 'Pre-prints, ID assigned by the Centre for Open Science.', true, 155),
        (21, 'Handle ID', 'ID assigned by a naming authority (handle system is a superset of DOIs).', false, 165),
        (22, 'ISBN', 'International Standard Book Number, assigned by the publisher.', false, 170),
        (23, 'ISTC', 'International Standard Text code ID, assigned by the international ISTC agency.', false, 175),
        (24, 'ISAN', 'International Standard Audiovisual Number, assigned by the ISAN international agency.', false, 180),
        (25, 'LSID', 'Life Science Identifier, a URN like specification with various issuing authorities.', false, 185),
        (26, 'Other bibliographic ID', 'Org id should identify the system manager.', false, 190),
        (33, 'NRCBL', 'KIE Reference Library (bioethics library) shelving location.', false, 195),
        (35, 'Serial item and contribution identifier ', 'The Serial Item and Contribution Identifier (SICI), a code used to uniquely identify specific volumes, articles or other identifiable parts of a serial. ', false, 197),
        (27, 'URL', 'Resource locator for a web based resource.', true, 205),
        (28, 'PURL', 'Persistent URL that redirects if necessary.', false, 210),
        (29, 'URN', 'Uniform Resource Name (a URI using the URN schema) that is location independent.', false, 220),
        (30, 'ARK', 'Archival Resource Key, a URL that provides additional metadata.', false, 225),
        (90, 'Other', 'None of the listed identifier types.', true, 990),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 995),
        (1, 'Type not provided', 'Missing type data in data source.', false, 998);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}

pub async fn create_object_filter_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.object_filter_types;
    CREATE TABLE lups.object_filter_types (
        id                 int4       NOT NULL PRIMARY KEY,
        filter_as          varchar    NOT NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.object_filter_types (id, filter_as, description, list_order)
       values
        (11, 'Trial registry entry', 'T13 - Summary of the study and its aims, posted prospectively or retrospectively to a public registry (not currently filtered in the UI).', 10),
        (12, 'Registry results summary', 'T28  - Summary of study results, as displayed in a trial registry.', 20),
        (21, 'Journal article', 'T12 - Articles in peer-reviewed publications that disseminate the results of original research and scholarship; T100 - Journal article abstract; T117 - Special journal issues; T135 - Working Paper / Pre-print; T152 - Grouped journal articles; 201 - Journal article - protocol; 202 - Journal article - results; 203 - Journal article - interim results; 204 - Journal article - review; 210 - Preprint article.', 30),
        (22, 'Study protocol', 'T11 - Protocol: Structured document describing the study, its rationale, methodology, outcome measures etc; T42 - Redacted protocol; T74 - Protocol with SAP; T75 - Protocol with informed consent forms; T76 - Protocol with SAP and ICFs; 201 - Journal article - protocol.', 50),
        (23, 'Study overview', 'T38 - A brief overview of the study, may be an abridged protocol, or as used within the study web site or other study documents.', 55),
        (24, 'Patient consent/information forms', 'T18 - Informed consent forms, the form or forms given to participants to formally record their informed consent to study participation; T19 - Patient information sheets, the information provided to study participants, especially as part of the consenting process; 75 - Protocol with informed consent forms; 76 - Protocol with SAP and ICFs.', 60),
        (25, 'Data collection forms', 'T21 - Data collection forms, Copies, in electronic and / or paper form, of the case report forms (CRFs and / or eCRFs) used for collecting data. T30 - Annotated copies of CRFs / eCRFs; T40 - Standard instruments: standardised rating instruments, including questionnaires.', 70),
        (26, 'Manual of procedures', 'T35 - Manual of Operations, T36 - Manual of Procedures. Description of specific operations, workflow, procedures and techniques within the study.', 80),
        (31, 'Statistical analysis plan', 'T22 - Statistical analysis plan: the details of the proposed analysis for the study, listing the individual descriptive statistics and tests of inference, and their parameters; T29 - Analysis notes: a summary of the analysis carried out and / or any caveats to be borne in mind when interpreting results; T43 - a redacted SAP; 74 - Protocol with SAP; 76 - Protocol with SAP and ICFs.', 90),
        (32, 'Clinical study report', 'T26 - Clinical study report - a full end of study report with detailed efficacy and safety results; T27 - a redacted clinical study report; T79 - a summary CSR; T85 - Unpublished Study Report: A report of a study, or part of a study, not formally published. May be an internal interim document within a long term study.', 100),
        (33, 'Data description', 'T20 - Database specification: Functional specification of the database including details of individual data items, types, ranges, logic checks, etc.; T31 - Data dictionary: A detailed, item by item, description of the data points in the dataset; T32 - Data Overview: A summary of the data indicating the nature of different tables, time points of data etc; T73 - IPD or aggregate data metadata definition;  T81 - Data collection schedule; T82 - Data coding manual.', 110),
        (34, 'IP or Aggregated Data', 'T80 - Individual participant data: A dataset simply called Individual Participant Data, or its equivalent, with no further qualification or description. Any of T51 - T68, being different specific types of IPD, e.g. relating to sub-populations or different time points, or metadata associated with IPD. T69 - Aggregated result dataset: Dataset with aggregated / summary results and statistics from the study. T70, T71, T72, being different specific types of aggregate data. Also T153 - Grouped analysis datasets, and T154 - Grouped aggregate datasets.', 120),
        (53, 'Samples description', 'T301 - Associated biosamples (link is to sample description)', 130),
        (36, 'Other study resource', 'T14 - Ethics submission; T15 - Ethics approval notification; T16 - Regulatory authority submission; T17 - Regulatory authority approval notification; T23 - Data management plan; T24 - Trial master file contents list; T25 - Data monitoring committee report; T33 - Definitions of terms; T34 - Literature Review; T39 - Publication List; T77 - Investigational Product Information; T78 - General background to research topic; T83 - Bibliography; T84 - Introduction to document set; T86 - List of web links; T115 - Funding Submission; T171 - Other dataset object: any dataset class resource not defined elsewhere.', 140),
        (41, 'Other informational material', 'T88 - Summary of results for public; T106 - Conference Abstract; T107 - Conference Paper; T108 - Conference Poster: T109 - Conference Program; T119 - Magazine Article; T121 - Newsletter Article; T122 - Newspaper Article; T101 - Book; T102 - Book chapter: T103 - Book Prospectus; T104 - Book Review; T105 - Book Series; ;T112 - Dissertation;  T113 - Edited Book;  T114 -Encyclopaedia Entry; T120 - Manual (for education / training purposes); T123 - Online Resource; T126 - Report; T127 - Research Tool; T128 - Supervised Student Publication.', 150),
        (51, 'Website', 'T134 - Stand-alone locations on the web where multiple types of information on a specific theme are available. May include interactive features for contributions from readers.', 190),
        (52, 'Software', 'T166 - Script(s) used in analysis; T167 - CDMS (Clinical Data Management System); T168 - Trial Management System; T169 - eTMF; T170 - Data Extraction System.', 200),
        (99, 'Other', 'All types in each object class labelled as Other (T37, T151, T155 - T165).', 210);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_object_relationship_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.object_relationship_types;
    CREATE TABLE lups.object_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.object_relationship_types (id, name, description, 
          use_in_data_entry, list_order)
       values 
        (11, 'Is cited by', 'Indicates that B includes A in a citation.', false, 10),
        (12, 'Cites', 'Indicates that A includes B in a citation.', false, 15),
        (13, 'Is supplement to', 'Indicates that A is a supplement to B.', true, 20),
        (14, 'Is supplemented by', 'Indicates that B is a supplement to A.', true, 25),
        (15, 'Is continued by', 'Indicates A is continued by the work B.', false, 30),
        (16, 'Continues', 'Indicates A is a continuation of the work B.', false, 35),
        (17, 'Is described by', 'Indicates A is described by B.', true, 40),
        (18, 'Describes', 'Indicates A describes B.', true, 45),
        (19, 'Has metadata', 'Indicates resource A has associated metadata B.', true, 50),
        (20, 'Is metadata for', 'Indicates is metadata for resource B.', true, 55),
        (21, 'Has version', 'Indicates A has a version B.', true, 60),
        (22, 'Is version of', 'Indicates A is a version of B.', true, 65),
        (23, 'Is new version of', 'Indicates A is a new edition of B, where the new edition has been modified or updated.', false, 70),
        (24, 'Is previous version of', 'Indicates A is a previous edition of B.', false, 75),
        (25, 'Is part of', 'Indicates A is a portion of B; may be used for elements of a series.', false, 80),
        (26, 'Has part', 'Indicates A includes the part B.', false, 85),
        (27, 'Is referenced by', 'Indicates A is used as a source of information by B.', false, 90),
        (28, 'References', 'Indicates B is used as a source of information for A.', false, 95),
        (29, 'Is documented by', 'Indicates B is documentation about/ explaining A; e.g. points to software documentation.', true, 100),
        (30, 'Documents', 'Indicates A is documentation about B; e.g. points to software documentation.', true, 105),
        (31, 'Is compiled by', 'Indicates B is used to compile or create A.', false, 110),
        (32, 'Compiles', 'Indicates B is the result of a compile or creation event using A.', false, 115),
        (33, 'Is variant form of', 'Indicates A is a variant or different form of B.', false, 120),
        (34, 'Is original form of', 'Indicates A is the original form of B.', false, 125),
        (35, 'Is identical to', 'Indicates that A is identical to B, for use when there is a need to register two separate instances of the same resource.', false, 130),
        (36, 'Is reviewed by', 'Indicates that A is reviewed by B.', false, 135),
        (37, 'Reviews', 'Indicates that A is a review of B.', false, 140),
        (38, 'Is derived From', 'Indicates B is a source upon which A is based.', true, 145),
        (39, 'Is source of', 'Indicates A is a source upon which B is based.', true, 150),
        (40, 'Is required by', 'Indicates A is required by B (may be used to indicate software dependencies).', false, 155),
        (41, 'Requires', 'Indicates A requires B (may be used to indicate software dependencies).', false, 160),
        (42, 'Obsoletes', 'Indicates A replaces B.', true, 165),
        (43, 'Is obsoleted by', 'Indicates A is replaced by B.', true, 170),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_object_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.object_types;
    CREATE TABLE lups.object_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        object_class_id    int4       NULL,
	    filter_as_id       varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.object_types (id, name, object_class_id,
          filter_as_id, description, use_in_data_entry, list_order)
       values 

        (10, 'Introduction to document set', 23, '36', 'A ‘contents’, ‘readme’ or similar document that describes the other documents available.', true, 78),

        (11, 'Study protocol', 23, '22', 'Structured document describing the study, its rationale, methodology, outcome measures etc.', true, 7),
        (12, 'Statistical analysis plan', 23, '31', 'The details of the proposed analysis for the study, listing the individual descriptive statistics and tests of inference, and their parameters.', true, 60),
        (13, 'Informed consent forms', 23, '24', 'The form or forms given to participants to formally record their informed consent to study participation.', true, 40),
        (14, 'Patient information sheets', 23, '24', 'The information provided to study participants, especially as part of the consenting process.', true, 45),
        (15, 'Study overview', 23, '23', 'A brief overview of the study, may be an abridged protocol, or as used within the study web site or other study documents.', true, 20),
        (16, 'Redacted protocol', 23, '22', 'A redacted version of the study protocol.', false, 94),
        (17, 'Redacted SAP', 23, '31', 'A redacted version of the statistical analysis plan.', false, 61),
        (20, 'Protocol SAP', 23, '22, 31', 'Study Protocol and Statistical Analysis Plan, combined in one document.', true, 62),
        (21, 'Protocol ICF', 23, '22, 24', 'Study Protocol and Informed Consent Form, combined in one document.', true, 63),
        (22, 'Protocol SAP ICF', 23, '22, 24, 31', 'Study Protocol, Statistical Analysis Plan, and Informed Consent Form, all combined in one document.', true, 64),

        (30, 'Trial registry entry', 23, '11', 'Summary of the study and its aims, posted prospectively or retrospectively to a public registry.', false, 5),
        (31, 'Trial registry results summary', 23, '12', 'Summary of study results, as displayed in a tab or page within a trial registry.', false, 87),

        (40, 'Website', 23, '51', 'Stand-alone locations on the web where multiple types of information on a specific theme are available. May include interactive features for contributions from readers.', false, 355),
       
        (50, 'Database specification', 23, '33', 'Functional specification of the database including details of individual data items, types, ranges, etc. May also contain details of logic and consistency checks, and any derived values.', true, 50),
        (51, 'Data collection forms', 23, '25', 'Copies, in electronic and / or paper form, of the case report forms (CRFs and / or eCRFs) used for collecting data.', true, 55),
        (52, 'Annotated data collection forms', 23, '25', 'Data Collection forms (CRFs or eCRFs) annotated to provide further details of each item (e.g. data type, allowable range).', true, 56),
        (53, 'Data dictionary', 23, '33', 'A detailed, item by item, description of the data points in the dataset, sufficient for accurate analysis of the data by others.', true, 52),
        (54, 'Data overview', 23, '33', 'A summary of the data, without the details of a data dictionary but indicating the nature of different tables, time points of data etc.', true, 48),
        (55, 'Data management plan', 23, '36', 'A plan for, and record of, data management activities in the study, covering the whole data life cycle.', true, 65),
        (56, 'Data collection schedule', 23, '33', 'A document detailing the time points of data collection in a study (or ‘visits’).', true, 72),
        (57, 'Data coding manual', 23, '33', 'A manual or guide that provides instructions on how to complete and / or interpret scores and codes within a study.', true, 74),
       
        (60, 'Trial master file contents list', 23, '36', 'A listing of the documents expected within a trial master file, and their organisation, in electronic and / or paper form.', false, 70),
        (61, 'Manual of operations', 23, '26', 'Description of specific operations and workflow within the study.', true, 37),
        (62, 'Manual of procedures', 23, '26', 'Description of specific procedures and techniques used within the study.', true, 38),
        (63, 'Investigational product information', 23, '36', 'Summary of information about a medicinal product. May be a package insert or investigator’s brochure.', true, 92),
       
        (80, 'Literature review', 23, '36', 'Publications referenced within the literature review undertaken prior to the study.', false, 90),
        (81, 'General background paper', 23, '36', 'Supporting document summarising relevant research and / or research programs, or aspects of condition pathology, epidemiology, etc.', true, 93),
        (82, 'Publication list', 23, '36', 'List of publications related to the study.', true, 91),
        (83, 'Bibliography', 23, '36', 'A list of publications making up a bibliography relevant to the study, but not necessarily generated or triggered by the study.', false, 76),
        (84, 'List of web links', 23, '36', 'A web page that includes a list of links to different items, e.g. individual CRFs.', false, 89),
        
        (90, 'Course materials (for teaching)', 23, '41', 'Course and assignment materials produced for teaching purposes.', false, 285),
        (91, 'Online resource', 23, '41', 'Information or data accessible only on the web via technical methods (e.g. search, hyperlinks).', false, 300),
        (92, 'Standard instruments', 23, '25', 'Standardised rating instruments, including questionnaires, used or developed in a study.', true, 58),
        (93, 'CDMS', 21, '52', 'Clinical Database Management System, or component within such a system, as used for clinical data management.', false, 410),
        (94, 'Trial management system', 21, '52', 'System used to support study administration, including an eTMF.', false, 415),
        (95, 'Data extraction system', 21, '52', 'Used for extracting data from remote resources using XML or web scraping.', false, 425),
        
        (120, 'Clinical study report', 23, '32', 'Full end of study report with detailed efficacy and safety results.', true, 80),
        (121, 'Redacted clinical study report', 23, '32', 'End of study report with some data withheld, usually because of commercial sensitivity.', true, 85),
        (122, 'Results or CSR summary', 23, '32', 'A results summary, or a  summary derived from  a Clinical Study Report', true, 83),
        (123, 'Unpublished study report', 23, '32', 'A report of a study, or part of a study, not formally published. May be an internal interim document within a long term study.', true, 80),
        (124, 'Summary of results for public', 23, '32', 'Summary of results specifically targeted at participants and members of the public, usually as a web page', true, 82),
        (125, 'Deliverable Report', 23, '41', 'Reports disseminating the outcomes and deliverables of a research contract, e.g. a deliverable within an EU funded programme. May entail a contribution to public policy.', false, 315),
        (126, 'Data monitoring committee report', 23, '36', 'A report concerning the safety or efficacy of a study, from independent experts. Often containing recommendations about the continuation of the study.', false, 75),

        (150, 'Funding submission', 23, '36', 'Information about specific requests for funds submitted to potential funders of the activity. The standard allows details to be collected for multiple years.', false, 260),
        (151, 'Ethics submission', 23, '36', 'Documents provided to an ethics review board, often with the protocol, when seeking ethical approval for a study.', true, 22),
        (152, 'Ethics approval notification', 23, '36', 'Documents from an ethical review board confirming that ethical approval has been granted.', true, 25),
        (153, 'Regulatory authority submission', 23, '36', 'Documents provided to a regulatory authority, seeking approval to run a clinical trial.', true, 30),
        (154, 'Regulatory authority approval notification', 23, '36', 'Documents from a regulatory authority confirming that approval to run the trial has been granted.', true, 35),
              
        (200, 'Journal article - unspecified', 23, '21', 'Article in a peer-reviewed publication - exact relationship to the research study not known.', true, 10),
        (202, 'Journal article - results', 23, '21', 'Article in a peer-reviewed publication - specifically describing the results of a study.', true, 12),
        (201, 'Journal article - protocol', 23, '21, 22', 'Article in a peer-reviewed publication - describing the protocol of a study.', true, 14),
        (203, 'Journal article - interim results', 23, '21', 'Article in a peer-reviewed publication - specifically describing interim results of a study, or results of a feasibility study.', true, 15),
        (204, 'Journal article - review', 23, '21', 'Article in a peer-reviewed publication - providing a review of arelated scientific area or a summary funder based report.', true, 16),
       
        (210, 'Preprint article', 23, '21', 'Article in a preprint journal.', true, 19),
        (211, 'Grouped journal articles', 12, '21', 'A collection of journal articles on the same topic or study (should also be recorded separately).', false, 370),
        (212, 'Dissertation', 23, '41', 'Treatise advancing an original point of view resulting from research: a requirement for a doctoral degree.', false, 245),
        (213, 'Supervised student publication', 23, '41', 'Articles on research findings published jointly with or supervised by the thesis advisor. The findings relate to research undertaken by the student or the supervisor’s program of research.', false, 325),
        (217, 'Journal issue', 23, '21', 'Periodical publications aimed at fostering intellectual debate and inquiry. Special journal issues are produced by editors with an established record of scholarship in the field and able to provide the direction of the theme. Journal issues bear a unique number of reference for publication.', false, 270),
        
        (220, 'Book', 23, '41', 'Books written by a single author or collaboratively based on research or scholarly findings generally derived from peer reviewed funding.', false, 190),
        (221, 'Book chapter', 23, '41', 'Texts written by a single author or collaboratively based on research or scholarly findings and expertise in a field.', false, 195),
        (222, 'Book series', 23, '41', 'Set of related books written by a single author or collaboratively based on research or scholarly findings.', false, 210),
        (223, 'Edited book', 23, '41', 'Books edited by a single author or collaboratively for the dissemination of research or scholarly findings that generally result from peer reviewed funding.', false, 250),

        (226, 'Conference abstract', 23, '41', 'Texts of a specified length that states the issue to be discussed in a proposed conference paper. It serves as the basis for the acceptance of the paper at a conference. The abstract is published along with the paper.', true, 215),
        (227, 'Conference paper', 23, '41', 'Papers written alone or collaboratively, presented at an academic conference, and published in the proceedings (not in scholarly journals).', true, 220),
        (228, 'Conference poster', 23, '41', 'Posters displayed in a conference setting and conveying research highlights in an efficient manner by compelling graphics. They may be peer-reviewed prior to acceptance and be published in the proceedings.', true, 225),

        (230, 'Magazine article', 23, '41', 'Articles in thematic publications published at fixed intervals.', false, 280),
        (231, 'Newsletter article', 23, '41', 'Articles in publications aimed at researchers, decision-makers, professionals and the public that report on a research project or on the activities of a research chair or a research centre. The Newsletters promote research activities to the community and the university; mobilize knowledge to improve practice and inform policy, and provide relevant and accessible information to the broader public.', false, 290),
        (232, 'Newspaper article', 23, '41', 'Articles in a daily, weekly or monthly publication reporting on news and social issues aimed at the public. May entail critical analysis based on expertise in the field.', false, 295),
      
        (250, 'Individual participant data', 14, '34', 'A dataset simply called Individual Participant Data, or its equivalent, with no further qualification or description', true, 97),
        (251, 'IPD final analysis datasets (full study population)', 14, '34', 'Full final dataset supporting all analyses carried out on the study data.', true, 98),
        (252, 'IPD final analysis datasets (supporting specific paper)', 14, '34', 'Dataset supporting the analyses and conclusions of a specific single paper.', true, 99),
        (253, 'IPD final analysis datasets (sub-population)', 14, '34', 'Dataset with the data from a sub-population of the complete study (e.g. the control arm, or a particular age group).', true, 100),
        (254, 'IPD final analysis datasets (sub-study)', 14, '34', 'Dataset with the data of a sub-study, supporting the analysis only of that sub-study - may also involve a sub-population of the whole.', true, 103),
        (256, 'IPD interim analysis dataset (sub-population)', 14, '34', 'Dataset with the data from a sub-population of the complete study, from an earlier time point than the primary analysis.', false, 104),
        (257, 'IPD interim analysis dataset (sub-study)', 14, '34', 'Dataset with the data of a sub-study, supporting the analysis only of that sub-study, from an earlier time point than the primary analysis.', false, 105),
        (258, 'IPD long term follow up analysis dataset', 14, '34', 'Supplementary dataset with additional data from long term follow up, data collected after primary analysis.', false, 106),
        (255, 'IPD interim analysis dataset', 14, '34', 'Dataset covering the whole study but from an earlier time point than the primary analysis.', false, 110),
        (259, 'IPD safety analysis dataset', 14, '34', 'Dataset of individual data supporting comprehensive safety analysis.', false, 115),
        (260, 'IPD PK analysis dataset', 14, '34', 'Dataset of pharmaco-kinetic data.', false, 120),
        (261, 'IPD PD analysis dataset', 14, '34', 'Dataset of pharmaco-dynamic data.', false, 125),
        (262, 'IPD quality of life analysis dataset', 14, '34', 'Dataset supporting analysis of quality of life measures within the study.', false, 130),
        (263, 'IPD analysis dataset, other', 14, '34', 'Analysis dataset not listed in other options.', false, 135),
        (264, 'IPD analysis dataset metadata definition', 14, '34', 'A dataset that is the metadata for an analysis dataset.', true, 140),
        (265, 'IPD CDMS format dataset', 14, '34', 'A dataset in the format used by the data collection system (CDMS). May be a database file.', false, 145),
        (266, 'IPD CDMS format dataset metadata definition', 14, '34', 'The metadata definition for the study’s data collection system - equivalent to the functional specification of the system but as a dataset rather than a document.', false, 150),
        (267, 'IPD transport format dataset', 14, '34', 'A dataset in a format designed specifically for transfer between systems.', true, 155),
        (268, 'IPD transport format dataset metadata definition', 14, '34', 'Metadata describing the dataset when in a transport format.', true, 160),
        (269, 'Grouped analysis datasets', 12, '34', 'A collection of analysis datasets on the same topic or study (should also be recorded separately).', false, 375),

        (280, 'Aggregated result dataset', 14, '34', 'Dataset with aggregated / summary results and statistics from the study.', true, 165),
        (281, 'Aggregated result dataset, efficacy measures', 14, '34', 'Dataset with aggregated results and statistics focusing on efficacy measures.', false, 170),
        (282, 'Aggregated result dataset, safety measures', 14, '34', 'Dataset with aggregated results and statistics focusing on safety measures.', false, 175),
        (283, 'Aggregated result dataset, other', 14, '34', 'Dataset with aggregated results and statistics, not listed elsewhere.', false, 180),
        (284, 'Grouped aggregated result datasets', 12, '34', 'A collection of aggregated result datasets on the same topic or study (should also be recorded separately).', false, 380),
        
        (290, 'Metadata definition', 23, '33', 'Metadata definition, for IPD and / or an aggregated result set, when a separate file.', true, 185),
        (291, 'Script(s) used in analysis', 21, '52', 'Statistical software scripts used in study analysis.', true, 405),
        (292, 'Analysis notes', 23, '31', 'A summary of the analysis carried out and / or any caveats to be borne in mind when interpreting results. Less formal than a statistical analysis plan.', true, 61),

        (301, 'Sample Description', 19, '53', 'A description of a bio-sample collection associated with or generated by a study. The URL is to a summary description of the biosamples with further details on access', true, 99),
     
        (951, 'Other AV object', 11, '99', 'Any audiovisual resource not defined elsewhere.', true, 1365),
        (955, 'Other collection object', 12, '99', 'Any collection class resource not defined elsewhere.', false, 1385),
        (956, 'Other data paper object', 13, '99', 'Any data paper class resource not defined elsewhere.', false, 1390),
        (971, 'Other dataset object', 14, '36', 'Any dataset class resource not defined elsewhere.', true, 1392),
        (957, 'Other event', 15, '99', 'Any event class resource not defined elsewhere.', false, 1395),
        (958, 'Other image object', 16, '99', 'Any image class resource not defined elsewhere.', true, 1400),
        (959, 'Other interactive object', 17, '99', 'Any interactive resource class not defined elsewhere.', true, 1405),
        (961, 'Other physical object', 19, '99', 'Any physical object class resource not defined elsewhere.', false, 1415),
        (962, 'Other service resource', 20, '99', 'Any serviceclass resource not defined elsewhere.', false, 1420),
        (965, 'Other software object', 21, '99', 'Any software class resource not defined elsewhere.', true, 1422),
        (963, 'Other sound object', 22, '99', 'Any sound class resource not defined elsewhere.', true, 1425),
        (960, 'Other text based object', 23, '99', 'A text class resource not defined elsewhere.', true, 1427),
        (964, 'Other workflow object', 24, '99', 'Any workflow class resource not defined elsewhere.', false, 1430),
        (990, 'Other object, in ‘other’ class', 18, '99', 'Any object not defined elsewhere and in the ‘other’ class', false, 1998),

        (0, 'Not yet known', 0, '99', 'Dummy value supplied by default on entity creation.', false, 1999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


   
pub async fn create_resource_types(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.resource_types;
    CREATE TABLE lups.resource_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.resource_types (id, name, description, 
          use_in_data_entry, list_order)
       values
        (11, 'PDF', '', true, 10),
        (35, 'Web text', 'Web page or text, no download or API options.', false, 12),
        (36, 'Web text with download', 'Web text, with download option (e.g. PubMed Central entry).', false, 14),
        (37, 'Web text with XML via API ', 'Web page or text, content as XML via API.', false, 16),
        (38, 'Web text with JSON via API ', 'Web page or text, content as JSON via API.', false, 18),
        (39, 'Web text with XML or JSON via API ', 'Web page or text, content as XML or JSON via API (e.g. from Clinical trials.gov, PubMed).', false, 20),
        (40, 'Web text journal abstract', 'Web page or text that is an abstract of a journal article', false, 21),
        (12, 'Comma separated values', 'Usually .csv files.', true, 30),
        (13, 'Tab separated values', 'May be .tsv or .txt files.', true, 32),
        (14, 'Plain text file', 'e.g. from text editor, or data with delimiter unknown.', true, 34),
        (15, 'Rich text file', '.rtf files.', true, 36),
        (16, 'Word doc', '.doc or .docx files.', true, 38),
        (17, 'Other WP document', 'e.g. .odt (libra office), .gdoc (google docs) files.', true, 40),
        (18, 'Excel spreadsheet(s)', '.xsl or .xslx. files.', true, 45),
        (19, 'Other spreadsheet(s)', 'e.g. .ods (libra office), .gsheet (google docs) files.', true, 50),
        (20, 'PowerPoint', '.ppt or .pptx files.', true, 55),
        (21, 'Other presentation', 'e.g. .odp (libra office), .gslides (google docs) files).', true, 60),
        (22, 'ODM XML document', 'Data or metadata following CDISC ODM schema.', true, 65),
        (23, 'SDTM XML document', 'Data using the CDISC SDTM schema.', true, 70),
        (24, 'Define.XML document', 'Metadata following the CDISC Define.xml schema.', true, 75),
        (25, 'ADaM XML document', 'Analysis data following thbe CDISC ADaM schema.', true, 80),
        (26, 'XML document', 'XML schema not listed elsewhere.', true, 82),
        (27, 'SAS transport file', '.xpt files.', true, 84),
        (80, 'Virtual dataset', 'Dataset files (exact format unknown) available by application', false, 90),
        (81, 'Virtual document', 'Document file (exact format unknown) available by application', false, 91),
        (82, 'Virtual samples', 'Samples available by application', false, 92),
        (28, 'R workspace file', '.rdata or .rda files.', true, 100),
        (29, 'SSPS data file', '.sav file.', true, 105),
        (30, 'Stata data file', '.dta file.', true, 110),
        (31, 'Statistical program data file', 'Stats data file in format not listed elsewhere.', true, 115),
        (32, 'Database file', 'A complete DB file that could be remounted directly into a DB system.', false, 120),
        (33, 'Graphic image', 'e.g. .png, .jpeg, .svg files.', true, 125),
        (34, 'Media file', 'e.g. mp3, .mp4, .mpeg files.', true, 130),
        (90, 'Other', 'File format not listed elsewhere.', true, 900),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn create_size_units(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists lups.size_units;
    CREATE TABLE lups.size_units (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL
    );"#;
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into lups.size_units (id, name, list_order)
       values 
        (11, 'Kb', 10),
        (12, 'Mb', 20),
        (13, 'Gb', 30),
        (14, 'Pages', 40),
        (15, 'Minutes', 50),
        (16, 'Other', 60),
        (0, 'Not yet known', 99);"#;

    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}

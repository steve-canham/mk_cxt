

pub fn contribution_types<'a>() -> &'a str {

    r#"insert into lkup.contribution_types (id, name, applies_to, description,
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (51, 'Study lead', 'individual', 'The individual who, if not the sponsor themselves, leads and co-ordinates the scientific and clinical activity within a clinical study, including co-ordinating the work of principal investigators at clinical sites. May be known as the Co-ordinating Investigator, the Study Chair, Study Director, the Scientific Contact or similar terms.', true, 10, 'ECRIN', '2019-01-14'),
        (52, 'CT site principal investigator', 'individual', 'The individual responsible for the safe conduct of a clinical trial at a particular clinical site.', true, 15, 'ECRIN', '2019-01-14'),
        (53, 'Clinical study manager', 'individual', 'An individual responsible for the operational management of a clinical study. Similar to a Project Manager but a study manager is heavily involved in the management of data and data collection.', true, 20, 'ECRIN', '2019-01-14'),
        (12, 'Contact person', 'individual', 'Person with knowledge of how to access, troubleshoot, or otherwise field issues related to the resource. May also be “Point of Contact” in organisation that controls access to the resource.', true, 25, 'DataCite', '2019-01-14'),
        (55, 'Sponsor contact', 'individual', 'An individual representing the sponsor and acting as an initial contact point.', true, 27, 'ECRIN', '2019-01-14'),
        (56, 'Public contact', 'individual', 'An individual designated as dealing with non-scientific queries from the public or press.', true, 29, 'ECRIN', '2019-01-14'),
        (57, 'Recruitment contact', 'individual', 'An individual designated as providing periodic updates on recruitment information or status, at all sites, usually for monitoring purposes.', false, 31, 'ECRIN', '2019-01-14'),
        (59, 'Funder contact', 'individual', 'An individual representing the funder and acting as an initial contact point.', false, 33, 'ECRIN', '2019-01-14'),
        (71, 'Results contact', 'individual', 'The individual, occasionally organisation, to be contacted for further information on the study results.', true, 35, 'ECRIN', '2019-01-14'),
        (11, 'Creator / Author', 'individual', 'The main researchers involved in producing the data, or the authors of the publication, in priority order. To supply multiple creators, repeat this property. May be a corporate/institutional or personal name.', true, 45, 'DataCite', '2019-01-14'),
        (17, 'Editor', 'individual', 'A person who oversees the details related to the publication format of the resource. If the Editor is to be credited in place of multiple creators, the Editor’s name may be supplied as Creator, with “(Ed.)” appended to the name.', false, 50, 'DataCite', '2019-01-14'),
        (20, 'Project leader', 'individual', 'Person officially designated as head of project team or subproject team instrumental in the work necessary to development of the resource. The Project Leader is not “removed” from the work that resulted in the resource; he or she remains intimately involved throughout the life of the particular project team.', true, 55, 'DataCite', '2019-01-14'),
        (21, 'Project manager', 'individual', 'Person officially designated as manager of a project. Project may consist of one or many project teams and sub-teams. The manager of a project normally has more administrative responsibility than actual work involvement.', true, 60, 'DataCite', '2019-01-14'),
        (22, 'Project member', 'individual', 'Person on the membership list of a designated project/project team. This vocabulary may or may not indicate the quality, quantity, or substance of the person’s involvement. ', true, 65, 'DataCite', '2019-01-14'),
        (25, 'Related person', 'individual', 'A person without a specifically defined role in the development of the resource, but who is someone the author wishes to recognize. This person could be an author’s intellectual mentor, a person providing intellectual leadership in the discipline or subject domain, etc.', false, 80, 'DataCite', '2019-01-14'),
        (26, 'Researcher', 'individual', 'A person involved in analyzing data or the results of an experiment or formal study. May indicate an intern or assistant to one of the authors who helped with research but who was not so “key” as to be listed as an author. Should be a person, not an institution.', true, 85, 'DataCite', '2019-01-14'),
        (30, 'Supervisor', 'individual', 'Designated administrator over one or more groups/teams working to produce a resource or over one or more steps of a development process.', false, 105, 'DataCite', '2019-01-14'),
        (31, 'Work package leader', 'individual', 'A Work Package is a recognized data product, not all of which is included in publication. The package, instead, may include notes, discarded documents, etc. The Work Package Leader is responsible for ensuring the comprehensive contents, versioning, and availability of the Work Package during the development of the resource.', true, 110, 'DataCite', '2019-01-14'),
        (27, 'Research group', 'organisation', 'Typically refers to a group of individuals with a lab, department, or division; the group has a particular, defined focus of activity. May operate at a narrower level of scope; may or may not hold less administrative responsibility than a project team.', true, 120, 'DataCite', '2019-01-14'),
        (54, 'Trial sponsor', 'organisation', 'The organisation or individual that for a clinical trial has the formal, legal role of a clinical trial sponsor, and for observational role has an anologous responsibility for the organisation and conduct of the study.', true, 130, 'ECRIN', '2019-01-14'),
        (58, 'Study funder', 'organisation', 'An organisation providing some or all of the additional funds required for the study.', true, 150, 'ECRIN', '2019-01-14'),
        (112, 'Study sponsor and funder', 'organisation', 'The organisation that for a clinical trial has the formal, legal role of a clinical trial sponsor,  and which is also listed as a study funder.', true, 155, 'ECRIN', '2023-05-04'),
        (60, 'Independent monitoring committee member', 'individual', 'A member of a safety monitoring committee for a clinical trial, independent of the researchers and research activity.', true, 160, 'ECRIN', '2019-01-14'),
        (61, 'Medicinal product supplier', 'organisation', 'Organisation that provides one or more of the medicines investigated in a clinical study.', true, 165, 'ECRIN', '2019-01-14'),
        (62, 'Medical device supplier', 'organisation', 'Organisation that provides one or more of the medical devices in a clinical study.', true, 170, 'ECRIN', '2019-01-14'),
        (63, 'Logistics support organisation', 'organisation', 'Organisation that provides logistical input, e.g. provides a drug distribution service.', true, 175, 'ECRIN', '2019-01-14'),
        (64, 'Scientific support organisation', 'organisation', 'Organisation that provides scientific support, e.g. a national or international research network.', true, 180, 'ECRIN', '2019-01-14'),
        (65, 'Central laboratory', 'organisation', 'Organisation that provides a central specialist laboratory testing facility.', true, 185, 'ECRIN', '2019-01-14'),
        (66, 'Central imaging facility', 'organisation', 'Organisation that provides a central specialist imaging or scanning facility.', true, 190, 'ECRIN', '2019-01-14'),
        (67, 'Clinical organisation', 'organisation', 'Organisation, usually a primary or secondary health care organisation, that manages one or more of the sites where a clinical study takes place.', true, 195, 'ECRIN', '2019-01-14'),
        (68, 'Clinical site', 'organisation', 'Organisation or location, usually in a primary or secondary health care organisation, that is one of the sites where a clinical study takes place.', false, 200, 'ECRIN', '2019-01-14'),
        (69, 'Collaborating organisation', 'organisation', 'May be listed as a secondary sponsor, an organisation other than the lead sponsor involved in supporting a study.', true, 205, 'ECRIN', '2019-01-14'),
        (29, 'Sponsor', 'both', 'Person or organisation that issued a contract or under the auspices of which a work has been written, printed, published, developed, etc. Includes organisations that provide in-kind support, through donation, provision of people or a facility or instrumentation necessary for the development of the resource, etc.', false, 205, 'DataCite', '2019-01-14'),
        (23, 'Registration agency', 'organisation', 'Institution/organisation officially appointed by a Registration Authority to handle specific tasks within a defined area of responsibility, e.g. DataCite is a Registration Agency for the International DOI Foundation (IDF).', true, 210, 'DataCite', '2019-01-14'),
        (70, 'Sponsor-investigator', 'individual', 'An individual with the role of sponsor as well as being the co-ordinating investigator for the study.', true, 210, 'ECRIN', '2019-01-14'),
        (72, 'Research group member', 'individual', 'From PubMed, an individual (e.g., collaborator or investigator) who is not an author of a paper but is listed as a member of a collective/corporate group that is an author of the paper. ', true, 220, 'PubMed', '2019-08-24'),
        (16, 'Distributor', 'organisation', 'Institution tasked with responsibility to generate/disseminate copies of the resource in either electronic or print form. Works stored in more than one archive/repository may credit each as a distributor. ', false, 235, 'DataCite', '2019-01-14'),
        (18, 'Hosting institution', 'organisation', 'Typically, the organisation allowing the resource to be available on the internet through the provision of its hardware/software/operating support. May also be used for an organisation that stores the data offline. Often a data centre (if that data centre is not the “publisher” of the resource).', false, 245, 'DataCite', '2019-01-14'),
        (19, 'Producer', 'both', 'Typically a person or organisation responsible for the artistry and form of a media product. In the data industry, this may be a company “producing” DVDs that package data for future dissemination by a distributor. ', false, 250, 'DataCite', '2019-01-14'),
        (24, 'Registration authority', 'organisation', 'A standards-setting body from which Registration Agencies obtain official recognition and guidance, e.g. the IDF serves as the Registration Authority for the International Standards Organisation (ISO) in the area/domain of Digital Object Identifiers. ', false, 275, 'DataCite', '2019-01-14'),
        (13, 'Data collector', 'individual', 'Person/institution responsible for finding, gathering/collecting data under the guidelines of the author(s) or Principal Investigator (PI).', true, 320, 'DataCite', '2019-01-14'),
        (14, 'Data curator', 'individual', 'Person tasked with reviewing, enhancing, cleaning, or standardizing metadata and the associated data submitted for storage, use, and maintenance within a data centre or repository. The Data Curator’s role encompasses quality assurance focused on content and metadata, e.g. checking whether the submitted dataset is complete, with all files and components as described by submitter, whether the metadata is standardized to appropriate systems and schema, whether specialized metadata is needed to add value and ensure access across disciplines, and determining how  the metadata might map to search engines, database products, and automated feeds. ', true, 325, 'DataCite', '2019-01-14'),
        (15, 'Data manager', 'individual', 'Person (or organisation with a staff of data managers, such as a data centre) responsible for maintaining the finished resource. The work done by this person or organisation ensures that the resource is periodically “refreshed” in terms of software/hardware support, is kept available or is protected from unauthorized access, is stored in accordance with industry standards, and is handled in accordance with the records management requirements applicable to it.', true, 330, 'DataCite', '2019-01-14'),
        (28, 'Rights holder ', 'both', 'Person or institution owning or managing property rights, including intellectual property rights over the resource.', true, 395, 'DataCite', '2019-01-14'),
        (90, 'Other', 'both', 'Any person or institution making a significant contribution to the development and/or maintenance of the resource, but whose contribution does not “fit” other controlled vocabulary for contributorType.', true, 900, 'DataCite', '2019-01-14'),
        (0, 'Not yet known', 'both', 'Dummy value supplied by default on entity creation.', false, 999, 'ECRIN', '2019-01-14');"#
}


pub fn dataset_consent_types<'a>() -> &'a str {

    r#"insert into lkup.dataset_consent_types (id, name, description, 
          list_order, source_org, date_added)
       values 
         (0, 'Not known', 'Information about consent for secondary use unavailable', 10, 'ECRIN', '2019-02-08'),
         (1, 'No explicit consent', 'No specific consent was given for the sharing of data or its re-use beyond the study in which it was originally collected.', 20, 'ECRIN', '2020-08-20'),
         (2, 'No restriction', 'No restriction explicitly stated in the consent documents, OR a broad consent to re-use is present without qualification.', 30, 'DUO', '2020-08-20'),
         (3, 'General research use', 'Consent indicates that use is allowed for general research use for any research purpose.', 40, 'DUO', '2020-08-20'),
         (4, 'Health / medical / biomedical research', 'Consent indicates that use is allowed for health/medical/biomedical purposes; does not include the study of population origins or ancestry, or the development of methods / algorithms (e.g. for ML)', 50, 'DUO', '2020-08-20'),
         (5, 'Disease-specific research', 'Consent indicates that use, for health/medical/biomedical research is allowed provided it is related to a specified disease (area). The disease (area) must be named or coded in the associated comments field.', 60, 'DUO', '2020-08-20'),
         (6, 'Consent specified, not elsewhere categorised', 'A descriptive statement regarding consent is available, but it does not fit into categories 1 - 5', 70, 'ECRIN', '2020-09-23');"#
}


pub fn dataset_deidentification_levels<'a>() -> &'a str {

    r#"insert into lkup.dataset_deidentification_levels (id, name, description, 
          list_order, source_org, date_added)
       values 
        (0, 'Not known', 'No clear information available about the de-identification, if any, applied to the data', 10, 'ECRIN', '2019-02-08'),
        (1, 'No de-identification', 'Confirmed that no de-identification measures have been applied to the data set.', 20, 'ECRIN', '2019-02-08'),
        (2, 'De-identification applied', 'Some de-identification measures have been applied. Details should be described in comments and / or indicated in the linked boolean fields, or in separate documents.', 30, 'ECRIN', '2019-02-08'),
        (3, 'De-identification applied, primary outcomes re-assessed', 'Some de-identification measures have been applied and are described. In addition the data has been re-analysed against the primary outcomes and the results described.', 40, 'ECRIN', '2019-02-08');"#
}


pub fn dataset_recordkey_types<'a>() -> &'a str {

    r#"insert into lkup.dataset_recordkey_types (id, name, description, 
          list_order, source_org, date_added)
       values 
        (0, 'Not known', 'No clear information available about the record keys in use.', 10, 'ECRIN', '2019-02-08'),
        (2, 'Anonymised', 'Data controller or manager describes dataset as ‘anonymised’, in their interpretation of the term.', 20, 'ECRIN', '2019-02-08'),
        (3, 'Pseudonymised', 'Data controller or manager describes dataset as ‘pseudonymised’, in their interpretation of the term.', 30, 'ECRIN', '2019-02-08'),
        (4, 'Identifiable', 'Data controller or manager describes dataset as ‘identifiable’, in their interpretation of the term.', 40, 'ECRIN', '2019-02-08');"#
}


pub fn date_types<'a>() -> &'a str {

    r#"insert into lkup.date_types (id, name, description, on_papers_only,
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (15, 'Created', 'The date the resource itself was put together; this could be a date range or a single date for a final component, e.g., the finalised file with all of the data. Recommended for discovery. ', false, true, 10, 'DataCite', '2019-01-14'),
        (11, 'Accepted', 'The date that the publisher accepted the resource into their system. To indicate the start of an embargo period, use Submitted or Accepted, as appropriate.', false, false, 12, 'DataCite', '2019-01-14'),
        (12, 'Available', 'The date the resource is made publicly available. May be a range. To indicate the end of an embargo period, use Available. ', false, true, 15, 'DataCite', '2019-01-14'),
        (51, 'Controlled access in force', 'The date the resource is made available under controlled access of some form. May be a range. To indicate the end of such a period, use Available.', false, false, 18, 'ECRIN', '2019-01-14'),
        (13, 'Copyrighted', 'The specific, documented date at which the resource receives a copyrighted status, if applicable.', false, false, 20, 'DataCite', '2019-01-14'),
        (14, 'Collected', 'The date or date range in which the resource content was collected. To indicate precise or particular timeframes in which research was conducted. ', false, true, 25, 'DataCite', '2019-01-14'),
        (16, 'Issued', 'The date that the resource is published or distributed e.g. to a data centre.', false, false, 35, 'DataCite', '2019-01-14'),
        (17, 'Submitted', 'The date the creator submits the resource to a publisher. This could be different from the Accepted date if the publisher then applies a selection process. Recommended for discovery.', false, true, 40, 'DataCite', '2019-01-14'),
        (18, 'Updated', 'The date of the last update to the resource, when the resource is being added to. May be a range.', false, true, 45, 'DataCite', '2019-01-14'),
        (19, 'Valid', 'The date or date range during which the dataset or resource is accurate.', false, false, 50, 'DataCite', '2019-01-14'),
        (52, 'Pubmed citation created', 'The date in the Pubmed XML, in the element ‘DateCreated’.', true, false, 52, 'PubMed', '2019-08-24'),
        (53, 'Pubmed citation revised', 'The date in the Pubmed XML, in the element ‘DateRevised’.', true, false, 53, 'PubMed', '2019-08-24'),
        (54, 'Pubmed citation completed', 'The date in the Pubmed XML, in the element ‘DateCompleted’.', true, false, 54, 'PubMed', '2019-08-24'),
        (55, 'E-published', 'Date of electronic publication.', true, false, 55, 'PubMed', '2019-08-24'),
        (56, 'P-published', 'Date of print publication.', true, false, 56, 'PubMed', '2019-08-24'),
        (57, 'Revised', 'Date an article was revised in publication by the authors.', true, false, 57, 'PubMed', '2019-08-24'),
        (58, 'E-published ahead of print', 'Date an article was published electronically, ahead of print publication (PubMed date category).', true, false, 58, 'PubMed', '2019-08-24'),
        (59, 'Retracted', 'Date the publisher retracted an article or a resource.', true, false, 59, 'PubMed', '2019-08-24'),
        (60, 'Added to e-collection', 'Date an article was included in an electronic collection (similar to an issue).', true, false, 60, 'PubMed', '2019-08-24'),
        (61, 'Added to PMC', 'Date article was added to PMC.', true, false, 61, 'PubMed', '2019-08-24'),
        (62, 'Added to Pubmed', 'Date the citation was added to PubMed, unless the citation is added to PubMed more than twelve months since the date of publication. In that case, the PubMed date is set to the date of publication.', true, false, 62, 'PubMed', '2019-08-24'),
        (63, 'Added to Medline', 'Date the citation completed Medline processing. Up until the citation has been indexed for Medline the medline date is the same as the entrez date.', true, false, 63, 'PubMed', '2019-08-24'),
        (64, 'PMC embargo release', 'Date a full-text article was released from embargo in PubMed Central (PMC).', true, false, 64, 'PubMed', '2019-08-24'),
        (65, 'Added to entrez', 'Date when pubmed entry entered into the e-utils Entrez system.', true, false, 65, 'PubMed', '2019-08-24'),
        (90, 'Other', 'Date type not defined elsewhere.', false, true, 90, 'ECRIN', '2019-01-14'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, false, 99, 'ECRIN', '2019-01-14');"#
}


pub fn description_types<'a>() -> &'a str {

    r#"insert into lkup.description_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (20, 'Summary description', 'A textual summary of the resource (data, document or AV) and its contents', true, 5, 'ECRIN', '2022-02-17'),
        (11, 'Abstract', 'A complete abstract or summary from a document.', true, 10, 'DataCite', '2019-01-14'),
        (16, 'Abstract section', 'A labelled section in a structured abstract.', true, 12, 'PubMed', '2019-08-24'),
        (17, 'External abstract', 'Abstract not written by the authors (may be a translation).', false, 14, 'PubMed', '2019-08-24'),
        (12, 'Methods', 'A description of how the data or other object type was constructed.', true, 15, 'DataCite', '2019-01-14'),
        (13, 'Series information', 'Information about a repeating series, such as volume, issue, number.', false, 20, 'DataCite', '2019-01-14'),
        (14, 'Table of contents', 'A listing of the Table of Contents.', true, 30, 'DataCite', '2019-01-14'),
        (15, 'Technical information', 'Detailed information that may be associated with design, implementation, operation, use, and/or maintenance of a process or system. Includes population information for a clinical research dataset.', true, 35, 'DataCite', '2019-01-14'),
        (18, 'Journal source string', 'The bibliographic reference to a journal article - the string that normally follows authors and title.', true, 40, 'PubMed', '2019-08-24'),
        (19, 'Data availability description', 'A description of the location of a Dryad dataset', false, 45, 'PubMed', '2019-09-15'),
        (90, 'Other', 'A description not falling into any of the other categories.', true, 90, 'ECRIN', '2019-01-14'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 99, 'ECRIN', '2019-01-14');"#
}

    
pub fn doi_status_types<'a>() -> &'a str {

    r#"insert into lkup.doi_status_types (id, name, description, 
          list_order, source_org, date_added)
       values 
        (1, 'Exists in MDR system', 'This value should be accompanied by a valid DOI in the data object record.', 10, 'ECRIN', '2019-02-08'),
        (3, 'Exists but not yet in MDR system', 'There is good evidence that a DOI exists but has yet to be captured.', 20, 'ECRIN', '2019-02-08'),
        (5, 'Does not yet exist but is expected', 'Reasonable to expect that a DOI will be minted in the future.', 30, 'ECRIN', '2019-02-08'),
        (9, 'Does not exist, not currently expected', 'No indication exists that a DOI will be minted for this object.', 40, 'ECRIN', '2019-02-08'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', 99, 'ECRIN', '2019-02-08');"#
}


pub fn gender_eligibility_types<'a>() -> &'a str {

    r#"insert into lkup.gender_eligibility_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
       (1, 'Female', 'Study recruits only female participants', true, 10, 'ECRIN', '2025-03-01'),
       (2, 'Male', 'Study recruits only male participants', true, 20, 'ECRIN', '2025-03-01'),
       (3, 'Both', 'Study open to both male and female participants', true, 30, 'ECRIN', '2025-03-01');"#
}


pub fn identifier_types<'a>() -> &'a str {

    r#"insert into lkup.identifier_types (id, name, applies_to, description, 
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
        (53, 'BBMRI Collection Id', 'Data Object', 'The Id assigned by BBMRI to a collection of samples', false, 60, 'BBMRI', '2023-09-02'),
        (15, 'Repository accession number', 'Data Object', 'Org id should identify the repository manager.', false, 105, 'ECRIN', '2019-01-14'),
        (16, 'PMID', 'Data Object', 'Citation ID assigned by PubMed.', true, 110, 'PubMed', '2019-01-14'),
        (31, 'PMCID', 'Data Object', 'Pub med central ID, of the manuscript itself.', true, 115, 'PubMed', '2019-01-14'),
        (32, 'NIH manuscript ID', 'Data Object', 'A Manuscript ID, an identifier assigned to an author manuscript submitted to the NIH Manuscript Submission System. ', false, 120, 'PubMed', '2019-02-08'),
        (36, 'Medline UID', 'Data Object', 'Medline identifier.', false, 125, 'PubMed', '2019-02-08'),
        (34, 'Publisher article ID', 'Data Object', 'Internal reference of article publisher for journal article.', true, 130, 'PubMed', '2019-08-24'),
        (37, 'PMC publisher ID', 'Data Object', 'Publisher Id supplied to PubMed Central.', false, 132, 'PubMed', '2019-08-24'),
        (38, 'PM publisher ID', 'Data Object', 'Publisher Id supplied to PubMed.', false, 134, 'PubMed', '2019-08-24'),
        (17, 'bioRxiv ID', 'Data Object', 'Pre-prints, ID assigned by Cold Spring Harbour Laboratory.', true, 140, 'ECRIN', '2019-02-08'),
        (18, 'arXiv ID', 'Data Object', 'Pre-prints, ID assigned by Cornell university.', true, 145, 'DataCite', '2019-02-08'),
        (19, 'psyXiv ID', 'Data Object', 'Pre-prints, ID assigned by the Centre for Open Science.', true, 150, 'ECRIN', '2019-02-08'),
        (20, 'socXiv ID', 'Data Object', 'Pre-prints, ID assigned by the Centre for Open Science.', true, 155, 'ECRIN', '2019-02-08'),
        (21, 'Handle ID', 'Data Object', 'ID assigned by a naming authority (handle system is a superset of DOIs).', false, 165, 'DataCite', '2019-02-08'),
        (22, 'ISBN', 'Data Object', 'International Standard Book Number, assigned by the publisher.', false, 170, 'DataCite', '2019-02-08'),
        (23, 'ISTC', 'Data Object', 'International Standard Text code ID, assigned by the international ISTC agency.', false, 175, 'DataCite', '2019-02-08'),
        (24, 'ISAN', 'Data Object', 'International Standard Audiovisual Number, assigned by the ISAN international agency.', false, 180, 'ECRIN', '2019-02-08'),
        (25, 'LSID', 'Data Object', 'Life Science Identifier, a URN like specification with various issuing authorities.', false, 185, 'DataCite', '2019-02-08'),
        (26, 'Other bibliographic ID', 'Data Object', 'Org id should identify the system manager.', false, 190, 'ECRIN', '2019-02-08'),
        (33, 'NRCBL', 'Data Object', 'KIE Reference Library (bioethics library) shelving location.', false, 195, 'PubMed', '2019-02-08'),
        (35, 'Serial item and contribution identifier ', 'Data Object', 'The Serial Item and Contribution Identifier (SICI), a code used to uniquely identify specific volumes, articles or other identifiable parts of a serial. ', false, 197, 'PubMed', '2019-08-24'),
        (27, 'URL', 'All', 'Resource locator for a web based resource.', true, 205, 'DataCite', '2019-02-08'),
        (28, 'PURL', 'All', 'Persistent URL that redirects if necessary.', false, 210, 'DataCite', '2019-02-08'),
        (29, 'URN', 'All', 'Uniform Resource Name (a URI using the URN schema) that is location independent.', false, 220, 'DataCite', '2019-02-08'),
        (30, 'ARK', 'All', 'Archival Resource Key, a URL that provides additional metadata.', false, 225, 'DataCite', '2019-02-08'),
        (90, 'Other', 'All', 'None of the listed identifier types.', true, 990, 'ECRIN', '2019-01-14'),
        (0, 'Not yet known', 'All', 'Dummy value supplied by default on entity creation.', false, 990, 'ECRIN', '2019-01-14'),
        (1, 'Type not provided', 'All', 'Missing type data in data source.', false, 998, 'ECRIN', '2019-11-03');"#
}

pub fn iec_level_types<'a>() -> &'a str {

    r#"insert into lkup.iec_level_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (0, 'None', 'No inclusion / exclusion material in source material', true, 10, 'ECRIN', '2025-03-15'),
        (1, 'Single statement', 'A single statement, without carriage returns. May not be headed. If headed, often as ‘eligibility’, rather than ‘inclusion’', true, 20, 'ECRIN', '2025-03-15'),
        (2, 'Single paragraph', 'With internal carriage returns, implying multiple statements. The paragraph often labelled ‘eligibility’ or equivalent, rather than ‘inclusion’.', true, 30, 'ECRIN', '2025-03-15'),
        (4, 'Single inclusion statement', 'Includes the word inclusion but without carriage returns. May be structured into separate criteria or may be a sigle statement.', true, 40, 'ECRIN', '2025-03-15'),
        (8, 'Inclusion paragraph', 'Includes the word inclusion and contains carriage returns. Assumed that different lines represent different criteria.', true, 50, 'ECRIN', '2025-03-15'),
        (16, 'Single exclusion statement', 'Includes the word exclusion but without carriage returns. May be structured into separate criteria or may be a sigle statement.', true, 60, 'ECRIN', '2025-03-15'),
        (32, 'Exclusion paragraph', 'Includes the word exclusion and contains carriage returns. Assumed that different lines represent different criteria.', true, 70, 'ECRIN', '2025-03-15'),
        (20, 'Single inclusion + Single exclusion', 'A single statement for each of inclusion and exclusion', true, 80, 'ECRIN', '2025-03-15'),
        (36, 'Single inclusion + Multiple exclusion', 'A single statement for inclusion criteria but multiple exclusion criteria', true, 90, 'ECRIN', '2025-03-15'),
        (24, 'Multiple inclusion + Single exclusion', 'Multiple inclusion criteria but with a single statement covering exclusion', true, 100, 'ECRIN', '2025-03-15'),
        (40, 'Multiple inclusion + exclusion', 'Sets of both inclusion and exclusion criteria statements', true, 110, 'ECRIN', '2025-03-15');"#
}

pub fn language_usage_types<'a>() -> &'a str {

    r#"insert into lkup.language_usage_types (id, name, description, 
          list_order, source_org, date_added)
       values 
        (11, 'Original in English', 'Originally written in English.', 10, 'ECRIN', '2019-08-24'),
        (12, 'Translated', 'Translated into this language (almost always English).', 20, 'ECRIN', '2019-08-24'),
        (13, 'Transliterated', 'Transliterated into Latin characters in this language.', 30, 'ECRIN', '2019-08-24'),
        (21, 'Original (explicitly non English)', 'Explicitly listed as a title or text ‘in the vernacular’.', 40, 'ECRIN', '2019-09-20'),
        (22, 'Original (presumed non English)', 'Presumed as a non English title or text from context or content.', 50, 'ECRIN', '2019-09-20'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', 99, 'ECRIN', '2019-08-24');"#
}


pub fn object_access_types<'a>() -> &'a str {

    r#"insert into lkup.object_access_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (12, 'Public on-screen access', 'Completely open access on the web but content not available in any other format', false, 10, 'ECRIN', '2019-01-14'),
        (20, 'Public on-screen and API access', 'Completely open access, material viewable and also available through an API', false, 12, 'ECRIN', '2019-11-01'),
        (11, 'Public on-screen access and download', 'Completely open access, material viewable and also directly available as a file download', true, 14, 'ECRIN', '2019-01-14'),
        (13, 'Public download (self-attestation required)', 'Public and downloadable (only) once the user identifies and / or describes themselves.', false, 20, 'ECRIN', '2019-01-14'),
        (14, 'Public on-screen access (self-attestation required)', 'Public and viewable (only) once the user identifies and / or describes themselves.', false, 25, 'ECRIN', '2019-01-14'),
        (15, 'Restricted download', 'The user is an authenticated member of a defined group and can view / download the material (includes pay-walled journal articles).', true, 30, 'ECRIN', '2019-01-14'),
        (16, 'Restricted on-screen access', 'The user is an authenticated member of a defined group but can only view the material. Analysis tools may be available.', true, 35, 'ECRIN', '2019-01-14'),
        (17, 'Case by case access', 'Based on a review of an individual request, often also requiring supporting documentation.', true, 40, 'ECRIN', '2019-01-14'),
        (18, 'Case by case on-screen access', 'Based on a review of an individual request, often also requiring supporting documentation. Analysis tools may be available.', true, 45, 'ECRIN', '2019-01-14'),
        (19, 'Non public access - no details', 'The site asserts that the resource is not publicly available but provides no further details.', false, 50, 'ECRIN', '2019-01-14'),
        (90, 'Other', 'None of the listed options.', true, 90, 'ECRIN', '2019-01-14'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 99, 'ECRIN', '2019-01-14');"#
}


pub fn object_classes<'a>() -> &'a str {

    r#"insert into lkup.object_classes (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (23, 'Text', 'A resource consisting primarily of words for reading, includes grey literature, lab notes, accompanying materials as well as published articles.', true, 5, 'DataCite', '2019-01-14'),
        (12, 'Collection', 'An aggregation of resources, which may encompass collections of one resource type as well as those of mixed types. A collection is described as a group; its parts may also be separately described, e.g. a collection of samples, or various files making up a report.', false, 15, 'DataCite', '2019-01-14'),
        (14, 'Dataset', 'Data encoded in a defined structure. A data file or files.', true, 25, 'DataCite', '2019-01-14'),
        (16, 'Image', 'A visual representation other than text, e.g. digitised or born digital images, drawings or photographs. ', true, 30, 'DataCite', '2019-01-14'),
        (15, 'Event', 'A non-persistent, time based occurrence. Descriptive information and/or content that is the basis for discovery of the purpose, location, duration, and responsible agents associated with an event such as a webcast or convention.', false, 30, 'DataCite', '2019-01-14'),
        (11, 'Audiovisual', 'A series of visual representations imparting an impression of motion when shown in succession. May or may not include sound. May be used for films, video, etc.', true, 35, 'DataCite', '2019-01-14'),
        (22, 'Sound', 'A resource primarily intended to be heard, e.g. an audio recording.', true, 38, 'DataCite', '2019-01-14'),
        (21, 'Software', 'A computer program in source code (text) or compiled form. Use this type for all software components supporting scholarly research.', true, 40, 'DataCite', '2019-01-14'),
        (18, 'Model', 'An abstract, conceptual, graphical, mathematical or visualization model that represents empirical objects, phenomena, or physical processes, e.g. modelled descriptions of different aspects of languages or a molecular biology reaction chain.', true, 45, 'DataCite', '2019-01-14'),
        (19, 'Physical object', 'An inanimate, three dimensional object or substance, e.g. artefacts, specimens.', false, 50, 'DataCite', '2019-01-14'),
        (20, 'Service', 'An organized system of apparatus, appliances, staff, etc., for supplying some function(s) required by end users, e.g. a data management service, or long term preservation service.  ', false, 55, 'DataCite', '2019-01-14'),
        (24, 'Workflow', 'A structured series of steps which can be executed to produce a final outcome, allowing users a means to specify and enact their work in a more reproducible manner.', false, 75, 'DataCite', '2019-01-14'),
        (17, 'Interactive resource', 'A resource requiring interaction from the user to be understood, executed, or experienced, e.g. training modules, files that require use of a viewer, or query/response portals.', true, 90, 'DataCite', '2019-01-14'),
        (13, 'Data paper', 'A factual and objective publication with a focused intent to identify and describe specific data, sets of data, or data collections to facilitate discoverability. A data paper describes data provenance and methodologies used in the gathering, processing, organizing, and representing the data. ', false, 120, 'DataCite', '2019-01-14'),
        (90, 'Other', 'Object class not listed elsewhere.', true, 998, 'DataCite', '2019-01-14'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999, 'ECRIN', '2019-02-08');"#
}


pub fn object_filter_types<'a>() -> &'a str {

    r#"insert into lkup.object_filter_types (id, filter_as, description, 
          list_order, source_org, date_added)
       values
        (11, 'Trial registry entry', 'T13 - Summary of the study and its aims, posted prospectively or retrospectively to a public registry (not currently filtered in the UI).', 10, 'ECRIN', '2020-04-10'),
        (12, 'Registry results summary', 'T28  - Summary of study results, as displayed in a trial registry.', 20, 'ECRIN', '2020-04-10'),
        (21, 'Journal article', 'T12 - Articles in peer-reviewed publications that disseminate the results of original research and scholarship; T100 - Journal article abstract; T117 - Special journal issues; T135 - Working Paper / Pre-print; T152 - Grouped journal articles; 201 - Journal article - protocol; 202 - Journal article - results; 203 - Journal article - interim results; 204 - Journal article - review; 210 - Preprint article.', 30, 'ECRIN', '2020-04-10'),
        (22, 'Study protocol', 'T11 - Protocol: Structured document describing the study, its rationale, methodology, outcome measures etc; T42 - Redacted protocol; T74 - Protocol with SAP; T75 - Protocol with informed consent forms; T76 - Protocol with SAP and ICFs; 201 - Journal article - protocol.', 50, 'ECRIN', '2020-04-10'),
        (23, 'Study overview', 'T38 - A brief overview of the study, may be an abridged protocol, or as used within the study web site or other study documents.', 55, 'ECRIN', '2020-04-10'),
        (24, 'Patient consent/information forms', 'T18 - Informed consent forms, the form or forms given to participants to formally record their informed consent to study participation; T19 - Patient information sheets, the information provided to study participants, especially as part of the consenting process; 75 - Protocol with informed consent forms; 76 - Protocol with SAP and ICFs.', 60, 'ECRIN', '2020-04-10'),
        (25, 'Data collection forms', 'T21 - Data collection forms, Copies, in electronic and / or paper form, of the case report forms (CRFs and / or eCRFs) used for collecting data. T30 - Annotated copies of CRFs / eCRFs; T40 - Standard instruments: standardised rating instruments, including questionnaires.', 70, 'ECRIN', '2020-04-10'),
        (26, 'Manual of procedures', 'T35 - Manual of Operations, T36 - Manual of Procedures. Description of specific operations, workflow, procedures and techniques within the study.', 80, 'ECRIN', '2020-04-10'),
        (31, 'Statistical analysis plan', 'T22 - Statistical analysis plan: the details of the proposed analysis for the study, listing the individual descriptive statistics and tests of inference, and their parameters; T29 - Analysis notes: a summary of the analysis carried out and / or any caveats to be borne in mind when interpreting results; T43 - a redacted SAP; 74 - Protocol with SAP; 76 - Protocol with SAP and ICFs.', 90, 'ECRIN', '2020-04-10'),
        (32, 'Clinical study report', 'T26 - Clinical study report - a full end of study report with detailed efficacy and safety results; T27 - a redacted clinical study report; T79 - a summary CSR; T85 - Unpublished Study Report: A report of a study, or part of a study, not formally published. May be an internal interim document within a long term study.', 100, 'ECRIN', '2020-04-10'),
        (33, 'Data description', 'T20 - Database specification: Functional specification of the database including details of individual data items, types, ranges, logic checks, etc.; T31 - Data dictionary: A detailed, item by item, description of the data points in the dataset; T32 - Data Overview: A summary of the data indicating the nature of different tables, time points of data etc; T73 - IPD or aggregate data metadata definition;  T81 - Data collection schedule; T82 - Data coding manual.', 110, 'ECRIN', '2020-04-10'),
        (34, 'IP or Aggregated Data', 'T80 - Individual participant data: A dataset simply called Individual Participant Data, or its equivalent, with no further qualification or description. Any of T51 - T68, being different specific types of IPD, e.g. relating to sub-populations or different time points, or metadata associated with IPD. T69 - Aggregated result dataset: Dataset with aggregated / summary results and statistics from the study. T70, T71, T72, being different specific types of aggregate data. Also T153 - Grouped analysis datasets, and T154 - Grouped aggregate datasets.', 120, 'ECRIN', '2020-04-10'),
        (53, 'Samples description', 'T301 - Associated biosamples (link is to sample description)', 130, 'ECRIN', '2023-07-04'),
        (36, 'Other study resource', 'T14 - Ethics submission; T15 - Ethics approval notification; T16 - Regulatory authority submission; T17 - Regulatory authority approval notification; T23 - Data management plan; T24 - Trial master file contents list; T25 - Data monitoring committee report; T33 - Definitions of terms; T34 - Literature Review; T39 - Publication List; T77 - Investigational Product Information; T78 - General background to research topic; T83 - Bibliography; T84 - Introduction to document set; T86 - List of web links; T115 - Funding Submission; T171 - Other dataset object: any dataset class resource not defined elsewhere.', 140, 'ECRIN', '2020-04-10'),
        (41, 'Other informational material', 'T88 - Summary of results for public; T106 - Conference Abstract; T107 - Conference Paper; T108 - Conference Poster: T109 - Conference Program; T119 - Magazine Article; T121 - Newsletter Article; T122 - Newspaper Article; T101 - Book; T102 - Book chapter: T103 - Book Prospectus; T104 - Book Review; T105 - Book Series; ;T112 - Dissertation;  T113 - Edited Book;  T114 -Encyclopaedia Entry; T120 - Manual (for education / training purposes); T123 - Online Resource; T126 - Report; T127 - Research Tool; T128 - Supervised Student Publication.', 150, 'ECRIN', '2020-04-10'),
        (51, 'Website', 'T134 - Stand-alone locations on the web where multiple types of information on a specific theme are available. May include interactive features for contributions from readers.', 190, 'ECRIN', '2020-04-10'),
        (52, 'Software', 'T166 - Script(s) used in analysis; T167 - CDMS (Clinical Data Management System); T168 - Trial Management System; T169 - eTMF; T170 - Data Extraction System.', 200, 'ECRIN', '2020-04-10'),
        (99, 'Other', 'All types in each object class labelled as Other (T37, T151, T155 - T165).', 210, 'ECRIN', '2020-04-10');"#
}


pub fn object_relationship_types<'a>() -> &'a str {

    r#"insert into lkup.object_relationship_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (11, 'Is cited by', 'Indicates that B includes A in a citation.', false, 10, 'DataCite', '2019-01-14'),
        (12, 'Cites', 'Indicates that A includes B in a citation.', false, 15, 'DataCite', '2019-01-14'),
        (13, 'Is supplement to', 'Indicates that A is a supplement to B.', true, 20, 'DataCite', '2019-01-14'),
        (14, 'Is supplemented by', 'Indicates that B is a supplement to A.', true, 25, 'DataCite', '2019-01-14'),
        (15, 'Is continued by', 'Indicates A is continued by the work B.', false, 30, 'DataCite', '2019-01-14'),
        (16, 'Continues', 'Indicates A is a continuation of the work B.', false, 35, 'DataCite', '2019-01-14'),
        (17, 'Is described by', 'Indicates A is described by B.', true, 40, 'DataCite', '2019-01-14'),
        (18, 'Describes', 'Indicates A describes B.', true, 45, 'DataCite', '2019-01-14'),
        (19, 'Has metadata', 'Indicates resource A has associated metadata B.', true, 50, 'DataCite', '2019-01-14'),
        (20, 'Is metadata for', 'Indicates is metadata for resource B.', true, 55, 'DataCite', '2019-01-14'),
        (21, 'Has version', 'Indicates A has a version B.', true, 60, 'DataCite', '2019-01-14'),
        (22, 'Is version of', 'Indicates A is a version of B.', true, 65, 'DataCite', '2019-01-14'),
        (23, 'Is new version of', 'Indicates A is a new edition of B, where the new edition has been modified or updated.', false, 70, 'DataCite', '2019-01-14'),
        (24, 'Is previous version of', 'Indicates A is a previous edition of B.', false, 75, 'DataCite', '2019-01-14'),
        (25, 'Is part of', 'Indicates A is a portion of B; may be used for elements of a series.', false, 80, 'DataCite', '2019-01-14'),
        (26, 'Has part', 'Indicates A includes the part B.', false, 85, 'DataCite', '2019-01-14'),
        (27, 'Is referenced by', 'Indicates A is used as a source of information by B.', false, 90, 'DataCite', '2019-01-14'),
        (28, 'References', 'Indicates B is used as a source of information for A.', false, 95, 'DataCite', '2019-01-14'),
        (29, 'Is documented by', 'Indicates B is documentation about/ explaining A; e.g. points to software documentation.', true, 100, 'DataCite', '2019-01-14'),
        (30, 'Documents', 'Indicates A is documentation about B; e.g. points to software documentation.', true, 105, 'DataCite', '2019-01-14'),
        (31, 'Is compiled by', 'Indicates B is used to compile or create A.', false, 110, 'DataCite', '2019-01-14'),
        (32, 'Compiles', 'Indicates B is the result of a compile or creation event using A.', false, 115, 'DataCite', '2019-01-14'),
        (33, 'Is variant form of', 'Indicates A is a variant or different form of B.', false, 120, 'DataCite', '2019-01-14'),
        (34, 'Is original form of', 'Indicates A is the original form of B.', false, 125, 'DataCite', '2019-01-14'),
        (35, 'Is identical to', 'Indicates that A is identical to B, for use when there is a need to register two separate instances of the same resource.', false, 130, 'DataCite', '2019-01-14'),
        (36, 'Is reviewed by', 'Indicates that A is reviewed by B.', false, 135, 'DataCite', '2019-01-14'),
        (37, 'Reviews', 'Indicates that A is a review of B.', false, 140, 'DataCite', '2019-01-14'),
        (38, 'Is derived From', 'Indicates B is a source upon which A is based.', true, 145, 'DataCite', '2019-01-14'),
        (39, 'Is source of', 'Indicates A is a source upon which B is based.', true, 150, 'DataCite', '2019-01-14'),
        (40, 'Is required by', 'Indicates A is required by B (may be used to indicate software dependencies).', false, 155, 'DataCite', '2019-01-14'),
        (41, 'Requires', 'Indicates A requires B (may be used to indicate software dependencies).', false, 160, 'DataCite', '2019-01-14'),
        (42, 'Obsoletes', 'Indicates A replaces B.', true, 165, 'DataCite', '2019-08-24'),
        (43, 'Is obsoleted by', 'Indicates A is replaced by B.', true, 170, 'DataCite', '2019-08-24'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation', false, 999, 'ECRIN', '2019-01-14');"#
}


pub fn object_types<'a>() -> &'a str {

    r#"insert into lkup.object_types (id, name, object_class_id,
          filter_as_id, description, use_in_data_entry, list_order, source_org, date_added)
       values 
        (13, 'Trial registry entry', 23, '11', 'Summary of the study and its aims, posted prospectively or retrospectively to a public registry.', false, 5, 'ECRIN', '2019-01-14'),
        (11, 'Study protocol', 23, '22', 'Structured document describing the study, its rationale, methodology, outcome measures etc.', true, 7, 'ECRIN', '2019-01-14'),
        (12, 'Journal article - unspecified', 23, '21', 'Article in a peer-reviewed publication - exact relationship to the research study not known.', true, 10, 'CASRAI', '2019-01-14'),
        (202, 'Journal article - results', 23, '21', 'Article in a peer-reviewed publication - specifically describing the results of a study.', true, 12, 'ISRCTN', '2023-01-16'),
        (201, 'Journal article - protocol', 23, '21, 22', 'Article in a peer-reviewed publication - describing the protocol of a study.', true, 14, 'ISRCTN', '2023-01-16'),
        (203, 'Journal article - interim results', 23, '21', 'Article in a peer-reviewed publication - specifically describing interim results of a study, or results of a feasibility study.', true, 15, 'ECRIN', '2023-01-19'),
        (204, 'Journal article - review', 23, '21', 'Article in a peer-reviewed publication - providing a review of arelated scientific area or a summary funder based report.', true, 16, 'ECRIN', '2023-01-19'),
        (210, 'Preprint article', 23, '21', 'Article in a preprint journal.', true, 19, 'ECRIN', '2023-01-19'),
        (38, 'Study overview', 23, '23', 'A brief overview of the study, may be an abridged protocol, or as used within the study web site or other study documents.', true, 20, 'BioLINCC', '2019-03-27'),
        (14, 'Ethics submission', 23, '36', 'Documents provided to an ethics review board, often with the protocol, when seeking ethical approval for a study.', true, 22, 'ECRIN', '2019-01-14'),
        (15, 'Ethics approval notification', 23, '36', 'Documents from an ethical review board confirming that ethical approval has been granted.', true, 25, 'ECRIN', '2019-01-14'),
        (16, 'Regulatory authority submission', 23, '36', 'Documents provided to a regulatory authority, seeking approval to run a clinical trial.', true, 30, 'ECRIN', '2019-01-14'),
        (17, 'Regulatory authority approval notification', 23, '36', 'Documents from a regulatory authority confirming that approval to run the trial has been granted.', true, 35, 'ECRIN', '2019-01-14'),
        (35, 'Manual of operations', 23, '26', 'Description of specific operations and workflow within the study.', true, 37, 'BioLINCC', '2019-03-27'),
        (36, 'Manual of procedures', 23, '26', 'Description of specific procedures and techniques used within the study.', true, 38, 'BioLINCC', '2019-03-27'),
        (18, 'Informed consent forms', 23, '24', 'The form or forms given to participants to formally record their informed consent to study participation.', true, 40, 'ECRIN', '2019-01-14'),
        (19, 'Patient information sheets', 23, '24', 'The information provided to study participants, especially as part of the consenting process.', true, 45, 'ECRIN', '2019-01-14'),
        (32, 'Data overview', 23, '33', 'A summary of the data, without the details of a data dictionary but indicating the nature of different tables, time points of data etc.', true, 48, 'BioLINCC', '2019-03-27'),
        (20, 'Database specification', 23, '33', 'Functional specification of the database including details of individual data items, types, ranges, etc. May also contain details of logic and consistency checks, and any derived values.', true, 50, 'ECRIN', '2019-01-14'),
        (31, 'Data dictionary', 23, '33', 'A detailed, item by item, description of the data points in the dataset, sufficient for accurate analysis of the data by others.', true, 52, 'BioLINCC', '2019-03-27'),
        (21, 'Data collection forms', 23, '25', 'Copies, in electronic and / or paper form, of the case report forms (CRFs and / or eCRFs) used for collecting data.', true, 55, 'ECRIN', '2019-01-14'),
        (30, 'Annotated data collection forms', 23, '25', 'Data Collection forms (CRFs or eCRFs) annotated to provide further details of each item (e.g. data type, allowable range).', true, 56, 'BioLINCC', '2019-03-27'),
        (40, 'Standard instruments', 23, '25', 'Standardised rating instruments, including questionnaires.', true, 58, 'BioLINCC', '2019-03-27'),
        (22, 'Statistical analysis plan', 23, '31', 'The details of the proposed analysis for the study, listing the individual descriptive statistics and tests of inference, and their parameters.', true, 60, 'ECRIN', '2019-01-14'),
        (29, 'Analysis notes', 23, '31', 'A summary of the analysis carried out and / or any caveats to be borne in mind when interpreting results. Less formal than a statistical analysis plan.', true, 61, 'BioLINCC', '2019-03-27'),
        (74, 'Protocol SAP', 23, '22, 31', 'Study Protocol and Statistical Analysis Plan, combined in one document.', true, 62, 'ClinicalTrials.gov', '2019-11-03'),
        (75, 'Protocol ICF', 23, '22, 24', 'Study Protocol and Informed Consent Form, combined in one document.', true, 63, 'ClinicalTrials.gov', '2019-11-03'),
        (76, 'Protocol SAP ICF', 23, '22, 24, 31', 'Study Protocol, Statistical Analysis Plan, and Informed Consent Form, all combined in one document.', true, 64, 'ClinicalTrials.gov', '2019-11-03'),
        (23, 'Data management plan', 23, '36', 'A plan for, and record of, data management activities in the study, covering the whole data life cycle.', true, 65, 'ECRIN', '2019-01-14'),
        (33, 'Definitions', 23, '36', 'A glossary list for use with other study documents.', true, 67, 'BioLINCC', '2019-03-27'),
        (24, 'Trial master file contents list', 23, '36', 'A listing of the documents expected within a trial master file, and their organisation, in electronic and / or paper form.', false, 70, 'ECRIN', '2019-01-14'),
        (81, 'Data collection schedule', 23, '33', 'A document detailing the time points of data collection in a study (or ‘visits’).', true, 72, 'BioLINCC', '2019-12-14'),
        (82, 'Data coding manual', 23, '33', 'A manual or guide that provides instructions on how to complete and / or interpret scores and codes within a study.', true, 74, 'BioLINCC', '2019-12-14'),
        (25, 'Data monitoring committee report', 23, '36', 'A report concerning the safety or efficacy of a study, from independent experts. Often containing recommendations about the continuation of the study.', false, 75, 'ECRIN', '2019-01-14'),
        (83, 'Bibliography', 23, '36', 'A list of publications making up a bibliography relevant to the study, but not necessarily generated or triggered by the study.', false, 76, 'BioLINCC', '2019-12-14'),
        (84, 'Introduction to document set', 23, '36', 'A ‘contents’, ‘readme’ or similar document that describes the other documents available.', true, 78, 'BioLINCC', '2019-12-14'),
        (26, 'Clinical study report', 23, '32', 'Full end of study report with detailed efficacy and safety results.', true, 80, 'ECRIN', '2019-01-14'),
        (85, 'Unpublished study report', 23, '32', 'A report of a study, or part of a study, not formally published. May be an internal interim document within a long term study.', true, 80, 'BioLINCC', '2019-12-14'),
        (88, 'Summary of results for public', 23, '32', 'Summary of results specifically targeted at participants and members of the public, usually as a web page', true, 82, 'ISRCTN', '2022-06-01'),
        (79, 'Results or CSR summary', 23, '32', 'A results summary, or a  summary derived from  a Clinical Study Report', true, 83, 'ClinicalTrials.gov', '2019-11-07'),
        (27, 'Redacted clinical study report', 23, '32', 'End of study report with some data withheld, usually because of commercial sensitivity.', true, 85, 'ECRIN', '2019-01-14'),
        (28, 'Trial registry results summary', 23, '12', 'Summary of study results, as displayed in a trial registry.', false, 87, 'ECRIN', '2019-01-14'),
        (86, 'List of web links', 23, '36', 'A web page that includes a list of links to different items, e.g. individual CRFs.', false, 89, 'BioLINCC', '2019-12-14'),
        (34, 'Literature review', 23, '36', 'Publications referenced within the literature review undertaken prior to the study.', false, 90, 'BioLINCC', '2019-03-27'),
        (39, 'Publication list', 23, '36', 'List of publications related to the study.', true, 91, 'BioLINCC', '2019-03-27'),
        (77, 'Investigational product information', 23, '36', 'Summary of information about a medicinal product. May be a package insert or investigator’s brochure.', true, 92, 'ClinicalTrials.gov', '2019-11-04'),
        (78, 'General background paper', 23, '36', 'Supporting document summarising relevant research and / or research programs, or aspects of condition pathology, epidemiology, etc.', true, 93, 'ClinicalTrials.gov', '2019-11-04'),
        (42, 'Redacted protocol', 23, '22', 'A redacted version of the study protocol.', false, 94, 'ClinicalTrials.gov', '2019-11-21'),
        (43, 'Redacted SAP', 23, '31', 'A redacted version of the statistical analysis plan.', false, 95, 'ClinicalTrials.gov', '2019-11-21'),
        (80, 'Individual participant data', 14, '34', 'A dataset simply called Individual Participant Data, or its equivalent, with no further qualification or description', true, 97, 'ClinicalTrials.gov', '2019-11-07'),
        (51, 'IPD final analysis datasets (full study population)', 14, '34', 'Full final dataset supporting all analyses carried out on the study data.', true, 98, 'ECRIN', '2019-01-14'),
        (301, 'Sample Description', 19, '53', 'A description of a bio-sample collection associated with or generated by a study. The URL is to a summary description of the biosamples with further details on access', true, 99, 'ECRIN', '2023-07-04'),
        (52, 'IPD final analysis datasets (supporting specific paper)', 14, '34', 'Dataset supporting the analyses and conclusions of a specific single paper.', true, 99, 'ECRIN', '2019-01-14'),
        (53, 'IPD final analysis datasets (sub-population)', 14, '34', 'Dataset with the data from a sub-population of the complete study (e.g. the control arm, or a particular age group).', true, 100, 'ECRIN', '2019-02-08'),
        (54, 'IPD final analysis datasets (sub-study)', 14, '34', 'Dataset with the data of a sub-study, supporting the analysis only of that sub-study - may also involve a sub-population of the whole.', true, 103, 'ECRIN', '2019-02-08'),
        (56, 'IPD interim analysis dataset (sub-population)', 14, '34', 'Dataset with the data from a sub-population of the complete study, from an earlier time point than the primary analysis.', false, 104, 'ECRIN', '2019-02-08'),
        (57, 'IPD interim analysis dataset (sub-study)', 14, '34', 'Dataset with the data of a sub-study, supporting the analysis only of that sub-study, from an earlier time point than the primary analysis.', false, 105, 'ECRIN', '2019-02-08'),
        (58, 'IPD long term follow up analysis dataset', 14, '34', 'Supplementary dataset with additional data from long term follow up, data collected after primary analysis.', false, 106, 'ECRIN', '2019-02-08'),
        (55, 'IPD interim analysis dataset', 14, '34', 'Dataset covering the whole study but from an earlier time point than the primary analysis.', false, 110, 'ECRIN', '2019-01-14'),
        (59, 'IPD safety analysis dataset', 14, '34', 'Dataset of individual data supporting comprehensive safety analysis.', false, 115, 'ECRIN', '2019-02-08'),
        (60, 'IPD PK analysis dataset', 14, '34', 'Dataset of pharmaco-kinetic data.', false, 120, 'ECRIN', '2019-02-08'),
        (61, 'IPD PD analysis dataset', 14, '34', 'Dataset of pharmaco-dynamic data.', false, 125, 'ECRIN', '2019-02-08'),
        (62, 'IPD quality of life analysis dataset', 14, '34', 'Dataset supporting analysis of quality of life measures within the study.', false, 130, 'ECRIN', '2019-02-08'),
        (63, 'IPD analysis dataset, other', 14, '34', 'Analysis dataset not listed in other options.', false, 135, 'ECRIN', '2019-02-08'),
        (64, 'IPD analysis dataset metadata definition', 14, '34', 'A dataset that is the metadata for an analysis dataset.', true, 140, 'ECRIN', '2019-02-08'),
        (65, 'IPD CDMS format dataset', 14, '34', 'A dataset in the format used by the data collection system (CDMS). May be a database file.', false, 145, 'ECRIN', '2019-02-08'),
        (66, 'IPD CDMS format dataset metadata definition', 14, '34', 'The metadata definition for the study’s data collection system - equivalent to the functional specification of the system but as a dataset rather than a document.', false, 150, 'ECRIN', '2019-02-08'),
        (67, 'IPD transport format dataset', 14, '34', 'A dataset in a format designed specifically for transfer between systems.', true, 155, 'ECRIN', '2019-02-08'),
        (68, 'IPD transport format dataset metadata definition', 14, '34', 'Metadata describing the dataset when in a transport format.', true, 160, 'ECRIN', '2019-02-08'),
        (69, 'Aggregated result dataset', 14, '34', 'Dataset with aggregated / summary results and statistics from the study.', true, 165, 'ECRIN', '2019-02-08'),
        (70, 'Aggregated result dataset, efficacy measures', 14, '34', 'Dataset with aggregated results and statistics focusing on efficacy measures.', false, 170, 'ECRIN', '2019-02-08'),
        (71, 'Aggregated result dataset, safety measures', 14, '34', 'Dataset with aggregated results and statistics focusing on safety measures.', false, 175, 'ECRIN', '2019-02-08'),
        (72, 'Aggregated result dataset, other', 14, '34', 'Dataset with aggregated results and statistics, not listed elsewhere.', false, 180, 'ECRIN', '2019-02-08'),
        (73, 'Metadata definition', 23, '33', 'Metadata definition, for IPD and / or an aggregated result set, when a separate file.', true, 185, 'ECRIN', '2019-02-08'),
        (101, 'Book', 23, '41', 'Books written by a single author or collaboratively based on research or scholarly findings generally derived from peer reviewed funding.', false, 190, 'CASRAI', '2019-02-08'),
        (102, 'Book chapter', 23, '41', 'Texts written by a single author or collaboratively based on research or scholarly findings and expertise in a field.', false, 195, 'CASRAI', '2019-02-08'),
        (103, 'Book prospectus', 23, '41', 'ocument that describes a forthcoming book based on research or scholarly findings.', false, 200, 'CASRAI', '2019-02-08'),
        (104, 'Book review', 23, '41', 'Critical review of works of fiction or non-fiction highlighting the contributions to an art, field or discipline.', false, 205, 'CASRAI', '2019-02-08'),
        (105, 'Book series', 23, '41', 'Set of related books written by a single author or collaboratively based on research or scholarly findings.', false, 210, 'CASRAI', '2019-02-08'),
        (106, 'Conference abstract', 23, '41', 'Texts of a specified length that states the issue to be discussed in a proposed conference paper. It serves as the basis for the acceptance of the paper at a conference. The abstract is published along with the paper.', true, 215, 'CASRAI', '2019-02-08'),
        (107, 'Conference paper', 23, '41', 'Papers written alone or collaboratively, presented at an academic conference, and published in the proceedings (not in scholarly journals).', true, 220, 'CASRAI', '2019-02-08'),
        (108, 'Conference poster', 23, '41', 'Posters displayed in a conference setting and conveying research highlights in an efficient manner by compelling graphics. They may be peer-reviewed prior to acceptance and be published in the proceedings.', true, 225, 'CASRAI', '2019-02-08'),
        (109, 'Conference program', 23, '41', 'Document giving details of papers to be presented at an academic conference, compiled from the accepted submissions.', false, 230, 'CASRAI', '2019-02-08'),
        (112, 'Dissertation', 23, '41', 'Treatise advancing an original point of view resulting from research: a requirement for a doctoral degree.', false, 245, 'CASRAI', '2019-02-08'),
        (113, 'Edited book', 23, '41', 'Books edited by a single author or collaboratively for the dissemination of research or scholarly findings that generally result from peer reviewed funding.', false, 250, 'CASRAI', '2019-02-08'),
        (114, 'Encyclopaedia entry', 23, '41', 'Authored entries in a reference work or a compendium focusing on a particular domain or on all branches of knowledge.', false, 255, 'CASRAI', '2019-02-08'),
        (115, 'Funding submission', 23, '36', 'Information about specific requests for funds submitted to potential funders of the activity. The standard allows details to be collected for multiple years.', false, 260, 'CASRAI', '2019-02-08'),
        (117, 'Journal issue', 23, '21', 'Periodical publications aimed at fostering intellectual debate and inquiry. Special journal issues are produced by editors with an established record of scholarship in the field and able to provide the direction of the theme. Journal issues bear a unique number of reference for publication.', false, 270, 'CASRAI', '2019-02-08'),
        (119, 'Magazine article', 23, '41', 'Articles in thematic publications published at fixed intervals.', false, 280, 'CASRAI', '2019-02-08'),
        (120, 'Manual (for teaching)', 23, '41', 'Course and assignment materials produced for teaching purposes.', false, 285, 'CASRAI', '2019-02-08'),
        (121, 'Newsletter article', 23, '41', 'Articles in publications aimed at researchers, decision-makers, professionals and the public that report on a research project or on the activities of a research chair or a research centre. The Newsletters promote research activities to the community and the university; mobilize knowledge to improve practice and inform policy, and provide relevant and accessible information to the broader public.', false, 290, 'CASRAI', '2019-02-08'),
        (122, 'Newspaper article', 23, '41', 'Articles in a daily, weekly or monthly publication reporting on news and social issues aimed at the public. May entail critical analysis based on expertise in the field.', false, 295, 'CASRAI', '2019-02-08'),
        (123, 'Online resource', 23, '41', 'Information accessible only on the web via traditional technical methods (i.e. hyperlinks).', false, 300, 'CASRAI', '2019-02-08'),
        (126, 'Report', 23, '41', 'Reports disseminating the outcomes and deliverables of a research contract. May entail a contribution to public policy.', false, 315, 'CASRAI', '2019-02-08'),
        (127, 'Research tool', 23, '41', 'Series of observations, measurements or facts identified from the research. They include bibliographies, indices and catalogues of research collections; concordances and dictionaries; materials that facilitate access to archival holdings or collections such as repository guides, inventories of a group of manuscripts or of a body of archives, inventories or documentary materials, thematic guides to archival materials, records surveys and special indices; scholarly editions; and data series.', false, 320, 'CASRAI', '2019-02-08'),
        (128, 'Supervised student publication', 23, '41', 'Articles on research findings published jointly with or supervised by the thesis advisor. The findings relate to research undertaken by the student or the supervisor’s program of research.', false, 325, 'CASRAI', '2019-02-08'),
        (134, 'Website', 23, '51', 'Stand-alone locations on the web where multiple types of information on a specific theme are available. May include interactive features for contributions from readers.', false, 355, 'CASRAI', '2019-02-08'),
        (135, 'Working paper / pre-print', 23, '21', 'Preliminary versions of articles that have not undergone review but that may be shared for comment.', true, 360, 'CASRAI', '2019-02-08'),
        (152, 'Grouped journal articles', 12, '21', 'A collection of journal articles on the same topic or study (should also be recorded separately).', false, 370, 'ECRIN', '2019-02-08'),
        (153, 'Grouped analysis datasets', 12, '34', 'A collection of analysis datasets on the same topic or study (should also be recorded separately).', false, 375, 'ECRIN', '2019-02-08'),
        (154, 'Grouped aggregated result datasets', 12, '34', 'A collection of aggregated result datasets on the same topic or study (should also be recorded separately).', false, 380, 'ECRIN', '2019-02-08'),
        (166, 'Script(s) used in analysis', 21, '52', 'Statistical software scripts used in study analysis.', true, 405, 'ECRIN', '2019-08-18'),
        (167, 'CDMS', 21, '52', 'Clinical Database Management System, or component within such a system, as used for clinical data management.', false, 410, 'ECRIN', '2019-08-18'),
        (168, 'Trial management system', 21, '52', 'System used to support study administration.', false, 415, 'ECRIN', '2019-08-18'),
        (169, 'eTMF', 21, '52', 'Electronic trial management system.', false, 420, 'ECRIN', '2019-08-18'),
        (170, 'Data extraction system', 21, '52', 'Used for extracting data from remote resources using XML or web scraping.', false, 425, 'ECRIN', '2019-08-18'),
        (151, 'Other AV object', 11, '99', 'Any audiovisual resource not defined elsewhere.', true, 1365, 'ECRIN', '2019-02-08'),
        (155, 'Other collection object', 12, '99', 'Any collection class resource not defined elsewhere.', false, 1385, 'ECRIN', '2019-02-08'),
        (156, 'Other data paper object', 13, '99', 'Any data paper class resource not defined elsewhere.', false, 1390, 'ECRIN', '2019-02-08'),
        (171, 'Other dataset object', 14, '36', 'Any dataset class resource not defined elsewhere.', true, 1392, 'ECRIN', '2019-02-08'),
        (157, 'Other event', 15, '99', 'Any event class resource not defined elsewhere.', false, 1395, 'ECRIN', '2019-02-08'),
        (158, 'Other image object', 16, '99', 'Any image class resource not defined elsewhere.', true, 1400, 'ECRIN', '2019-02-08'),
        (159, 'Other interactive object', 17, '99', 'Any interactive resource class not defined elsewhere.', true, 1405, 'ECRIN', '2019-02-08'),
        (161, 'Other physical object', 19, '99', 'Any physical object class resource not defined elsewhere.', false, 1415, 'ECRIN', '2019-02-08'),
        (162, 'Other service resource', 20, '99', 'Any serviceclass resource not defined elsewhere.', false, 1420, 'ECRIN', '2019-02-08'),
        (165, 'Other software object', 21, '99', 'Any software class resource not defined elsewhere.', true, 1422, 'ECRIN', '2019-02-08'),
        (163, 'Other sound object', 22, '99', 'Any sound class resource not defined elsewhere.', true, 1425, 'ECRIN', '2019-02-08'),
        (37, 'Other text based object', 23, '99', 'A text class resource not defined elsewhere.', true, 1427, 'BioLINCC', '2019-03-27'),
        (164, 'Other workflow object', 24, '99', 'Any workflow class resource not defined elsewhere.', false, 1430, 'ECRIN', '2019-02-08'),
        (160, 'Other object, in ‘other’ class', 18, '99', 'Any object not defined elsewhere and in the ‘other’ class', false, 1998, 'ECRIN', '2019-02-08'),
        (0, 'Not yet known', 0, '99', 'Dummy value supplied by default on entity creation.', false, 1999, 'ECRIN', '2019-02-08');"#
}


pub fn org_attribute_types<'a>() -> &'a str {

    r#"insert into lkup.org_attribute_types (id, name, data_type, 
          description, list_order, source_org, date_added)
       values 
        (11, 'URL', 'url', 'URL of the landing page of the organisation – may be in various language versions', 10, 'ROR', '2021-07-18'),
        (12, 'Wikipedia entry', 'url', 'URL of main Wikipedia page, if one exists', 20, 'ROR', '2021-07-18'),
        (13, 'Wikidata entry', 'url', 'URL of a Wikidata link, if one exists', 30, 'ROR', '2021-07-18'),
        (1001, 'NLM Databank Id', 'string', 'Abbreviation used within PubMed to indicate a data repository', 40, 'ECRIN', '2020-06-01'),
        (1004, 'BBMRI identifier', 'string', 'PID applieed by BBMRI to biobank organisations', 50, 'BBMRI', '2023-09-01'),
        (1002, 'ISNI id', 'string', 'String identifier from the International Standard Name Identifier scheme', 150, 'ROR', '2020-06-01'),
        (1003, 'GRID id', 'string', 'String identifier from the Global Research Identifier Database', 160, 'ROR', '2020-06-01');"#
}


pub fn org_classes<'a>() -> &'a str {

    r#"insert into lkup.org_classes (id, name, description, list_order, source_org, date_added)
       values 
       (100, 'Government', 'An organization that is part of or operated by a national or regional government and that conducts or supports research.', 10, 'ROR', '2025-01-15'), 
       (200, 'Education', 'A university or similar institution involved in providing education and educating/employing researchers.', 20, 'ROR', '2025-01-15'), 
       (300, 'Healthcare', 'A medical care facility such as hospital or medical clinic. Excludes medical schools, which should be categorized as ‘Education”.', 30, 'ROR', '2025-01-15'), 
       (400, 'Company', 'A private for-profit corporate entity involved in conducting or sponsoring research.', 40, 'ROR', '2025-01-15'), 
       (500, 'Nonprofit', 'A non-profit and non-governmental organization involved in conducting or funding research.', 50, 'ROR', '2025-01-15'), 
       (600, 'Funder', 'An organization that awards research funds or provides in-kind support. Organisations linked to a Funder ID will be assigned this type, usually in conjunction with another type.', 60, 'ROR', '2025-01-15'),
       (700, 'Facility', 'A specialized facility where research takes place, such as a laboratory or telescope or dedicated research area.', 70, 'ROR', '2025-01-15'), 
       (800, 'Archive', 'An organization involved in stewarding research and cultural heritage materials. Includes libraries, museums, and zoos.', 80, 'ROR', '2025-01-15'),  
       (900, 'Other', 'A category for any organization that does not fit the other named categories.', 90, 'ROR', '2025-01-15');"#
}


pub fn org_name_qualifier_types<'a>() -> &'a str {

    r#"insert into lkup.org_name_qualifier_types (id, name, description, 
          list_order, source_org, date_added)
       values 
        (1, 'Default', 'The default (and usual display) name. Should normally be in the dominant local language.', 10, 'ECRIN', '2020-06-01'),
        (2, 'Also listed as', 'Alternative name, in any language, including English versions of the default name if different.', 20, 'ECRIN', '2020-06-01'),
        (3, 'Abbreviated name', 'Abbreviation or acronym that is distinct enough to be listed / matched as a valid name option', 30, 'ECRIN', '2023-08-02'),
        (10, 'Abbreviated as', 'Abbreviation or acronym', 50, 'ECRIN', '2020-06-01'),
        (18, 'Previously known as', 'Historic name, no longer in current use', 60, 'ECRIN', '2020-06-01'),
        (19, 'External Foreign name', 'Name in a foreign language NOT generally used in local area (e.g. as found in a foreign listing)', 70, 'ECRIN', '2020-06-01');
        (20, 'Minor variation', 'Minor variation or mis-spelt version of a name, or one with a location suffix or different punctuation, that exists in source data but is not in normal usage.', 80, 'ECRIN', '2020-06-01');"#
}



pub fn org_relationship_types<'a>() -> &'a str {

    r#"insert into lkup.org_relationship_types (id, name, description, 
          list_order, source_org, date_added)
       values 
        (1001, 'is a department within', '', 20, 'ECRIN', '2020-06-01'),
        (1012, 'contains', '', 25, 'ECRIN', '2020-06-01'),
        (1002, 'is part of', '', 30, 'ECRIN', '2020-06-01'),
        (1004, 'includes', '', 35, 'ECRIN', '2020-06-01'),
        (1003, 'is a member of ', '', 40, 'ECRIN', '2020-06-01'),
        (1011, 'includes collaboration of', '', 45, 'ECRIN', '2020-06-01'),
        (1005, 'was formed from the split of ', '', 60, 'ECRIN', '2020-06-01'),
        (1008, 'was acquired by', '', 70, 'ECRIN', '2020-06-01'),
        (1006, 'was formed from the merger of', '', 80, 'ECRIN', '2020-06-01'),
        (1009, 'was merged into', '', 90, 'ECRIN', '2020-06-01'),
        (1007, 'is funded by', '', 100, 'ECRIN', '2020-06-01'),
        (1010, 'is affiliated with / related to', '', 110, 'ECRIN', '2020-06-01'),
        (1013, 'other (in ROR terms)', '', 999, 'ROR', '2022-08-09');"#
}


pub fn org_types<'a>() -> &'a str {

    r#"insert into lkup.org_types (id, class_id, name, 
          description, list_order, source_org, date_added)
       values
        (1044, 11, 'National research institute / centre', '', 101, 'ECRIN', '2020-06-01'),
        (1026, 11, 'University research institute', '', 105, 'ECRIN', '2020-06-01'),
        (1048, 11, 'Hospital research institute', '', 110, 'ECRIN', '2020-06-01'),
        (1021, 11, 'Other government research institution', '', 112, 'ECRIN', '2020-06-01'),
        (1006, 11, 'Independent research organisation (not for profit)', '', 115, 'ECRIN', '2020-06-01'),
        (1001, 11, 'Clinical trials unit', '', 120, 'ECRIN', '2020-06-01'),
        (1004, 11, 'Genetics / molecular biology laboratory', '', 125, 'ECRIN', '2020-06-01'),
        (1043, 11, 'Specialist cancer research centre', '', 130, 'ECRIN', '2020-06-01'),
        (1003, 11, 'Biochemistry / cell biology laboratory', '', 130, 'ECRIN', '2020-06-01'),
        (1002, 11, 'Epidemiology research unit', '', 135, 'ECRIN', '2020-06-01'),
        (1014, 14, 'University', '', 160, 'ECRIN', '2020-06-01'),
        (1025, 14, 'University faculty', '', 165, 'ECRIN', '2020-06-01'),
        (1039, 14, 'Medical university', '', 170, 'ECRIN', '2020-06-01'),
        (1060, 14, 'University specialising in traditional medicine', '', 175, 'ECRIN', '2020-06-01'),
        (1053, 14, 'Postgraduate medical education department ', '', 180, 'ECRIN', '2020-06-01'),
        (1042, 12, 'University hospital', '', 201, 'ECRIN', '2020-06-01'),
        (1041, 12, 'Hospital group', '', 203, 'ECRIN', '2020-06-01'),
        (1011, 12, 'General hospital ', '', 205, 'ECRIN', '2020-06-01'),
        (1012, 12, 'Children’s hospital or children and women’s hospital', '', 210, 'ECRIN', '2020-06-01'),
        (1054, 12, 'Hospital specialising in traditional medicine', '', 215, 'ECRIN', '2020-06-01'),
        (1013, 12, 'Specialist cancer hospital or centre', '', 220, 'ECRIN', '2020-06-01'),
        (1062, 12, 'Community based health organisation', '', 225, 'ECRIN', '2023-09-01'),
        (1064, 14, 'Other HE Institution', '', 225, 'ECRIN', '2023-09-01'),
        (1015, 13, 'Pharmaceutical company', '', 301, 'ECRIN', '2020-06-01'),
        (1016, 13, 'Biotech company', '', 303, 'ECRIN', '2020-06-01'),
        (1017, 13, 'Medical device company', '', 305, 'ECRIN', '2020-06-01'),
        (1063, 13, 'Supplier of biological materials, reagents', '', 310, 'ECRIN', '2023-09-01'),
        (1050, 13, 'Journal publishers', '', 315, 'ECRIN', '2020-06-01'),
        (1052, 13, 'Multi-sector company', '', 318, 'ECRIN', '2020-06-01'),
        (1018, 13, 'Commercial research organisation', '', 320, 'ECRIN', '2020-06-01'),
        (1019, 13, 'Consultancy (pharma industry)', '', 330, 'ECRIN', '2020-06-01'),
        (1020, 13, 'Consultancy (IT)', '', 333, 'ECRIN', '2020-06-01'),
        (1022, 13, 'Software vendor', '', 336, 'ECRIN', '2020-06-01'),
        (1023, 13, 'IT hosting service', '', 341, 'ECRIN', '2020-06-01'),
        (1036, 15, 'United Nations agency', '', 401, 'ECRIN', '2020-06-01'),
        (1045, 15, 'Supra-national government agency', '', 405, 'ECRIN', '2020-06-01'),
        (1055, 15, 'Health funding programme', '', 410, 'ECRIN', '2020-06-01'),
        (1056, 15, 'Regional healthcare co-ordination / funding agency', '', 415, 'ECRIN', '2020-06-01'),
        (1037, 15, 'Government funding agency', '', 420, 'ECRIN', '2020-06-01'),
        (1029, 15, 'Government health related department', '', 425, 'ECRIN', '2020-06-01'),
        (1038, 15, 'Other government department', '', 430, 'ECRIN', '2020-06-01'),
        (1027, 16, 'National regulatory authority', '', 460, 'ECRIN', '2020-06-01'),
        (1028, 16, 'National regulator for data protection', '', 465, 'ECRIN', '2020-06-01'),
        (1051, 16, 'Review Board or Ethics Committee', '', 470, 'ECRIN', '2020-06-01'),
        (1008, 16, 'International research infrastructure', '', 475, 'ECRIN', '2020-06-01'),
        (1007, 16, 'National research infrastructure', '', 480, 'ECRIN', '2020-06-01'),
        (1065, 16, 'Research collaborative group', '', 485, 'ECRIN', '2023-09-01'),
        (1030, 16, 'Non governmental research funder', '', 495, 'ECRIN', '2020-06-01'),
        (1033, 17, 'Trial registry (WHO Source)', '', 501, 'ECRIN', '2020-06-01'),
        (1069, 17, 'Studies registry (MDR source, not a WHO source)', '', 503, 'ECRIN', '2023-09-04'),
        (1061, 17, 'Studies registry (neither an MDR or WHO source)', '', 505, 'ECRIN', '2022-08-30'),
        (1035, 17, 'Data repository', '', 510, 'ECRIN', '2020-06-01'),
        (1032, 17, 'Bibliographic database', '', 515, 'ECRIN', '2020-06-01'),
        (1034, 17, 'Metadata repository', '', 520, 'ECRIN', '2020-06-01'),
        (1066, 17, 'Biobank or samples collection', '', 525, 'ECRIN', '2023-09-01'),
        (1031, 18, 'Charity', '', 601, 'ECRIN', '2020-06-01'),
        (1040, 18, 'General professional association', '', 605, 'ECRIN', '2020-06-01'),
        (1067, 18, 'Condition specific professional Association', '', 610, 'ECRIN', '2023-09-01'),
        (1068, 18, 'Patient / advocacy group', '', 615, 'ECRIN', '2023-09-01');"#
}

    
pub fn resource_types<'a>() -> &'a str {

    r#"insert into lkup.resource_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values
        (11, 'PDF', '', true, 10, 'ECRIN', '2019-02-08'),
        (35, 'Web text', 'Web page or text, no download or API options.', false, 12, 'PubMed', '2019-08-24'),
        (36, 'Web text with download', 'Web text, with download option (e.g. PubMed Central entry).', false, 14, 'PubMed', '2019-08-24'),
        (37, 'Web text with XML via API ', 'Web page or text, content as XML via API.', false, 16, 'ECRIN', '2019-11-01'),
        (38, 'Web text with JSON via API ', 'Web page or text, content as JSON via API.', false, 18, 'ECRIN', '2019-11-01'),
        (39, 'Web text with XML or JSON via API ', 'Web page or text, content as XML or JSON via API (e.g. from Clinical trials.gov, PubMed).', false, 20, 'ECRIN', '2019-11-01'),
        (40, 'Web text journal abstract', 'Web page or text that is an abstract of a journal article', false, 21, 'ECRIN', '2020-04-14'),
        (12, 'Comma separated values', 'Usually .csv files.', true, 30, 'ECRIN', '2019-02-08'),
        (13, 'Tab separated values', 'May be .tsv or .txt files.', true, 32, 'ECRIN', '2019-02-08'),
        (14, 'Plain text file', 'e.g. from text editor, or data with delimiter unknown.', true, 34, 'ECRIN', '2019-02-08'),
        (15, 'Rich text file', '.rtf files.', true, 36, 'ECRIN', '2019-02-08'),
        (16, 'Word doc', '.doc or .docx files.', true, 38, 'ECRIN', '2019-02-08'),
        (17, 'Other WP document', 'e.g. .odt (libra office), .gdoc (google docs) files.', true, 40, 'ECRIN', '2019-02-08'),
        (18, 'Excel spreadsheet(s)', '.xsl or .xslx. files.', true, 45, 'ECRIN', '2019-02-08'),
        (19, 'Other spreadsheet(s)', 'e.g. .ods (libra office), .gsheet (google docs) files.', true, 50, 'ECRIN', '2019-02-08'),
        (20, 'PowerPoint', '.ppt or .pptx files.', true, 55, 'ECRIN', '2019-02-08'),
        (21, 'Other presentation', 'e.g. .odp (libra office), .gslides (google docs) files).', true, 60, 'ECRIN', '2019-02-08'),
        (22, 'ODM XML document', 'Data or metadata following CDISC ODM schema.', true, 65, 'ECRIN', '2019-02-08'),
        (23, 'SDTM XML document', 'Data using the CDISC SDTM schema.', true, 70, 'ECRIN', '2019-02-08'),
        (24, 'Define.XML document', 'Metadata following the CDISC Define.xml schema.', true, 75, 'ECRIN', '2019-02-08'),
        (25, 'ADaM XML document', 'Analysis data following thbe CDISC ADaM schema.', true, 80, 'ECRIN', '2019-02-08'),
        (26, 'XML document', 'XML schema not listed elsewhere.', true, 82, 'ECRIN', '2019-02-08'),
        (27, 'SAS transport file', '.xpt files.', true, 84, 'ECRIN', '2019-02-08'),
        (80, 'Virtual dataset', 'Dataset files (exact format unknown) available by application', false, 90, 'ECRIN', '2023-09-09'),
        (81, 'Virtual document', 'Document file (exact format unknown) available by application', false, 91, 'ECRIN', '2023-09-09'),
        (82, 'Virtual samples', 'Samples available by application', false, 92, 'ECRIN', '2023-09-09'),
        (28, 'R workspace file', '.rdata or .rda files.', true, 100, 'ECRIN', '2019-02-08'),
        (29, 'SSPS data file', '.sav file.', true, 105, 'ECRIN', '2019-02-08'),
        (30, 'Stata data file', '.dta file.', true, 110, 'ECRIN', '2019-02-08'),
        (31, 'Statistical program data file', 'Stats data file in format not listed elsewhere.', true, 115, 'ECRIN', '2019-02-08'),
        (32, 'Database file', 'A complete DB file that could be remounted directly into a DB system.', false, 120, 'ECRIN', '2019-02-08'),
        (33, 'Graphic image', 'e.g. .png, .jpeg, .svg files.', true, 125, 'ECRIN', '2019-02-08'),
        (34, 'Media file', 'e.g. mp3, .mp4, .mpeg files.', true, 130, 'ECRIN', '2019-02-08'),
        (90, 'Other', 'File format not listed elsewhere.', true, 900, 'ECRIN', '2019-02-08'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999, 'ECRIN', '2019-02-08');"#
}

pub fn size_units<'a>() -> &'a str {

    r#"insert into lkup.size_units (id, name, list_order, source_org, date_added)
       values 
        (11, 'Kb', 10, 'ECRIN', '2019-02-08'),
        (12, 'Mb', 20, 'ECRIN', '2019-02-08'),
        (13, 'Gb', 30, 'ECRIN', '2019-02-08'),
        (14, 'Pages', 40, 'ECRIN', '2019-02-08'),
        (15, 'Minutes', 50, 'ECRIN', '2019-02-08'),
        (16, 'Other', 60, 'ECRIN', '2019-02-08'),
        (0, 'Not yet known', 99, 'ECRIN', '2019-02-08');"#
}

pub fn study_feature_categories<'a>() -> &'a str {

    r#"insert into lkup.study_feature_categories (id, feature_type_id, name, 
          description, list_order, source_org, date_added)
       values 
        (105, 20, 'Early phase 1','Exploratory trials, involving very limited human exposure, with no therapeutic or diagnostic intent (e.g., screening studies, microdose studies).', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (110, 20, 'Phase 1','Initial studies to determine the metabolism and pharmacologic actions of drugs in humans, side effects, and to gain early evidence of effectiveness; may include healthy participants and/or patients.', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (115, 20, 'Phase 1/2','Trials that are a combination of phases 1 and 2.', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (120, 20, 'Phase 2','Controlled clinical studies conducted to evaluate the effectiveness of the intervention for a particular indication in participants with the disease or condition under study, and to determine the common short-term side effects and risks.', 40, 'ClinicalTrials.gov', '2019-11-04'),
        (125, 20, 'Phase 2/3','Trials that are a combination of phases 2 and 3.', 50, 'ClinicalTrials.gov', '2019-11-04'),
        (130, 20, 'Phase 3','Trials conducted after preliminary evidence suggesting effectiveness of the drug has been obtained, and are intended to gather additional information to evaluate the overall benefit-risk relationship of the drug.', 60, 'ClinicalTrials.gov', '2019-11-04'),
        (135, 20, 'Phase 4','Studies of approved drugs to delineate additional information including the drug’s risks, benefits, and optimal use.', 70, 'ClinicalTrials.gov', '2019-11-04'),
        (100, 20, 'Not applicable','Trials without phases (for example, studies of devices or behavioral interventions).', 80, 'ClinicalTrials.gov', '2019-11-04'),
        (140, 20, 'Not provided','No data was provided in the source record.', 90, 'ClinicalTrials.gov', '2019-11-04'),
        (400, 21, 'Treatment','One or more interventions are being evaluated for treating a disease, syndrome, or condition.', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (405, 21, 'Prevention','One or more interventions are being assessed for preventing the development of a specific disease or health condition.', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (410, 21, 'Diagnostic','One or more interventions are being evaluated for identifying a disease or health condition.', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (415, 21, 'Supportive care','One or more interventions are evaluated for maximizing comfort, minimizing side effects, or mitigating against a decline in the participant’s health or function.', 40, 'ClinicalTrials.gov', '2019-11-04'),
        (420, 21, 'Screening','One or more interventions are assessed or examined for identifying a condition, or risk factors for a condition, in people who are not yet known to have the condition or risk factor.', 50, 'ClinicalTrials.gov', '2019-11-04'),
        (425, 21, 'Health services research','One or more interventions for evaluating the delivery, processes, management, organization, or financing of healthcare.', 60, 'ClinicalTrials.gov', '2019-11-04'),
        (430, 21, 'Basic science','One or more interventions for examining the basic mechanism of action (for example, physiology or biomechanics of an intervention).', 70, 'ClinicalTrials.gov', '2019-11-04'),
        (435, 21, 'Device feasibility','An intervention of a device product is being evaluated in a small clinical trial (generally fewer than 10 participants) to determine the feasibility of the product; or a clinical trial to test a prototype device for feasibility and not health outcomes. Such studies are conducted to confirm the design and operating specifications of a device before beginning a full clinical trial.', 80, 'ClinicalTrials.gov', '2019-11-04'),
        (450, 21, 'Educational / counselling / training','An intervention involving psychosocial or educational input', 90, 'ClinicalTrials.gov', '2020-03-27'),
        (440, 21, 'Other','None of the other options applies.', 100, 'ClinicalTrials.gov', '2019-11-04'),
        (445, 21, 'Not provided','No data was provided in the source record.', 900, 'ClinicalTrials.gov', '2019-11-04'),
        (205, 22, 'Randomised','Participants are assigned to intervention groups by chance', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (210, 22, 'Nonrandomised','Participants are expressly assigned to intervention groups through a non-random method, such as physician choice', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (200, 22, 'Not applicable','For a single-arm trial', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (215, 22, 'Not provided','No data was provided in the source record.', 40, 'ClinicalTrials.gov', '2019-11-04'),
        (300, 23, 'Single group assignment','Clinical trials with a single arm', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (305, 23, 'Parallel assignment','Participants are assigned to one of two or more groups in parallel for the duration of the study', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (310, 23, 'Crossover assignment','Participants receive one of two (or more) alternative interventions during the initial phase of the study and receive the other intervention during the second phase of the study', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (315, 23, 'Factorial assignment','Two or more interventions, each alone and in combination, are evaluated in parallel against a control group', 40, 'ClinicalTrials.gov', '2019-11-04'),
        (320, 23, 'Sequential assignment','Groups of participants are assigned to receive interventions based on prior milestones being reached in the study, such as in some dose escalation and adaptive design studies', 50, 'ClinicalTrials.gov', '2019-11-04'),
        (325, 23, 'Not provided','No data was provided in the source record.', 60, 'ClinicalTrials.gov', '2019-11-04'),
        (500, 24, 'None (Open Label)','', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (502, 24, 'Blinded (no details)','From a statement that says the study was blinded, but where the degree of blinding was not provided', 15, 'WHO', '2020-06-10'),
        (505, 24, 'Single','', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (510, 24, 'Double','', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (515, 24, 'Triple','', 40, 'ClinicalTrials.gov', '2019-11-04'),
        (520, 24, 'Quadruple','', 50, 'ClinicalTrials.gov', '2019-11-04'),
        (599, 24, 'Not applicable','Explicitly labelled as not applicable - usually because the study is non-interventional	', 60, 'ECRIN', '2020-06-10'),
        (525, 24, 'Not provided','No data was provided in the source record.', 90, 'ClinicalTrials.gov', '2019-11-04'),
        (600, 30, 'Cohort','Group of individuals, initially defined and composed, with common characteristics (for example, condition, birth year), who are examined or traced over a given time period.', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (605, 30, 'Case-control','Group of individuals with specific characteristics (for example, conditions or exposures) compared to group(s) with different characteristics, but otherwise similar.', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (610, 30, 'Case-only','Single group of individuals with specific characteristics.', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (615, 30, 'Case-crossover','Characteristics of case immediately prior to disease onset (sometimes called the hazard period) compared to characteristics of same case at a prior time (that is, control period).', 40, 'ClinicalTrials.gov', '2019-11-04'),
        (620, 30, 'Ecologic or community study','Geographically defined populations, such as countries or regions within a country, compared on a variety of environmental (for example, air pollution intensity, hours of sunlight) and/or global measures not reducible to individual level characteristics (for example, healthcare system, laws or policies median income, average fat intake, disease rate).', 50, 'ClinicalTrials.gov', '2019-11-04'),
        (625, 30, 'Family-based','Studies conducted among family members, such as genetic studies within families or twin studies and studies of family environment.', 60, 'ClinicalTrials.gov', '2019-11-04'),
        (640, 30, 'Defined population','', 70, 'ClinicalTrials.gov', '2020-03-27'),
        (645, 30, 'Natural history',' ', 80, 'ClinicalTrials.gov', '2020-03-27'),
        (630, 30, 'Other','None of the othjer categories provided.', 100, 'ClinicalTrials.gov', '2019-11-04'),
        (635, 30, 'Not provided','No data was provided in the source record.', 900, 'ClinicalTrials.gov', '2019-11-04'),
        (700, 31, 'Retrospective','Look back using observations collected predominantly prior to subject selection and enrollment', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (705, 31, 'Prospective','Look forward using periodic observations collected predominantly following subject enrollment', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (710, 31, 'Cross-sectional','Observations or measurements made at a single point in time, usually at subject enrollment', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (725, 31, 'Retrospective / prospective','', 40, 'ClinicalTrials.gov', '2020-03-27'),
        (730, 31, 'Longitudinal','', 50, 'ClinicalTrials.gov', '2020-03-27'),
        (715, 31, 'Other','Explain in Detailed Description', 100, 'ClinicalTrials.gov', '2019-11-04'),
        (720, 31, 'Not provided','No data was provided in the source record.', 900, 'ClinicalTrials.gov', '2019-11-04'),
        (800, 32, 'None retained','', 10, 'ClinicalTrials.gov', '2019-11-04'),
        (805, 32, 'Samples with DNA ','', 20, 'ClinicalTrials.gov', '2019-11-04'),
        (810, 32, 'Samples without DNA','', 30, 'ClinicalTrials.gov', '2019-11-04'),
        (815, 32, 'Not provided','No data was provided in the source record.', 40, 'ClinicalTrials.gov', '2019-11-04');"#
}

pub fn study_feature_types<'a>() -> &'a str {

    r#"insert into lkup.study_feature_types (id, context, name, 
          description, list_order, source_org, date_added)
       values 
        (20, 'interventional', 'Phase','For a clinical trial of a drug product (including a biological product), the numerical phase of the clinical trial.', 10, 'CliniclTrials.gov', '2019-11-05'),
        (21, 'interventional', 'Primary purpose','In very broad terms, the clinical or scientific area the trial is intended to contribute towards.', 20, 'CliniclTrials.gov', '2019-11-05'),
        (22, 'interventional', 'Allocation type','The method by which participants are assigned to arms in a clinical trial.', 30, 'CliniclTrials.gov', '2019-11-05'),
        (23, 'interventional', 'Intervention model','The strategy for assigning interventions to participants.', 40, 'CliniclTrials.gov', '2019-11-05'),
        (24, 'interventional', 'Masking','The party or parties involved in the clinical trial who are prevented from having knowledge of the interventions assigned to individual participants.', 50, 'CliniclTrials.gov', '2019-11-05'),
        (30, 'observational', 'Observational model','The Primary strategy for participant identification and follow-up.', 60, 'CliniclTrials.gov', '2019-11-05'),
        (31, 'observational', 'Time perspective','For observational studies, describes the temporal relationship of observation period to time of participant enrollment.', 70, 'CliniclTrials.gov', '2019-11-05'),
        (32, 'observational', 'Biospecimens retained','Indicates whether samples of material from research participants are retained in a biorepository.', 80, 'CliniclTrials.gov', '2019-11-05');"#
}

pub fn study_relationship_types<'a>() -> &'a str {

    r#"insert into lkup.study_relationship_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (11, 'Is a sub-study of', 'This study is a sub-study or sub-protocol undertaken at the same time as <the target study>.', true, 10, 'ECRIN', '2019-01-14'),
        (12, 'Includes as a sub-study', 'Has <the target study> as a sub-study or sub-protocol, undertaken at the same time as this study.', true, 15, 'ECRIN', '2019-01-14'),
        (13, 'Is in the same series as', 'This study is in a sequence that began with <the target study>.', true, 20, 'ECRIN', '2019-01-14'),
        (14, 'Is the first of a sequence including', 'This study began a sequence that includes <the target study>.', true, 25, 'ECRIN', '2019-01-14'),
        (15, 'Is a feasibility study for', 'This study is a feasibility or pilot study for <the target study>.', true, 30, 'ECRIN', '2019-01-14'),
        (16, 'Is preceded by the feasibility study', 'This study was preceded by <the target study> as a feasibility or pilot study.', true, 35, 'ECRIN', '2019-01-14'),
        (17, 'Is a later phase variant of', 'For trials using the phase I to IV classification, this study is a later phase continuation of <the target study>.', true, 40, 'ECRIN', '2019-01-14'),
        (18, 'Is an earlier phase variant of', 'For trials using the phase I to IV classification, this study is an earlier phase precursor to <the target study>.', true, 45, 'ECRIN', '2019-01-14'),
        (19, 'Is a continuation of', 'This study uses some or all of the same subject population as <the target study>.', true, 50, 'ECRIN', '2019-01-14'),
        (20, 'Is continued by', 'This study has some or all of the same subject population targeted by <the target study>.', true, 55, 'ECRIN', '2019-01-14'),
        (21, 'Is a repeat of', 'This study uses a different population but the same or similar protocol as <the target study>.', true, 60, 'ECRIN', '2019-01-14'),
        (22, 'Is repeated by', 'This study is repeated by <the target study>, with the same or similar protocol but using a different population.', true, 65, 'ECRIN', '2019-01-14'),
        (23, 'has an expanded access version', 'This study has <the target study> as an expanded access version, (for people who cannot enrol in the trial but who may benefit from the product under investigation).', true, 70, 'ClinicalTrials.gov', '2019-11-04'),
        (24, 'is an expanded access version of', 'This study is an expanded access version of <the target study>, catering for people who cannot enrol in the trial but who may benefit from the product under investigation.', true, 75, 'ClinicalTrials.gov', '2019-11-04'),
        (25, 'Includes target as one of a group of non-registered studies', 'This study includes <the target study>. That study is not registered independently, but instead shares this registry entry with one or more other non-registered studies.', false, 80, 'BioLINCC', '2019-12-15'),
        (26, 'Non registered but included within a registered study group', 'This study is registered as <the target study>, along with one or more other studies that share the same registry entry and id.', false, 85, 'BioLINCC', '2019-12-15'),
        (27, 'Has link listed in registry but nature of link unclear', 'This study is linked to <the target study> within the registry entry, but the nature of the linkage is not clear.', false, 90, 'WHO', '2020-06-10'),
        (28, 'Includes target as one of a group of registered studies', 'This study includes <the target study>, which is registered elsewhere along with one or more other registered studies, forming a group that collectively equates to this study.', false, 95, 'WHO', '2020-06-10'),
        (29, 'Registered and is included elsewhere in group', 'This study is also registered, along with one or more other studies that together form an equivalent group, as <the target study>.', false, 100, 'WHO', '2020-06-10'),
        (30, 'One of a related group of 3 or more studies', 'One of a derived group of related studies - exact relationship between them is unclear', false, 105, 'ECRIN', '2020-06-10'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999, 'ECRIN', '2019-01-14');"#
}

pub fn study_statuses<'a>() -> &'a str {

    r#"insert into lkup.study_statuses (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values
        (10, 'Not yet recruiting', 'Participants are not yet being recruited.', true, 10, 'ClinicalTrials.gov', '2019-02-08'),
        (12, 'Withdrawn', 'Study halted prior to enrolment of first participant.', false, 70, 'ClinicalTrials.gov', '2019-02-08'),
        (14, 'Recruiting', 'Participants are currently being recruited, whether or not any participants have yet been enrolled.', true, 20, 'ClinicalTrials.gov', '2019-02-08'),
        (16, 'Enrolling by invitation', 'Participants are being (or will be) selected from a predetermined population.', false, 50, 'ClinicalTrials.gov', '2019-02-08'),
        (18, 'Active, not recruiting', 'Study is continuing, meaning participants are receiving an intervention or being examined, but new participants are not currently being recruited or enrolled.', true, 30, 'ClinicalTrials.gov', '2019-02-08'),
        (20, 'Ongoing', 'Trial is ongoing but recruitment status is unclear', false, 60, 'EU CTR', '2020-04-21'),
        (25, 'Suspended', 'Study halted prematurely but potentially will resume.', true, 80, 'ClinicalTrials.gov', '2019-02-08'),
        (30, 'Completed', 'The study has concluded normally; participants are no longer receiving an intervention or being examined (that is, last participant’s last visit has occurred).', true, 40, 'ClinicalTrials.gov', '2019-02-08'),
        (32, 'Terminated', 'Study halted prematurely and will not resume; participants are no longer being examined or receiving intervention.', true, 90, 'ClinicalTrials.gov', '2019-02-08'),
        (98, 'Not applicable', 'Explicitly listed as Not Applicable', true, 110, 'WHO', '2020-04-20'),
        (99, 'Other', 'Status given as "other" or does not fit within other available options', true, 100, 'WHO', '2020-04-20'),
        (0, 'Not provided', 'Status information not provided.', false, 999, 'ClinicalTrials.gov', '2019-02-08');"#
}


pub fn study_types<'a>() -> &'a str {

    r#"insert into lkup.study_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
       (11, 'Interventional', 'Participants are assigned to receive an intervention so that researchers can study the effects.', true, 10, 'ClinicalTrials.gov', '2025-03-15'),
       (12, 'Observational', 'Participants do not receive an assigned intervention. They are observed over time for health-related outcomes.', true, 20, 'ClinicalTrials.gov', '2025-03-15'),
       (13, 'Patient registry', 'A type of observational study that collects information about patients’ medical conditions and/or treatments to better understand how a condition or treatment affects patients in the real world.', true, 30, 'ClinicalTrials.gov', '2025-03-15'),
       (14, 'Expanded access', 'A way for patients with serious conditions to receive a drug or treatment outside of a clinical trial.', true, 40, 'ClinicalTrials.gov', '2025-03-15'),
       (15, 'Funded programme', 'Two or more studies funded and registered as a unit', true, 50, '', '2025-03-15'),
       (16, 'Diagnostic test', 'Studies which are explictly labelled as being of a diagnostic test rather than a particular condition or intervention', true, 60, 'Chinese Clinical Trial Registry', '2025-03-15'),
       (98, 'Not applicable', 'Studies where the type is explicitl;y given as ‘not applicable’, ‘N/A’ or equivalent', true, 70, 'Chinese Clinical Trial Registry', '2025-03-15'),
       (99, 'Other', 'Studies not falling within the other categories or ambiguous in terms of interventional / observational status', true, 80, 'ECRIN', '2025-03-15'),
       (0, 'Not provided', 'Studies where the provided value is null or one of ‘unknown’, ‘Not provided’, ‘Not specified’ or the equivalent.', true, 90, 'ECRIN', '2025-03-15');"#
}

pub fn time_units<'a>() -> &'a str {

    r#"insert into lkup.time_units(id, name, use_in_data_entry, list_order, source_org, date_added)
       values
        (11, 'Seconds', false, 10, 'ECRIN', '2020-03-27'),
        (12, 'Minutes', false, 20, 'ECRIN', '2020-03-27'),
        (13, 'Hours', false, 30, 'ECRIN', '2020-03-27'),
        (14, 'Days', true, 40, 'ECRIN', '2020-03-27'),
        (15, 'Weeks', true, 50, 'ECRIN', '2020-03-27'),
        (16, 'Months', true, 55, 'ECRIN', '2020-03-27'),
        (17, 'Years', true, 60, 'ECRIN', '2020-03-27'),
        (0, 'Not provided', false, 99, 'ECRIN', '2020-03-27');"#
}

pub fn title_types<'a>() -> &'a str {

    r#"insert into lkup.title_types (id, name, applies_to, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (15, 'Public title', 'Study', 'In contrast to the full scientific title, usually from a trial registry - the default display name.', true, 10, 'ECRIN', '2019-01-14'),
        (16, 'Registry scientific title', 'Study', 'Full scientific title, as quoted in a trial registry.', true, 20, 'ECRIN', '2019-09-15'),
        (17, 'Protocol title', 'Study', 'Full scientific title, from a published protocol or similar source (e.g. the researchers themselves).', true, 30, 'ECRIN', '2019-09-15'),
        (18, 'Other scientific title', 'Study', 'Full scientific title, not from the protocol or a registry entry. May be, for example, from a data repository catalogue.', true, 40, 'ECRIN', '2019-09-15'),
        (14, 'Acronym or abbreviation', 'Study', 'As provided by study sponsors.', true, 50, 'ECRIN', '2019-01-14'),
        (12, 'Subtitle', 'All', 'A subtitle provided by object creators or study sponsors.', true, 60, 'DataCite', '2019-01-14'),
        (13, 'Translated title', 'All', 'Used in conjunction with language code to indicate language translated into.', true, 70, 'DataCite', '2019-01-14'),
        (19, 'Journal article title', 'Data Object', 'Full journal title, as listed in citation.', true, 80, 'ECRIN', '2019-09-15'),
        (20, 'Unique title, from sponsor name and doc ID', 'Data Object', 'Combination of sponsor abbreviation or name and a supplied document Id or label', true, 90, 'ECRIN', '2019-09-15'),
        (21, 'Object title as provided in source', 'Data Object', 'A name provided for the data object, e.g  in a list - will often refer to its type and not be unique, but should be unique within the parent study', true, 100, 'ECRIN', '2019-09-15'),
        (22, 'Object file name (without extension)', 'Data Object', 'The name provided for a named file, without the file extension.', true, 110, 'ECRIN', '2019-09-15'),
        (23, 'Object name provided as type', 'Data Object', 'In some cases an object ‘type’ can be provided that appears more like a title than a generic type', false, 120, 'ECRIN', '2019-09-15'),
        (24, 'Study scientific name :: object type', 'Data Object', 'Constructed using study full name to prefix object’s type. If obvious from context study name can be omitted.', false, 130, 'ECRIN', '2019-09-15'),
        (25, 'Study registry ID :: object name', 'Data Object', 'Constructed using registry id to prefix a non unique name.  If obvious from context study ID can be omitted.', false, 140, 'ECRIN', '2019-11-01'),
        (26, 'Study registry ID :: object type', 'Data Object', 'Constructed using registry id to prefix object’s type. If obvious from context study ID can be omitted.', false, 150, 'ECRIN', '2019-11-01'),
        (27, 'Study registry ID :: object type :: Id', 'Data Object', 'Constructed using registry id to prefix object’s type and an Id. Used if multiple instances of object type exist (e.g. with sample collections)', false, 160, 'ECRIN', '2023-09-02'),
        (28, 'Additional title / label (may not be unique)', 'Data Object', 'Any title or label applied, e.g. by the object’s creator or a storage infrastructure. Not necessarily unique.', true, 170, 'ECRIN', '2023-09-02'),
        (90, 'Other alternative title', 'All', 'Any alternative title not described elsewhere.', true, 900, 'DataCite', '2019-01-14'),
        (0, 'Not yet known', 'All', 'Dummy value supplied by default on entity creation.', false, 999, 'ECRIN', '2019-01-14');"#

}


pub fn topic_types<'a>() -> &'a str {

    r#"insert into lkup.topic_types (id, name, description, value_type, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
        (11, 'Keyword', 'Topic that was not categorised or does not fit into one of the categories listed. Often written by study or object creators.', 'Free-text or controlled vocabulary', false, 10, 'CliniclTrials.gov', '2019-02-08'),
        (12, 'Chemical / agent', 'One or more chemicals or biological agents, relevant to the study, including as interventions under test.', 'Free-text or controlled vocabulary', true, 15, 'CliniclTrials.gov', '2019-02-08'),
        (21, 'Device', 'Name of a medical  device', 'Free-text or controlled vocabulary', true, 17, 'ISRCTN', '2023-01-13'),
        (13, 'Condition', 'Illness or condition that is being targeted within study.', 'Free-text or controlled vocabulary', true, 20, 'CliniclTrials.gov', '2019-02-08'),
        (14, 'Design', 'Aspect of study design methodology.', 'Free-text or controlled vocabulary', true, 30, 'PubMed', '2019-02-08'),
        (15, 'Outcome', 'Outcome measure or outcome produced within the study.', 'Free-text or controlled vocabulary', true, 35, 'PubMed', '2019-02-08'),
        (16, 'Geographic', 'A geographical entity that was the particular focus or limit of the study.', 'Free-text or controlled vocabulary', true, 40, 'PubMed', '2019-08-24'),
        (17, 'Organism', 'Organism, e.g. particular bacterium, that was targeted during the study.', 'Free-text or controlled vocabulary', true, 45, 'PubMed', '2019-08-24'),
        (18, 'Treatment protocol', 'The name of a particular treatment regime / protocol, e.g. a chemtherapy regime', 'Free-text', true, 48, 'PubMed', '2019-11-24'),
        (19, 'Subject characteristics', 'Descriptive term pertaining to the subject group of the study', 'Free-text', true, 49, 'PubMed', '2019-11-24'),
        (20, 'Other', 'Cannot be easily categorised using the available topic types', 'Free-text or controlled vocabulary', true, 998, 'ECRIN', '2022-02-16'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', 'Not Applicable', false, 999, 'ECRIN', '2019-02-08');"#
}

pub fn topic_vocabularies<'a>() -> &'a str {

    r#"insert into lkup.topic_vocabularies (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values
        (11, 'Authors', 'Free text, not intentionally part of any controlled vocabulary.', false, 10, 'ECRIN', '2019-02-08'),
        (12, 'ICD 10', 'See https://icd.who.int/browse10/2016/en', true, 15, 'ECRIN', '2019-02-08'),
        (13, 'ICD 11', 'See https://icd.who.int/browse11/l-m/en', true, 20, 'ECRIN', '2019-02-08'),
        (14, 'MESH', 'See https://www.nlm.nih.gov/mesh/meshhome.html', true, 25, 'ECRIN', '2019-02-08'),
        (15, 'SnoMed CT', 'See http://www.snomed.org/', true, 30, 'ECRIN', '2019-02-08'),
        (16, 'MedDRA', 'See https://www.meddra.org/', true, 35, 'ECRIN', '2019-02-08'),
        (17, 'NCI thesaurus', 'See https://ncit.nci.nih.gov/ncitbrowser/', true, 40, 'ECRIN', '2019-02-08'),
        (18, 'Cochrane PICO terminology', 'See https://linkeddata.cochrane.org/pico-ontology', true, 45, 'ECRIN', '2019-02-08'),
        (19, 'CDISC controlled terminology', 'See https://www.cdisc.org/standards/terminology', true, 50, 'ECRIN', '2019-02-08'),
        (20, 'LOINC', 'See https://loinc.org/', true, 55, 'ECRIN', '2019-02-08'),
        (21, 'ATC drug classification', 'See https://www.whocc.no/atc/structure_and_principles/', true, 60, 'ECRIN', '2019-02-08'),
        (22, 'WHO Drug', 'See https://www.who-umc.org/whodrug/whodrug-portfolio/', true, 65, 'ECRIN', '2019-02-08'),
        (29, 'CAS', 'CAS chemical registry identifier, see https://www.cas.org/cas-data/cas-registry', true, 68, 'Euctr', '2023-04-01'),
        (23, 'IUPAC chemical names', 'Includes biochemical names. See https://iupac.org/what-we-do/nomenclature/', true, 70, 'ECRIN', '2019-02-08'),
        (24, 'InChI chemical identifier', 'See https://iupac.org/who-we-are/divisions/division-details/inchi/', true, 75, 'ECRIN', '2019-02-08'),
        (25, 'Enzyme Commission numbers / names', 'See https://www.qmul.ac.uk/sbcs/iubmb/', true, 80, 'ECRIN', '2019-02-08'),
        (26, 'HGNC human genome codes / names', 'Includes related proteins. See https://www.genenames.org/', true, 85, 'ECRIN', '2019-02-08'),
        (27, 'Taxonomic names', 'For example, Linnaean binominals. See http://www.iczn.org/iczn/index.jsp', true, 90, 'ECRIN', '2019-02-08'),
        (30, 'MESH Tree', 'The full tree code chains from the mesh system. See https://www.nlm.nih.gov/mesh/meshhome.html', true, 100, 'ECRIN', '2023-04-01'),
        (28, 'None', 'No  controlled terminology used', true, 998, 'ECRIN', '2022-02-17'),
        (0, 'Not yet known', 'Dummy value supplied by default on entity creation.', false, 999, 'ECRIN', '2019-02-08');"#
}


pub fn trial_registries<'a>() -> &'a str {

    r#"insert into lkup.trial_registries (id, name, description, 
          list_order, source_org, date_added)
       values 
        (100120, 'ClinicalTrials.gov', '', 10, 'ECRIN', '2022-08-30'),
        (100123, 'EU Clinical Trials Register', '', 20, 'ECRIN', '2022-08-30'),
        (100126, 'ISRCTN', '', 30, 'ECRIN', '2022-08-30'),
        (100118, 'Chinese Clinical Trial Register', '', 50, 'ECRIN', '2022-08-30'),
        (100127, 'Japan Primary Registries Network', '', 60, 'ECRIN', '2022-08-30'),
        (100116, 'Australian New Zealand Clinical Trials Registry', '', 70, 'ECRIN', '2022-08-30'),
        (100124, 'Deutschen Register Klinischer Studien', '', 80, 'ECRIN', '2022-08-30'),
        (100132, 'The Netherlands National Trial Register', '', 90, 'ECRIN', '2022-08-30'),
        (100121, 'Clinical Trials Registry - India', '', 100, 'ECRIN', '2022-08-30'),
        (100119, 'Clinical Research Information Service (South Korea)', '', 110, 'ECRIN', '2022-08-30'),
        (100125, 'Iranian Registry of Clinical Trials', '', 120, 'ECRIN', '2022-08-30'),
        (100117, 'Registro Brasileiro de Ensaios Clínicos', '', 130, 'ECRIN', '2022-08-30'),
        (100129, 'Registro Peruano de Ensayos Clínicos', '', 140, 'ECRIN', '2022-08-30'),
        (100122, 'Registro Público Cubano de Ensayos Clínicos', '', 150, 'ECRIN', '2022-08-30'),
        (100128, 'Pan African Clinical Trial Registry', '', 160, 'ECRIN', '2022-08-30'),
        (100130, 'Sri Lanka Clinical Trials Registry', '', 170, 'ECRIN', '2022-08-30'),
        (100131, 'Thai Clinical Trials Register', '', 180, 'ECRIN', '2022-08-30'),
        (101989, 'Lebenon Clinical Trial Registry', '', 190, 'ECRIN', '2022-08-30');"#
}

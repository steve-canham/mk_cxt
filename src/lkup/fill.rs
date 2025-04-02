

pub fn contribution_types<'a>() -> &'a str {

    r#"insert into lup.contribution_types (id, name, applies_to, description,
          use_in_data_entry, list_order, source_org, date_added)
       values ()

    );"#
}


pub fn dataset_consent_types<'a>() -> &'a str {

    r#"insert into lup.dataset_consent_types (id, name, description, 
          list_order, source_org, date_added)
        values ()
    );"#
}


pub fn dataset_deidentification_levels<'a>() -> &'a str {

    r#"insert into lup.dataset_deidentification_levels (id, name, description, 
          list_order, source_org, date_added)
       values ()
        
    );"#
}

pub fn dataset_recordkey_types<'a>() -> &'a str {

    r#"insert into lup.dataset_recordkey_types (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}

pub fn date_types<'a>() -> &'a str {

    r#"insert into lup.date_types (id, name, description, on_papers_only,
          use_in_data_entry, list_order, source_org, date_added)
       values ()

    );"#
}

pub fn description_types<'a>() -> &'a str {

    r#"insert into lup.description_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}

    
pub fn doi_status_types<'a>() -> &'a str {

    r#"insert into lup.doi_status_types (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}

pub fn gender_eligibility_types<'a>() -> &'a str {

    r#"insert into lup.gender_eligibility_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
       (1, 'Female', 'Study recruits only female participants', true, 10, 'ECRIN', '2025-03-01'),
       (2, 'Male', 'Study recruits only male participants', true, 20, 'ECRIN', '2025-03-01'),
       (3, 'Both', 'Study open to both male and female participants', true, 30, 'ECRIN', '2025-03-01');"#
}

pub fn identifier_types<'a>() -> &'a str {

    r#"insert into lup.identifier_types (id, name, applies_to, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()

    );"#
}

pub fn iec_level_types<'a>() -> &'a str {

    r#"insert into lup.iec_level_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}

pub fn language_usage_types<'a>() -> &'a str {

    r#"insert into lup.language_usage_types (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}

pub fn object_access_types<'a>() -> &'a str {

    r#"insert into lup.object_access_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}


pub fn object_classes<'a>() -> &'a str {

    r#"insert into lup.object_classes (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}


pub fn object_filter_types<'a>() -> &'a str {

    r#"insert into lup.object_filter_types (id, filter_as, description, 
          list_order, source_org, date_added)
       values ()

    );"#
}


pub fn object_relationship_types<'a>() -> &'a str {

    r#"insert into lup.object_relationship_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}


pub fn object_types<'a>() -> &'a str {

    r#"insert into lup.object_types (id, name, data_type, object_class_id,
          filter_as_id, description, list_order, source_org, date_added)
       values ()
       
    );"#
}


pub fn org_attribute_types<'a>() -> &'a str {

    r#"insert into lup.org_attribute_types (id, name, data_type, 
          description, list_order, source_org, date_added)
       values ()
       
    );"#
}


pub fn org_classes<'a>() -> &'a str {

    r#"insert into lup.org_classes (id, name, description, list_order, source_org, date_added)
       values 
       (100, 'Government', 'An organization that is part of or operated by a national or regional government and that conducts or supports research.', 10, 'ROR', '2025-01-15'), 
       (200, 'Education', 'A university or similar institution involved in providing education and educating/employing researchers.', 20, 'ROR', '2025-01-15'), 
       (300, 'Healthcare', 'A medical care facility such as hospital or medical clinic. Excludes medical schools, which should be categorized as “Education”.', 30, 'ROR', '2025-01-15'), 
       (400, 'Company', 'A private for-profit corporate entity involved in conducting or sponsoring research.', 40, 'ROR', '2025-01-15'), 
       (500, 'Nonprofit', 'A non-profit and non-governmental organization involved in conducting or funding research.', 50, 'ROR', '2025-01-15'), 
       (600, 'Funder', 'An organization that awards research funds or provides in-kind support. All records that are mapped to a Funder ID will be assigned this type, usually in conjunction with an additional organization type.', 60, 'ROR', '2025-01-15'),
       (700, 'Facility, 'A specialized facility where research takes place, such as a laboratory or telescope or dedicated research area.', 70, 'ROR', '2025-01-15''), 
       (800, 'Archive', 'An organization involved in stewarding research and cultural heritage materials. Includes libraries, museums, and zoos.', 80, 'ROR', '2025-01-15'),  
       (900, 'Other', 'A category for any organization that does not fit the other named categories.', 90, 'ROR', '2025-01-15');"#
}


pub fn org_name_qualifier_types<'a>() -> &'a str {

    r#"insert into lup.org_name_qualifier_types (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}


pub fn org_relationship_types<'a>() -> &'a str {

    r#"insert into lup.org_relationship_types (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}


pub fn org_types<'a>() -> &'a str {

    r#"insert into lup.org_types (id, class_id, name, 
          description, list_order, source_org, date_added)
       values ()
       
    );"#
}

    
pub fn resource_types<'a>() -> &'a str {

    r#"insert into lup.resource_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}

pub fn size_units<'a>() -> &'a str {

    r#"insert into lup.size_units (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}

pub fn study_feature_categories<'a>() -> &'a str {

    r#"insert into lup.study_feature_categories (id, feature_type_id, name, 
          description, list_order, source_org, date_added)
       values ()
        
    );"#
}

pub fn study_feature_types<'a>() -> &'a str {

    r#"insert into lup.study_feature_types (id, context, name, 
          description, list_order, source_org, date_added)
       values ()

    );"#
}

pub fn study_relationship_types<'a>() -> &'a str {

    r#"insert into lup.study_relationship_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}

pub fn study_statuses<'a>() -> &'a str {

    r#"insert into lup.study_statuses (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}

pub fn study_types<'a>() -> &'a str {

    r#"insert into lup.study_types (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values 
       (11, 'Interventional', 'Participants are assigned to receive an intervention so that researchers can study the effects.', true, 10, 'ClinicalTrials.gov', '2025-03-15'),
       (12, 'Observational', 'Participants do not receive an assigned intervention. They are observed over time for health-related outcomes.', true, 20, 'ClinicalTrials.gov', '2025-03-15'),
       (13, 'Patient registry', 'A type of observational study that collects information about patients medical conditions and/or treatments to better understand how a condition or treatment affects patients in the real world.', true, 30, 'ClinicalTrials.gov', '2025-03-15'),
       (14, 'Expanded access', 'A way for patients with serious conditions to receive a drug or treatment outside of a clinical trial.', true, 40, 'ClinicalTrials.gov', '2025-03-15'),
       (15, 'Funded programme', 'Two or more studies funded and registered as a unit', true, 50, '', '2025-03-15'),
       (16, 'Diagnostic test', 'Studies which are explictly labelled as being of a diagnostic test rather than a particular condition or intervention', true, 60, 'Chinese Clinical Trial Registry', '2025-03-15'),
       (98, 'Not applicable', 'Studies where the type is explicitl;y given as 'not applicable', 'N/A' or equivalent', true, 70, 'Chinese Clinical Trial Registry', '2025-03-15'),
       (99, 'Other', 'Studies not falling within the other categories or ambiguous in terms of interventional / observational status', true, 80, 'ECRIN', '2025-03-15'),
       (0, 'Not provided', 'Studies where the provided value is null or one of 'unknown', 'Not provided', 'Not Specified' or the equivalent.', true, 90, 'ECRIN', '2025-03-15'),
    );"#
}

pub fn time_units<'a>() -> &'a str {

    r#"insert into lup.time_units(id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}

pub fn title_types<'a>() -> &'a str {

    r#"insert into lup.title_types (id, name, applies_to, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()

    );"#
}

pub fn topic_types<'a>() -> &'a str {

    r#"insert into lup.topic_types (id, name, description, value_type, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()

    );"#
}

pub fn topic_vocabularies<'a>() -> &'a str {

    r#"insert into lup.topic_vocabularies (id, name, description, 
          use_in_data_entry, list_order, source_org, date_added)
       values ()
       
    );"#
}


pub fn trial_registries<'a>() -> &'a str {

    r#"insert into lup.trial_registries (id, name, description, 
          list_order, source_org, date_added)
       values ()
    );"#
}

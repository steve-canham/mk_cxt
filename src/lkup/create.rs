
pub fn create_schema_sql <'a>() -> &'a str {
    r#"SET client_min_messages TO WARNING; 
    create schema if not exists lkup;"#
}

pub fn contribution_types<'a>() -> &'a str {

    r#"drop table if exists lkup.contribution_types;
    CREATE TABLE lkup.contribution_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        applies_to         varchar    NULL,
        description        varchar    NULL,
        use_in_data_entry  bool       NULL,
        list_order         int4       DEFAULT 10 NULL,
        source_org         varchar    NULL,
        date_added         date       NULL
    );"#
}


pub fn dataset_consent_types<'a>() -> &'a str {

    r#"drop table if exists lkup.dataset_consent_types;
    CREATE TABLE lkup.dataset_consent_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn dataset_deidentification_levels<'a>() -> &'a str {

    r#"drop table if exists lkup.dataset_deidentification_levels;
    CREATE TABLE lkup.dataset_deidentification_levels (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn dataset_recordkey_types<'a>() -> &'a str {

    r#"drop table if exists lkup.dataset_recordkey_types;
    CREATE TABLE lkup.dataset_recordkey_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn date_types<'a>() -> &'a str {

    r#"drop table if exists lkup.date_types;
    CREATE TABLE lkup.date_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    on_papers_only     bool       NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn description_types<'a>() -> &'a str {

    r#"drop table if exists lkup.description_types;
    CREATE TABLE lkup.description_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

    
pub fn doi_status_types<'a>() -> &'a str {

    r#"drop table if exists lkup.doi_status_types;
    CREATE TABLE lkup.doi_status_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn gender_eligibility_types<'a>() -> &'a str {

    r#"drop table if exists lkup.gender_eligibility_types;
    CREATE TABLE lkup.gender_eligibility_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn identifier_types<'a>() -> &'a str {

    r#"drop table if exists lkup.identifier_types;
    CREATE TABLE lkup.identifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    applies_to         varchar    NULL,
	    description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn iec_level_types<'a>() -> &'a str {

    r#"drop table if exists lkup.iec_level_types;
    CREATE TABLE lkup.iec_level_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn language_codes<'a>() -> &'a str {

    r#"drop table if exists lkup.language_codes;
    CREATE TABLE lkup.language_codes (
        code               char(2)    NOT NULL PRIMARY KEY,
      	marc_code          varchar    NULL,
	    lang_name_en       varchar    NULL,
	    lang_name_fr       varchar    NULL,
	    lang_name_de       varchar    NULL,
	    is_major           bool       NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn language_scripts<'a>() -> &'a str {

    r#"drop table if exists lkup.language_scripts;
    CREATE TABLE lkup.language_scripts (
        code               varchar(4) NOT NULL PRIMARY KEY,
	    unicode_name       varchar    NULL,
	    iso_name           varchar    NULL,
	    directionality     varchar    NULL,
	    num_characters     varchar    NULL,
	    notes              varchar    NULL,
	    hex_start          varchar    NULL,
	    hex_end            varchar    NULL,
	    ascii_start        int4       NULL,
	    ascii_end          int4       NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn language_usage_types<'a>() -> &'a str {

    r#"drop table if exists lkup.language_usage_types;
    CREATE TABLE lkup.language_usage_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn object_access_types<'a>() -> &'a str {

    r#"drop table if exists lkup.object_access_types;
    CREATE TABLE lkup.object_access_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn object_classes<'a>() -> &'a str {

    r#"drop table if exists lkup.object_classes;
    CREATE TABLE lkup.object_classes (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn object_filter_types<'a>() -> &'a str {

    r#"drop table if exists lkup.object_filter_types;
    CREATE TABLE lkup.object_filter_types (
        id                 int4       NOT NULL PRIMARY KEY,
        filter_as          varchar    NOT NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn object_relationship_types<'a>() -> &'a str {

    r#"drop table if exists lkup.object_relationship_types;
    CREATE TABLE lkup.object_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn object_types<'a>() -> &'a str {

    r#"drop table if exists lkup.object_types;
    CREATE TABLE lkup.object_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        object_class_id    int4       NULL,
	    filter_as_id       varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn org_attribute_types<'a>() -> &'a str {

    r#"drop table if exists lkup.org_attribute_types;
    CREATE TABLE lkup.org_attribute_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        data_type          varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL

    );"#
}


pub fn org_classes<'a>() -> &'a str {

    r#"drop table if exists lkup.org_classes;
    CREATE TABLE lkup.org_classes (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
        
    );"#
}


pub fn org_name_qualifier_types<'a>() -> &'a str {

    r#"drop table if exists lkup.org_name_qualifier_types;
    CREATE TABLE lkup.org_name_qualifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn org_relationship_types<'a>() -> &'a str {

    r#"drop table if exists lkup.org_relationship_types;
    CREATE TABLE lkup.org_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn org_types<'a>() -> &'a str {

    r#"drop table if exists lkup.org_types;
    CREATE TABLE lkup.org_types (
        id                 int4       NOT NULL PRIMARY KEY,
        class_id           int4       NULL,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

    
pub fn resource_types<'a>() -> &'a str {

    r#"drop table if exists lkup.resource_types;
    CREATE TABLE lkup.resource_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn size_units<'a>() -> &'a str {

    r#"drop table if exists lkup.size_units;
    CREATE TABLE lkup.size_units (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn study_feature_categories<'a>() -> &'a str {

    r#"drop table if exists lkup.study_feature_categories;
    CREATE TABLE lkup.study_feature_categories (
        id                 int4       NOT NULL PRIMARY KEY,
        feature_type_id    int4       NOT NULL,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn study_feature_types<'a>() -> &'a str {

    r#"drop table if exists lkup.study_feature_types;
    CREATE TABLE lkup.study_feature_types (
        id                 int4       NOT NULL PRIMARY KEY,
        context            varchar    NULL,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn study_relationship_types<'a>() -> &'a str {

    r#"drop table if exists lkup.study_relationship_types;
    CREATE TABLE lkup.study_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn study_statuses<'a>() -> &'a str {

    r#"drop table if exists lkup.study_statuses;
    CREATE TABLE lkup.study_statuses (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn study_types<'a>() -> &'a str {

    r#"drop table if exists lkup.study_types;
    CREATE TABLE lkup.study_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn time_units<'a>() -> &'a str {

    r#"drop table if exists lkup.time_units;
    CREATE TABLE lkup.time_units (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn title_types<'a>() -> &'a str {

    r#"drop table if exists lkup.title_types;
    CREATE TABLE lkup.title_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        applies_to         varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn topic_types<'a>() -> &'a str {

    r#"drop table if exists lkup.topic_types;
    CREATE TABLE lkup.topic_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
        value_type         varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn topic_vocabularies<'a>() -> &'a str {

    r#"drop table if exists lkup.topic_vocabularies;
    CREATE TABLE lkup.topic_vocabularies (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn trial_registries<'a>() -> &'a str {

    r#"drop table if exists lkup.trial_registries;
    CREATE TABLE lkup.trial_registries (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn reset_message_sql <'a>() -> &'a str {
    r#"SET client_min_messages TO NOTICE;"#
}
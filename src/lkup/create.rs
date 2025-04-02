
pub fn get_schema_sql <'a>() -> &'a str {
    r#"SET client_min_messages TO WARNING; 
    create schema if not exists src;"#
}

pub fn contribution_types<'a>() -> &'a str {

    r#"drop table if exists lup.contribution_types;
    CREATE TABLE lup.contribution_types (
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

    r#"drop table if exists lup.dataset_consent_types;
    CREATE TABLE lup.dataset_consent_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn dataset_deidentification_levels<'a>() -> &'a str {

    r#"drop table if exists lup.dataset_deidentification_levels;
    CREATE TABLE lup.dataset_deidentification_levels (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn dataset_recordkey_types<'a>() -> &'a str {

    r#"drop table if exists lup.dataset_recordkey_types;
    CREATE TABLE lup.dataset_recordkey_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn date_types<'a>() -> &'a str {

    r#"drop table if exists lup.date_types;
    CREATE TABLE lup.date_types (
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

    r#"drop table if exists lup.description_types;
    CREATE TABLE lup.description_types (
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

    r#"drop table if exists lup.doi_status_types;
    CREATE TABLE lup.doi_status_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn gender_eligibility_types<'a>() -> &'a str {

    r#"drop table if exists lup.gender_eligibility_types;
    CREATE TABLE lup.gender_eligibility_types (
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

    r#"drop table if exists lup.identifier_types;
    CREATE TABLE lup.identifier_types (
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

    r#"drop table if exists lup.iec_level_types;
    CREATE TABLE lup.iec_level_types (
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

    r#"drop table if exists lup.language_codes;
    CREATE TABLE lup.language_codes (
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

    r#"drop table if exists lup.language_scripts;
    CREATE TABLE lup.language_scripts (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        code               varchar(4) NULL,
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

    r#"drop table if exists lup.language_usage_types;
    CREATE TABLE lup.language_usage_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn object_access_types<'a>() -> &'a str {

    r#"drop table if exists lup.object_access_types;
    CREATE TABLE lup.object_access_types (
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

    r#"drop table if exists lup.object_classes;
    CREATE TABLE lup.object_classes (
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

    r#"drop table if exists lup.object_filter_types;
    CREATE TABLE lup.object_filter_types (
        id                 int4       NOT NULL PRIMARY KEY,
        filter_as          varchar    NOT NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn object_relationship_types<'a>() -> &'a str {

    r#"drop table if exists lup.object_relationship_types;
    CREATE TABLE lup.object_relationship_types (
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

    r#"drop table if exists lup.object_types;
    CREATE TABLE lup.object_types (
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

    r#"drop table if exists lup.org_attribute_types;
    CREATE TABLE lup.org_attribute_types (
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

    r#"drop table if exists lup.org_classes;
    CREATE TABLE lup.org_classes (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
        
    );"#
}


pub fn org_name_qualifier_types<'a>() -> &'a str {

    r#"drop table if exists lup.org_name_qualifier_types;
    CREATE TABLE lup.org_name_qualifier_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn org_relationship_types<'a>() -> &'a str {

    r#"drop table if exists lup.org_relationship_types;
    CREATE TABLE lup.org_relationship_types (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn org_types<'a>() -> &'a str {

    r#"drop table if exists lup.org_types;
    CREATE TABLE lup.org_types (
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

    r#"drop table if exists lup.resource_types;
    CREATE TABLE lup.resource_types (
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

    r#"drop table if exists lup.size_units;
    CREATE TABLE lup.size_units (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn study_feature_categories<'a>() -> &'a str {

    r#"drop table if exists lup.study_feature_categories;
    CREATE TABLE lup.study_feature_categories (
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

    r#"drop table if exists lup.study_feature_types;
    CREATE TABLE lup.study_feature_types (
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

    r#"drop table if exists lup.study_relationship_types;
    CREATE TABLE lup.study_relationship_types (
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

    r#"drop table if exists lup.study_statuses;
    CREATE TABLE lup.study_statuses (
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

    r#"drop table if exists lup.study_types;
    CREATE TABLE lup.study_types (
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

    r#"drop table if exists lup.time_units;
    CREATE TABLE lup.time_units (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    use_in_data_entry  bool       NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}

pub fn title_types<'a>() -> &'a str {

    r#"drop table if exists lup.title_types;
    CREATE TABLE lup.title_types (
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

    r#"drop table if exists lup.topic_types;
    CREATE TABLE lup.topic_types (
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

    r#"drop table if exists lup.topic_vocabularies;
    CREATE TABLE lup.topic_vocabularies (
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

    r#"drop table if exists lup.trial_registries;
    CREATE TABLE lup.trial_registries (
        id                 int4       NOT NULL PRIMARY KEY,
        name               varchar    NULL,
        description        varchar    NULL,
	    list_order         int4       DEFAULT 10 NOT NULL,
	    source_org         varchar    NULL,
	    date_added         date       NULL
    );"#
}


pub fn get_message_sql <'a>() -> &'a str {
    r#"SET client_min_messages TO NOTICE;"#
}
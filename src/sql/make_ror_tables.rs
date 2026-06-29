pub fn get_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 

    drop table if exists orgs.ror_orgs;
    create table orgs.ror_orgs
    (
          id                varchar     not null primary key
        , ror_full_id       varchar     not null
        , ror_name          varchar     not null	
        , status            int         not null default 1
        , established       int         null
        , location          varchar     null
        , csubdiv_code      varchar     null
        , country_code      varchar     null
    );
        
    drop table if exists orgs.ror_names;
    create table orgs.ror_names
    (
          id                varchar     not null
        , name              varchar     not null  
        , name_corrected    varchar     null 
        , name_to_match     varchar     null  
        , name_type         int         not null 
        , lang_code         varchar     null
        , lang_source       varchar     null
        , script_code       varchar     null
    );
    create index names_idx on orgs.ror_names(id);

    drop table if exists orgs.ror_rels;
    create table orgs.ror_rels
    (
          id                varchar     not null
        , ror_name          varchar     not null
        , rel_type          int         not null
        , related_id        varchar     not null
        , related_name      varchar     not null
    );  
    create index relationships_idx on orgs.ror_rels(id);

    drop table if exists orgs.ror_types;
    create table orgs.ror_types
    (
          id                varchar     not null
        , ror_name          varchar     not null
        , org_type          int         not null
    );  
    create index type_idx on orgs.ror_types(id);

    drop table if exists orgs.ror_locs;
    create table orgs.ror_locs
    (
          id                varchar     not null
        , ror_name          varchar     not null
        , geonames_id       int         null
        , location          varchar     null	
        , lat               real        null
        , lng               real        null
        , cont_code         varchar     null
        , cont_name         varchar     null
        , country_code      varchar     null
        , country_name      varchar     null
        , csubdiv_code      varchar     null  
        , csubdiv_name      varchar     null	
    );
    create index locations_idx on orgs.ror_locs(id);

    drop table if exists orgs.ror_countries;
    create table orgs.ror_countries
    (
          id                varchar     not null
        , country_code      varchar     null
    );
    create index countries_idx on orgs.ror_countries(id);

    drop table if exists orgs.ror_links;
    create table orgs.ror_links
    (
          id                varchar     not null
        , ror_name          varchar     not null  	  
        , link_type         int         not null
        , link              varchar     not null
    );
    create index links_idx on orgs.ror_links(id);

    drop table if exists orgs.ror_ext_ids;
    create table orgs.ror_ext_ids
    (
          id                varchar     not null
        , ror_name          varchar     not null	
        , id_type           int         not null
        , id_value          varchar     not null
        , is_preferred      bool        not null default false
    );
    create index external_ids_idx on orgs.ror_ext_ids(id);
   
    drop table if exists orgs.ror_domains;
    create table orgs.ror_domains
    (
          id                varchar     not null
        , ror_name          varchar     not null
        , domain            varchar     not null
    );  
    create index domains_idx on orgs.ror_domains(id);

    SET client_min_messages TO NOTICE;"#
}

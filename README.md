
DEVELOPMENT YET TO BEGIN

## Introduction

The intention of this system is to extract the MDR specific parts of contextual data management into a separate system, so as not to confuse the processes involved in importing ROR, Geonames, UMLS or any other contextual data. Thus utilities such as imp_ror, imp_geo and imp_uml should leave data relatively close to its 'natural' state, and in particular without additions, deletions or categorisations directed by the needs of ther MDR. In that way those systems both retain a more general utility and are easier to maintain. N.B. The MDR is the clinical research metadata repository - see other repos on this page.

MDR driven changes will instead be the responsibility of this program. These include, for example. the addition of alternate names for cities, countries and organisations, and the mapping of different UML schemes for disease nomenclature. This program is also charged with providing such contextual data in the form in which it can be directly used within the MDR, and with loading the data within the relevant tables of a 'cxt' database.

The main contextual data are currently stored in:
- Lookup tables, e.g. supporting categorised data points such as study and object types, study features, language codes etc. These tables can be (re)created as required within the 'lup' scvhema of the cxt database. They are not dependent on external data sources. Recreation would normally only be after any revision of the category systems in one or more tables. 
- Location data tables, including coded lists of cities and countries and other geographical entities. These are imported from the 'geo' database, and procesed further within the 'locs' schema of the cxt DB. Tables used directly by the MDR are copied to the 'cxt' schema within the cxt database.
- Organisation data tables, with coded lists of organisations and organisation names. These are imported from the 'ror' database, and procesed further within the 'ror' schema of the cxt DB. Tables used directly by the MDR are copied to the 'cxt' schema within the cxt database.
- Medical vocabulary data tables, with coded categorisations of medical conditions, drugs etc. These will bw imported from the 'uml' database, and procesed further within the 'uml' schema of the cxt DB. Tables used directly by the MDR are copied to the 'cxt' schema within the cxt database.

Other areas / types of data may be added at a later point -  e.g. tables linking PubMed and PubMedCentral IDs with DoIs, or CrossRef data, if they are seen as useful to the MDR.

The program is designed to be used by an IT professional within a Linux environment using Postgres as the DB backend. Familiarity with Postgres and a linux development environment (e.g. VS Code, Zed) is assumed. The program is not aimed at MDR end users.

## Functionality

The program can be run with one of 4 different flags:
- 'cargo run -- -k' recreates the lookup tables. The system holds both the definition and contents of these tables. The tables can therefore be modified, as and when necessary, by directly editing the code and then rerunning the program using this flag. Any changes that would risk 'orphaning' existing data within categorised fields in the MDR need to be avoided. Once categories are established and in use, therefore, most changes would involve *adding* additional category values, or amending the descriptions attached to a particular value.
- 'cargo run -- -g' re-imports the latest data from the ror databse (created by imp_ror) and processes it within the ror schema. Data related to multi-site companies are simplified, as the MDR only requires one entry per company. Organisations from ROR are integrated with those used in the MDR which currently have no ROR equivalent. Names from both systems are integrated and are simplified and made more consistent. A 'names_to_match' table is constructed. Organisation type is also integrated more simply with the organisation data.
- 'cargo run -- -c' re-imports the latest data from the geo databse (created by imp_geo) and processes it within the locs schema. Additional versions of city and country names, as found in the MDR data sources, are added. 
- 'cargo run -- -u' will re-import the latest data from the uml databse (created by imp_uml) and processes it, making relevant tables of conditions, topics and interventions available to the MDR system. ***N.B. To be implemented***


## Configuration file

The system makes use of a small toml configuration file that includes details of the database connection parameters, as well as the location for log files (a log file is generated each time the program is run). The file has the form:
```
[folders]
log_folder_path="path to log folder"

[database]
db_host="localhost"
db_user="<user_name>"
db_password="<password>"
db_port="5432"

cnxt_db_name="cxt"
orgs_db_name="ror"
locs_db_name="geo"
umls_db_name="uml"
`````
Values for log_folder_path, db_user and db_password are all essential nad must be provided.  
Values for db_host, db_port and the 4 database names can all take defaults - which are the values shown above - but in each case can be changed to reflect the actual values used for the host, port and databses.  
The file is stored at the conventional location for configuration files in Linux:
```
/home/<user name>/.config/mk_cxt/config.toml
```
The file must therefore be created in a text editor and placed at that location, creating the relevant folders as necessaary.

## Please note
DEVELOPMENT YET TO BEGIN

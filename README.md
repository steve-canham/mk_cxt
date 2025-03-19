
DEVELOPMENT YET TO BEGIN

<h2>Introduction</h2>

The intention of this system is to extract the MDR specific parts of contextual data management into a separate system, so as not to confuse the processes involved in importing ROR, Geonames, UMLS or other contextual data. Thus utilities such as imp_ror, imp_geo and imp_uml should leave data relatively close to its 'natural' state, and in particular without additions, deletions or categorisations directed by the needs of ther MDR. In that way they both retain a more general utility and are easier to maintain.<br/>
<br/>
MDR driven changes will instead be the responsibility of this program. These include, for example the addition of alternate names for cities, countries and organisations, the addition of publishing comnpanies to the organisation tables, and the mapping of different UML schemes for disease nomenclature. In addition, this program is charged with providing such contextual data in the form in which it can be directly used within the MDR, and with loading the data within the relevant tables of a 'cxt' database.<br/>
<br/>
DEVELOPMENT YET TO BEGIN

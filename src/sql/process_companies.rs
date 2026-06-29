
pub fn get_create_companies_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING;

    drop table if exists orgs.companies;
    create table orgs.companies (
        id                 varchar   not null  primary key
      , orig_ror_name      varchar   not null
      , mod_ror_name       varchar   not null
      , bare_name          varchar   null
      , country            varchar   null
      , loc_city           varchar   null
      , loc_country        varchar   null
      , parent_id          varchar   null
      , is_parent          bool      null
      , parent_ror_id      varchar   null
      , parent_ror_name    varchar   null
    );

    insert into orgs.companies (id, orig_ror_name, mod_ror_name)
    select g.id, g.ror_name, g.ror_name
    from orgs.ror_orgs g
    inner join orgs.ror_types t
    on g.id = t.id
    where org_type = 400
    and g.status <> 3;

    update orgs.companies c
    set loc_city = t.location,
    loc_country = t.country_name
    from orgs.ror_locs t
    where c.id = t.id;

    SET client_min_messages TO NOTICE;"#
}


pub fn correct_company_brackets_sql<'a>() -> &'a str {

    r#"
    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(UK) ', '')
    where mod_ror_name like '%(UK) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(U.K.) ', '')
    where mod_ror_name like '%(U.K.) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Beijing, China)', '(China)')
    where mod_ror_name like '%(Beijing, China)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Beijing) ', '')
    where mod_ror_name like '%(Beijing) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(bv) ', '')
    where mod_ror_name like '%(bv) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(China) ', '')
    where mod_ror_name like '%(China) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Company)', '')
    where mod_ror_name like '%(Company)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Malaysia) ', '')
    where mod_ror_name like '%(Malaysia) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(R&D) ', '')
    where mod_ror_name like '%(R&D) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(publ) ', '')
    where mod_ror_name like '%(publ) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Pty) ', '')
    where mod_ror_name like '%(Pty) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Group) ', '')
    where mod_ror_name like '%(Group) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Guangdong)', '')
    where mod_ror_name like '%(Guangdong)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Guangxi) ', '')
    where mod_ror_name like '%(Guangxi) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Hangzhou) ', '')
    where mod_ror_name like '%(Hangzhou) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Hong Kong) Limited', '[Hong Kong] Limited')
    where mod_ror_name like '%(Hong Kong) Limited%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Jiangsu)', '[Jiangsu]')
    where mod_ror_name like '%(Jiangsu)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Little Red Book)', '[Little Red Book]')
    where mod_ror_name like '%(Little Red Book)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Nanjing, China)', '(China)')
    where mod_ror_name like '%(Nanjing, China)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Nanjing) ', '')
    where mod_ror_name like '%(Nanjing) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(NI) ', '[Northern ireland]')
    where mod_ror_name like '%(NI) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(NIRC)', '[NIRC]')
    where mod_ror_name like '%(NIRC)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(NIT) ', '')
    where mod_ror_name like '%(NIT) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(OPC)', '[OPC]')
    where mod_ror_name like '%(OPC)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(qGenomics)', '[qGenomics]')
    where mod_ror_name like '%(qGenomics)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(REPA)', '[REPA]')
    where mod_ror_name like '%(REPA)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, ' (Rybářství Litomyšl)', '')
    where mod_ror_name like '% (Rybářství Litomyšl)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Shanghai, China)', '(China)')
    where mod_ror_name like '%(Shanghai, China)%';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Shanghai) ', '')
    where mod_ror_name like '%(Shanghai) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Shenzhen) ', '')
    where mod_ror_name like '%(Shenzhen) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Suzhou) ', '')
    where mod_ror_name like '%(Suzhou) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Wuhan) ', '')
    where mod_ror_name like '%(Wuhan) %';

    update orgs.companies c
    set mod_ror_name = replace(mod_ror_name, '(Nanning) ', '')
    where mod_ror_name like '%(Nanning) %';

    update orgs.companies c
    set bare_name = trim(split_part(mod_ror_name, '(', 1))
    , country = trim(rtrim(split_part(mod_ror_name, '(', 2), ')'))
    where mod_ror_name like '%(%';

    update orgs.companies c
    set bare_name = trim(mod_ror_name)
    where bare_name is null;

    -- ******* change to include id for parent

    update orgs.companies c
    set is_parent = true
    from orgs.ror_rels r
    where c.id = r.id
    and r.rel_type = 2;

    update orgs.companies c
    set parent_id = r.related_id
    from orgs.ror_rels r
    where c.id = r.id
    and r.rel_type = 1;
    "#
}

pub fn remove_company_suffixes_sql<'a>() -> &'a str {

    r#"
    -- retain these periods
    update orgs.companies
    set bare_name = replace(bare_name, '.', '||')
    where bare_name ilike '%.com%'
    or bare_name ilike '%.org%'
    or bare_name ilike '%.cz'
    or bare_name ilike '%.no'
    or bare_name ilike '%.cz'
    or bare_name ilike '%.ie';

    -- replace the parentheses back again

    update orgs.companies
    set bare_name = replace(bare_name, '[', '(')
    where bare_name ilike '%[%';

    update orgs.companies
    set bare_name = replace(bare_name, ']', ')')
    where bare_name ilike '%]%';

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 19)
    where bare_name ilike '% Sp. z o. o. (ltd.)' ;

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 14)
    where bare_name ilike '% GmbH & Co. KG' ;
    --6

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 12)
    where bare_name ilike '% Sp. z o. o.%';
    --2

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 11)
    where bare_name ilike '% Sp. z o.o.%';
    --8

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 10)
    where bare_name ilike '% Co., Ltd.'
    or bare_name ilike '% Ltd. Şti.'
    or bare_name ilike '% MFG., Co.';
    --963

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 9)
    where bare_name ilike '% Co., Ltd'
    or bare_name ilike '% Mfg.Co.,'
    or bare_name ilike '% MFG. Co.';
    --14

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 8)
    where bare_name ilike '% Co.ltd.';
    --2

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 7)
    where bare_name ilike '% (ltd.)'
    or bare_name ilike '% S.A.S.'
    or bare_name ilike '% S.p.A.'
    or bare_name ilike '% s.r.o.'
    or bare_name ilike '% d.o.o.'
    or bare_name ilike '% s.r.l.'
    or bare_name ilike '% S.L.U.';
    --37

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 6)
    where bare_name ilike '%, Inc.'
    or bare_name ilike '% MFG.,'
    or bare_name ilike '% IND.,'
    or bare_name ilike '% K. K.'
    or bare_name ilike '% d.o.o'
    or bare_name ilike '% S. L.';
    --284

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 5)
    where bare_name ilike '% Ltd.'
    or bare_name ilike '%,ltd.'
    or bare_name ilike '%.ltd.'
    or bare_name ilike '% Ltda'
    or bare_name ilike '% GmbH'
    or bare_name ilike '% S.A.'
    or bare_name ilike '%, LLC'
    or bare_name ilike '%, Inc'
    or bare_name ilike '% Inc.'
    or bare_name ilike '% MFG.'
    or bare_name ilike '% IND,'
    or bare_name ilike '% K.K.'
    or bare_name ilike '% S.L.'
    or bare_name ilike '% Pvt.'
    or bare_name ilike '% Pte.'
    or bare_name ilike '% Pty.'
    or bare_name ilike '% B.V.';
    --635

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 4)
    where bare_name ilike '% Ltd'
    or bare_name ilike '% SAS'
    or bare_name ilike '% LLC'
    or bare_name ilike '% Inc'
    or bare_name ilike '% SpA'
    or bare_name ilike '% Mfg'
    or bare_name ilike '% Srl'
    or bare_name ilike '% S.L'
    or bare_name ilike '% SLU'
    or bare_name ilike '% Pte'
    or bare_name ilike '% Pty';
    --122

    update orgs.companies
    set bare_name = substring(bare_name, 1, length(bare_name) - 3)
    where bare_name ilike '% SA'
    or bare_name ilike '% SL'
    or bare_name ilike '% AG'
    or bare_name ilike '% BV';
    --83

    update orgs.companies
    set bare_name = replace(bare_name, ', Inc., ', ', ')
    where bare_name ilike '%, Inc., %';
    --2

    update orgs.companies
    set bare_name = replace(bare_name, ' Incorporation', '')
    where bare_name ilike '% Incorporation';
    --13

    update orgs.companies
    set bare_name = replace(bare_name, ' Incorporated', '')
    where bare_name ilike '% Incorporated';
    --31

    update orgs.companies
    set bare_name = replace(bare_name, ' Corp.', 'Corporation')
    where bare_name ilike '% Corp.';
    --39

    update orgs.companies
    set bare_name = replace(bare_name, ' Co', 'Company')
    where bare_name ilike '% Co';
    --12

    update orgs.companies
    set bare_name = replace(bare_name, ' Co.', 'Company')
    where bare_name ilike '% Co.';
    --35

    update orgs.companies
    set bare_name = replace(bare_name, 'Dr.', 'Dr ')
    where bare_name ilike 'Dr.%';
    --25

    update orgs.companies
    set bare_name = replace(bare_name, ' Beratungsgesellschaft m.b.H', '')
    where bare_name ilike '% Beratungsgesellschaft m.b.H.';
    --1

    update orgs.companies
    set bare_name = replace(bare_name, ' Private Limited', '')
    where bare_name ilike '% Private Limited';
    --7

    update orgs.companies
    set bare_name = replace(bare_name, ' & Co. Limited', '')
    where bare_name ilike '% & Co. Limited';
    --1

    update orgs.companies
    set bare_name = replace(bare_name, ' Corp. Limited', '')
    where bare_name ilike '% Corp. Limited';
    --1

    update orgs.companies
    set bare_name = replace(bare_name, '.', '')
    where bare_name ilike '%.%';
    --239

    update orgs.companies
    set bare_name = replace(bare_name, ',', '')
    where bare_name ilike '%,%';
    --155

    update orgs.companies
    set bare_name = replace(bare_name, '"', '')
    where bare_name ilike '%"%';
    --9

    -- keep right quote in name?

    update orgs.companies
    set bare_name = replace(bare_name, '’', '')
    where bare_name ilike '%’%';
    --23

    -- ?? change apostrophe to right quote?

    update orgs.companies
    set bare_name = replace(bare_name, '''', '')
    where bare_name ilike '%''%';
    --80

    -- replace at end
    update orgs.companies
    set bare_name = replace(bare_name, '  ', ' ')
    where bare_name ilike '%  %';
    --23

    -- restore the periods to keep
    update orgs.companies
    set bare_name = replace(bare_name, '||', '.')
    where bare_name ilike '%||%';
    --14
    "#
}

pub fn create_revised_orgs_sql<'a>() -> &'a str {

    r#"
    
    drop table if exists orgs.rev_ror_orgs;
    create table orgs.rev_ror_orgs 
    (
	  id                varchar     not null primary key
	, ror_full_id       varchar     not null
	, ror_name          varchar     not null	
	, status            int         not null default 1
	, established       int         null
	, ended             int         null
	, notes             varchar     null
	, location          varchar     null
	, csubdiv_code      varchar     null
	, country_code      varchar     null
	, is_govment        bool        null
	, is_education      bool        null
	, is_health         bool        null
	, is_company        bool        null
	, is_nonprofit      bool        null
	, is_funder         bool        null
	, is_facility       bool        null
	, is_archive        bool        null
	, is_other          bool        null
    );
    
    -- add in the non-companies
    insert into orgs.rev_ror_orgs (id, ror_full_id, ror_name, status,           
	established, location, csubdiv_code, country_code)
        select g.id, ror_full_id, ror_name, status,           
	established, location, csubdiv_code, country_code
	from orgs.ror_orgs g
	left join orgs.companies c
	on g.id = c.id
	where c.id is null
	and g.status <> 3;
	    
    update orgs.rev_ror_orgs rr
    set is_govment = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 100;
     
    update orgs.rev_ror_orgs rr
    set is_education = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 200;
    
    update orgs.rev_ror_orgs rr
    set is_health = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 300;
    
    update orgs.rev_ror_orgs rr
    set is_nonprofit = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 500;
    
    update orgs.rev_ror_orgs rr
    set is_funder = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 600;
    
    update orgs.rev_ror_orgs rr
    set is_facility = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 700;
    
    update orgs.rev_ror_orgs rr
    set is_archive = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 800;
       
    update orgs.rev_ror_orgs rr
    set is_other = true
    from orgs.ror_types t
    where rr.id = t.id and t.org_type = 900;
    "#
}



pub fn process_multisite_companies_sql<'a>() -> &'a str {

    r#"
    
    "#
}


/*
 * for later use

select bare_name, count(id)
from orgs.companies
group by bare_name
having count(id) > 1
order by count(id) desc;


select bare_name, count(id)
from orgs.companies
group by bare_name
having count(id) = 1; *
 *
 *
 */

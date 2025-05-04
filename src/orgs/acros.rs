
/* 

drop table if exists orgs.ror_acronyms;
create table orgs.ror_acronyms (
     id  varchar
   , name varchar
   , name_to_match varchar
   , lang_code varchar
   , source_name varchar
);
create index ror_acrs_id on orgs.ror_acronyms(id);
create index ror_acrs_ntm on orgs.ror_acronyms(name_to_match);



insert into orgs.ror_acronyms (id, name, name_to_match)
select id, name, name_to_match
from orgs.ror_names
where name_type = 10
and lang_code is null;


drop table if exists orgs.ror_derived_acronyms;
create table orgs.ror_derived_acronyms (
     id  varchar
   , name varchar
   , name_to_match varchar
   , lang_code varchar
   , acro_base varchar
   , der_acro varchar
   , der_acro_wo_of varchar
   , der_acro_wo_ofand varchar
   , der_acro_wo_allsw varchar
);
create index ror_der_acrs_id on orgs.ror_derived_acronyms(id);
create index ror_der_acrs_ntm on orgs.ror_derived_acronyms(name_to_match);


insert into orgs.ror_derived_acronyms (id, name, name_to_match, lang_code, acro_base)
select n.id, n.name, n.name_to_match, n.lang_code, n.name_to_match
from orgs.ror_names n
inner join 
	(select distinct id from orgs.ror_acronyms) a
on n.id = a.id
where name_type <> 10
and script_code = 'Latn';

-- remove intial 'the 's

update orgs.ror_derived_acronyms
set acro_base = substring(acro_base, 5)
where acro_base like 'the %';

select * from orgs.ror_derived_acronyms
where name_to_match like 'the %';


drop table if exists orgs.ror_acro_words;
create table orgs.ror_acro_words (
     id int not null generated always as identity
   , ror_id  varchar
   , acro_base varchar
   , word varchar
);
create index ror_acro_words_id on orgs.ror_acro_words(id, acro_base);

insert into orgs.ror_acro_words (ror_id, acro_base, word)
select id, acro_base, regexp_split_to_table(acro_base, '\s+') as word
	 from orgs.ror_derived_acronyms;

-- derive full der acro

update orgs.ror_derived_acronyms a
set der_acro = f.ac
from (  
        select ror_id, acro_base, string_agg(substring(word, 1, 1), '' order by id) as ac
		from orgs.ror_acro_words
		group by ror_id, acro_base ) f
where a.id = f.ror_id
and a.acro_base = f.acro_base;

-- where der_acro is only one letter cannot be an acronym
-- as all are at least two letters

delete from orgs.ror_derived_acronyms 
where length(der_acro) = 1

-- 699 go

-- remove 'of' and 'the' from listing of name words

delete from orgs.ror_acro_words
where word in ('of', 'the', 'de', 'des', 'du', 'la', 'le', 'les', 'los', 'der', 'del', 'di', 'el');

-- about 30,799 go

-- need to remove initial l' from (french) words

update orgs.ror_acro_words
set word = substring(word, 3)
where word like 'l’%'

-- 769

update orgs.ror_derived_acronyms a
set der_acro_wo_of = f.ac
from (  
        select ror_id, acro_base, string_agg(substring(word, 1, 1), '' order by id) as ac
		from orgs.ror_acro_words
		group by ror_id, acro_base ) f
where a.id = f.ror_id
and a.acro_base = f.acro_base;

-- remove 'and' and '&' from listing of name words

delete from orgs.ror_acro_words
where word in ('and', '&', 'et', 'e', 'und', 'i');

--12298

update orgs.ror_derived_acronyms a
set der_acro_wo_ofand = f.ac
from (  
        select ror_id, acro_base, string_agg(substring(word, 1, 1), '' order by id) as ac
		from orgs.ror_acro_words
		group by ror_id, acro_base ) f
where a.id = f.ror_id
and a.acro_base = f.acro_base;

-- remove other stop words from listing of name words

delete from orgs.ror_acro_words
where word in ('for', 'für', 'in', 'en', 'y', 'on', 'a', 'v', 'pour', 'sur', 'à', 'voor', 'o', '/');

-- 11222 go

update orgs.ror_derived_acronyms a
set der_acro_wo_allsw = f.ac
from (  
        select ror_id, acro_base, string_agg(substring(word, 1, 1), '' order by id) as ac
		from orgs.ror_acro_words
		group by ror_id, acro_base ) f
where a.id = f.ror_id
and a.acro_base = f.acro_base;


select * from orgs.ror_derived_acronyms

select word, count(id) 
from orgs.ror_acro_words
where length(word) < 5
group by word
order by count(id) desc


select a.id, a.name, a.name_to_match, d.name, d.name_to_match, d.lang_code, d.der_acro_wo_allsw
from orgs.ror_acronyms a
inner join orgs.ror_derived_acronyms d
on a.id = d.id
and a.name_to_match = d.der_acro_wo_allsw


update orgs.ror_acronyms y
set lang_code = langs,
source_name = source
from
	(select id, name_to_match, source, string_agg(lang_code, ', ') as langs
	from 
		(select a.id, a.name, a.name_to_match, d.lang_code, d.name_to_match as source
		from orgs.ror_acronyms a
		inner join orgs.ror_derived_acronyms d
		on a.id = d.id
		and a.name_to_match = d.der_acro_wo_allsw) as w
	group by id, name_to_match, source) x
where y.id = x.id
and y.name_to_match = x.name_to_match

-- 25435

update orgs.ror_acronyms y
set lang_code = langs,
source_name = source
from
	(select id, name_to_match, source, string_agg(lang_code, ', ') as langs
	from 
		(select a.id, a.name, a.name_to_match, d.lang_code, d.name_to_match as source
		from orgs.ror_acronyms a
		inner join orgs.ror_derived_acronyms d
		on a.id = d.id
		and a.name_to_match = d.der_acro_wo_ofand
		and a.lang_code is null) as w
	group by id, name_to_match, source) x
where y.id = x.id
and y.name_to_match = x.name_to_match

--276

update orgs.ror_acronyms y
set lang_code = langs,
source_name = source
from
	(select id, name_to_match, source, string_agg(lang_code, ', ') as langs
	from 
		(select a.id, a.name, a.name_to_match, d.lang_code, d.name_to_match as source
		from orgs.ror_acronyms a
		inner join orgs.ror_derived_acronyms d
		on a.id = d.id
		and a.name_to_match = d.der_acro_wo_of
		and a.lang_code is null) as w
	group by id, name_to_match, source) x
where y.id = x.id
and y.name_to_match = x.name_to_match

--131


update orgs.ror_acronyms y
set lang_code = langs,
source_name = source
from
	(select id, name_to_match, source, string_agg(lang_code, ', ') as langs
	from 
		(select a.id, a.name, a.name_to_match, d.lang_code, d.name_to_match as source
		from orgs.ror_acronyms a
		inner join orgs.ror_derived_acronyms d
		on a.id = d.id
		and a.name_to_match = d.der_acro
		and a.lang_code is null) as w
	group by id, name_to_match, source) x
where y.id = x.id
and y.name_to_match = x.name_to_match

--442


select *
from orgs.ror_acronyms a
order by name_to_match 


select count(*)
from orgs.ror_acronyms a
where lang_code is not null


der_acro varchar
   , der_acro_wo_of varchar
   , der_acro_wo_ofand varchar


-- get orgs with single language non acronym names
-- list the ids and language
   
update orgs.ror_acronyms y
set lang_code = x.lang_code
from
   (select distinct n.id, n.lang_code from orgs.ror_names n
   inner join
		(select id, count(lang_code) from orgs.ror_names
		where name_type <> 10
		group by id
		having count(lang_code) = 1) s
   on n.id = s.id
   where n.name_type <> 10
   and lang_code <> 'cm') x 
where y.id = x.id
and y.lang_code is null






*/

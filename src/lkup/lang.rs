pub fn language_codes<'a>() -> &'a str {

    r#"insert into lup.language_codes (code, marc_code,
	    lang_name_en, lang_name_fr, lang_name_de,
        is_major, source_org, date_added)
    values

    );"#
}

pub fn language_scripts<'a>() -> &'a str {

    r#"insert into lup.language_scripts (id, name, code,
	    unicode_name, iso_name, directionality,
	    num_characters, notes, hex_start, hex_end,
	    ascii_start, ascii_end, source_org, date_added)
    values()
    
    ;"#
}

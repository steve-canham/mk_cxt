/***************************************************************************
 *
 ***************************************************************************/

 use clap::{command, Arg, ArgMatches};
 use crate::err::AppError;
 use std::ffi::OsString;
 
 pub struct CliPars {
     pub flags: Flags, 
 }
 
 #[derive(Debug, Clone, Copy)]
 pub struct Flags {
     pub create_lups: bool,
     pub import_orgs: bool,
     pub import_locs: bool,
     pub import_umls: bool,
     pub import_pubs: bool,
 }
 
 pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
 { 
     let parse_result = parse_args(args)?;
  
     // Flag values are false if not present, true if present.
      
     let mut k_flag = parse_result.get_flag("k_flag");
     let g_flag = parse_result.get_flag("g_flag");
     let c_flag = parse_result.get_flag("c_flag");
     let u_flag = parse_result.get_flag("u_flag");
     let p_flag = parse_result.get_flag("p_flag");
 
    // if no flags do the (re)creation of lookups as the default

    if !g_flag && !c_flag && !u_flag && !p_flag {
        k_flag = true;
    }

     let flags = Flags {
        create_lups: k_flag,
        import_orgs: g_flag,
        import_locs: c_flag,
        import_umls: u_flag,
        import_pubs: p_flag,
     };
      
     Ok(CliPars {
         flags: flags,
     })
 
 }
 
 
 fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {
 
     command!()
         .about("Creates or imports data from other databases to create contextual data schemas")
         .arg(
            Arg::new("k_flag")
           .short('k')
           .long("lookups")
           .required(false)
           .help("A flag signifying that lookup data needs to be (re-)created")
           .action(clap::ArgAction::SetTrue)
         )
         .arg(
             Arg::new("g_flag")
            .short('g')
            .long("orgs")
            .required(false)
            .help("A flag signifying that organisation data needs to be processed")
            .action(clap::ArgAction::SetTrue)
         )
         .arg(
             Arg::new("c_flag")
            .short('c')
            .long("locs")
            .required(false)
            .help("A flag signifying that location data needs to be processed")
            .action(clap::ArgAction::SetTrue)
         )
        .arg(
             Arg::new("u_flag")
             .short('u')
             .long("umls")
             .required(false)
             .help("A flag signifying that umls data needs to be processed")
             .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("p_flag")
            .short('p')
            .long("pubs")
            .required(false)
            .help("A flag signifying that publisher data needs to be processed")
            .action(clap::ArgAction::SetTrue)
       )
     .try_get_matches_from(args)
 
 }
 
 
 #[cfg(test)]
 mod tests {
     use super::*;
     
     // Ensure the parameters are being correctly extracted from the CLI arguments
 
     #[test]
     fn check_cli_no_explicit_params() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.create_lups, true);
         assert_eq!(res.flags.import_orgs, false);
         assert_eq!(res.flags.import_locs, false);
         assert_eq!(res.flags.import_umls, false);
         assert_eq!(res.flags.import_pubs, false);

     }
 
     #[test]
     fn check_cli_with_g_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-g"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.create_lups, false);
         assert_eq!(res.flags.import_orgs, true);
         assert_eq!(res.flags.import_locs, false);
         assert_eq!(res.flags.import_umls, false);
         assert_eq!(res.flags.import_pubs, false);
     }

     #[test]
     fn check_cli_with_all_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-g", "-c", "-u", "-k", "-p"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.create_lups, true);
         assert_eq!(res.flags.import_orgs, true);
         assert_eq!(res.flags.import_locs, true);
         assert_eq!(res.flags.import_umls, true);
         assert_eq!(res.flags.import_pubs, true);
     }
     
 }
 
 
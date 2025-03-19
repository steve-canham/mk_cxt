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
     pub import_data: bool,
     pub include_nonlatin: bool,
     pub test_run: bool,
 }
 
 pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
 { 
     let parse_result = parse_args(args)?;
  
     // Flag values are false if not present, true if present.
 
     let mut r_flag = parse_result.get_flag("r_flag");
     let n_flag = parse_result.get_flag("n_flag");
     let z_flag = parse_result.get_flag("z_flag");
     

     if !r_flag {
         r_flag = true;  // import is the default
     }
 
     let flags = Flags {
         import_data: r_flag,
         include_nonlatin: n_flag,
         test_run: z_flag,
     };
 
     Ok(CliPars {
         flags: flags,
     })
 
 }
 
 
 fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {
 
     command!()
         .about("Imports data from txt file and imports it into a database")
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
             .help("A flag signifying that umls type needs to be processed")
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
         assert_eq!(res.flags.import_data, true);
         assert_eq!(res.flags.test_run, false);
     }
 
     #[test]
     fn check_cli_with_r_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-r"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_data, true);
         assert_eq!(res.flags.include_nonlatin, false);
         assert_eq!(res.flags.test_run, false);
     }
 
     #[test]
     fn check_cli_with_n_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-n"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_data, true);
         assert_eq!(res.flags.include_nonlatin, true);
         assert_eq!(res.flags.test_run, false);
     }
 
     #[test]
     fn check_cli_with_z_flags() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-z"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_data, true);
         assert_eq!(res.flags.include_nonlatin, false);
         assert_eq!(res.flags.test_run, true);
     }
      
    
     #[test]
     fn check_cli_with_most_params_explicit() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-r", "-n", "-z"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.flags.import_data, true);
         assert_eq!(res.flags.include_nonlatin, true);
         assert_eq!(res.flags.test_run, true);
     }
 
 }
 
 
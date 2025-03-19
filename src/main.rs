
use mk_cxt::err;
use mk_cxt::run;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let args: Vec<_> = env::args_os().collect();
    match run(args).await
    {
      Ok(_) => println!("Done!"),
      Err(e) => err::report_error(e),
    };
}

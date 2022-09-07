mod errors;
mod cli;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
          log::info!("{:?}",args);
          errors::exit_with_retcode(Ok(()))
        },
        Err(e) => {
           log::error!("Error while parsing arguments:\n]t{}",e);
           errors::exit_with_retcode(Err(e));
        }
    }

}

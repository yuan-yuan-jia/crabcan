#[macro_use] extern crate scan_fmt;
mod errors;
mod ipc;
mod cli;
mod config;
mod child;
mod container;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
          log::info!("{:?}",args);
          errors::exit_with_retcode(container::start(args))
        },
        Err(e) => {
           log::error!("Error while parsing arguments:\n]t{}",e);
           errors::exit_with_retcode(Err(e));
        }
    }

}

use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug,StructOpt)]
#[structopt(name = "crabcan",about = "A simple container in Rust")]
pub struct Args {
    #[structopt(short, long)] 
    debug: bool,
    
    #[structopt(short, long)]
    pub command: String,

    #[structopt(short, long)]
    pub uid: u32,

    #[structopt(parse(from_os_str),short = "m", long = "mount")]
    pub mount_dir: PathBuf,
}


pub fn parse_args() -> Args {
    let args = Args::from_args();

    println!("args: {:?}",&args);

    args
}
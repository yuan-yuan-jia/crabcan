use std::process::exit;
use std::fmt;

#[derive(Debug)]
pub enum Errcode {
    
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            _ => write!(f,"{:?}",self)
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}




pub fn exit_with_retcode(res: Result<(),Errcode>) {
    match res {
        Ok(_) => {
           log::debug!("Exit without any error, returning 0");
           exit(0);
        },
        Err(e) => {
           let retcode = e.get_retcode();
           log::error!("Error on exit:\n\t{}\n\tReturning {}",e,retcode);
           exit(retcode);
        }
        
    }
}
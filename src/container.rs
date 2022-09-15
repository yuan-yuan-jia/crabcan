use crate::config::ContainerOpts;
use crate::cli::Args;
use crate::errors::Errcode;
use crate::child::generate_child_process;
use crate::mounts::clean_mounts;
use crate::namespace::handle_child_uid_map;
use nix::sys::utsname::uname;
use nix::unistd::{close,Pid};
use std::os::unix::io::RawFd;
use nix::sys::wait::waitpid;
pub struct Container {
    sockets: (RawFd,RawFd),
    config: ContainerOpts,
    child_pid: Option<Pid>
}


impl Container {
    pub fn new(args: Args) -> Result<Container,Errcode> {
       let (config,sockets) = ContainerOpts::new(
           args.command,
           args.uid,
           args.mount_dir
       )?;

       Ok(
        Container {
            sockets,
            config,
            child_pid: None,
        }
       )
    }


    pub fn create(&mut self) -> Result<(),Errcode> {
        let pid = generate_child_process(self.config.clone())?;
        self.child_pid = Some(pid);
        handle_child_uid_map(pid, self.sockets.0)?;
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(),Errcode> {
        log::debug!("Cleaning container");
        clean_mounts(&self.config.mount_dir)?;
        if let Err(e) = close(self.sockets.0) {
            log::error!("Unable to close write socket: {:?}",e);
            return Err(Errcode::SocketError(3));
        }
        if let Err(e) = close(self.sockets.1) {
            log::error!("Unable to close read socket: {:?}",e);
            return Err(Errcode::SocketError(4));
        }
        Ok(())
    }
}


pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(),Errcode> {
    let host = uname();
    log::debug!("Linux release: {}",host.release());

    if let Ok(version) = scan_fmt!(host.release(),"{f}.{}",f32) {
       if version < MINIMAL_KERNEL_VERSION {
          return Err(Errcode::NotSupported(0));
       } 
    } else {
       return Err(Errcode::ContainerError(0));       
    }

    if host.machine() != "x86_64" {
        return Err(Errcode::NotSupported(1));
    }
    Ok(())
}

pub fn start(args: Args) -> Result<(),Errcode> {
    check_linux_version()?;
    let mut container = Container::new(args)?;

    if let Err(e) = container.create() {
       container.clean_exit()?;
       log::error!("Error while creating container: {:?}",e);
       return Err(e);
    }
    log::debug!("Container child PID: {:?}",container.child_pid);
    wait_child(container.child_pid)?;
    log::debug!("Finished, cleaning & exit");
    container.clean_exit()
}

pub fn wait_child(pid: Option<Pid>) -> Result<(),Errcode> {
    if let Some(child_pid) = pid {
        log::debug!("Waiting for child (pid {}) to finish",child_pid);
        if let Err(e) = waitpid(child_pid, None) {
           log::error!("Error while waiting for pid to finish: {:?}",e);
           return Err(Errcode::ContainerError(1));
        }
    }

    Ok(())
}

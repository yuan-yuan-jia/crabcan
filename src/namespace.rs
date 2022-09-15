use std::os::unix::io::RawFd;
use std::fs::File;
use std::io::Write;
use nix::sched::{unshare,CloneFlags};
use nix::unistd::{Pid,Gid,Uid};
use nix::unistd::{setgroups,setresuid,setresgid};
use crate::errors::Errcode;
use crate::ipc::{send_boolean,recv_boolean};


pub fn userns(fd: RawFd,uid: u32) -> Result<(),Errcode> {
    log::debug!("Setting up user namespace with UID {}",uid);
    let has_userns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };

    send_boolean(fd, has_userns)?;

    if recv_boolean(fd)? {
       return Err(Errcode::NamespacesError(0)); 
    }
    if has_userns {
        log::info!("User namespaces set up");
    }else {
        log::info!("User namespaces not supported,continuing...");
    }

    log::debug!("Switching to uid {} / gid {}...", uid, uid);
    let gid = Gid::from_raw(uid);
    let uid = Uid::from_raw(uid);

    if let Err(_) = setgroups(&[gid]){
        return Err(Errcode::NamespacesError(1));
    }

    if let Err(_) = setresgid(gid, gid, gid){
        return Err(Errcode::NamespacesError(2));
    }

    if let Err(_) = setresuid(uid, uid, uid){
        return Err(Errcode::NamespacesError(3));
    }
    Ok(())
}

const USERNS_OFFSET: u64 = 10000;
const USERNS_COUNT:u64 = 2000;

pub fn handle_child_uid_map(pid: Pid,fd: RawFd) -> Result<(),Errcode> {
    if recv_boolean(fd) ? {
        // Perform UID/ GID map here
        if let Ok(mut uid_map) = File::create(format!("/proc/{}/{}", pid.as_raw(), "uid_map")) {
            if let Err(_) = uid_map.write_all(format!("0 {} {}", USERNS_OFFSET, USERNS_COUNT).as_bytes()) {
                return Err(Errcode::NamespacesError(4));
            }
        } else {
            return Err(Errcode::NamespacesError(5));
        }

        if let Ok(mut gid_map) = File::create(format!("/proc/{}/{}", pid.as_raw(), "gid_map")) {
            if let Err(_) = gid_map.write_all(format!("0 {} {}", USERNS_OFFSET, USERNS_COUNT).as_bytes()) {
                return Err(Errcode::NamespacesError(6));
            }
        } else {
            return Err(Errcode::NamespacesError(7));
        }

    } else {
        log::info!("No user namespaces set up from child process");
    }

    log::debug!("Child UID/GID map done,sending signal to child to continue...");
    send_boolean(fd, false)
}

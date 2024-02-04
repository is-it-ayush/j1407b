mod error;

use error::DaemonError;
use nix::{
    sched::{clone, CloneFlags},
    sys::wait::{waitpid, WaitPidFlag},
    unistd::Pid,
};
use serde::{Deserialize, Serialize};
use shared::config::ConfigHolder;

const CONFIG_FILE_NAME: &str = "daemon";
#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    number: i32,
}

const STACK_SIZE: usize = 2 * 1024 * 1024; // 2 MB

fn main() -> Result<(), DaemonError> {
    let config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;

    let pid: Pid;
    let mut stack = vec![0; STACK_SIZE];
    let child_stack = stack.as_mut_slice();
    let flags = CloneFlags::CLONE_NEWUTS
        | CloneFlags::CLONE_NEWPID
        | CloneFlags::CLONE_NEWNS
        | CloneFlags::CLONE_NEWNET
        | CloneFlags::CLONE_NEWIPC
        | CloneFlags::CLONE_NEWUSER
        | CloneFlags::CLONE_NEWCGROUP;
    unsafe {
        pid = clone(Box::new(|| child()), child_stack, flags, None)
            .map_err(|err| DaemonError::CloneError { errno: err })?
    }
    let exit = waitpid(Some(pid), Some(WaitPidFlag::__WCLONE))
        .map_err(|err| DaemonError::WaitError { errno: err })?;
    println!("Child exited with {:?}", exit);
    Ok(())
}

fn child() -> isize {
    0
}

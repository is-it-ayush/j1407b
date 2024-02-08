mod config;
mod daemon;
mod error;

use daemon::Daemon;
use error::DaemonError;

fn main() -> Result<(), DaemonError> {
    let daemon = Daemon::new()?;
    match daemon.run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err.to_string());
        }
    }
    Ok(())
}

// const STACK_SIZE: usize = 2 * 1024 * 1024; // 2 MB
// fn create_process() -> Result<(), DaemonError> {
//     let pid: Pid;
//     let mut stack = vec![0; STACK_SIZE];
//     let child_stack = stack.as_mut_slice();
//     let flags = CloneFlags::CLONE_NEWUTS
//         | CloneFlags::CLONE_NEWPID
//         | CloneFlags::CLONE_NEWNS
//         | CloneFlags::CLONE_NEWNET
//         | CloneFlags::CLONE_NEWIPC
//         | CloneFlags::CLONE_NEWUSER
//         | CloneFlags::CLONE_NEWCGROUP;
//     unsafe {
//         pid = clone(Box::new(child), child_stack, flags, None)
//             .map_err(|err| DaemonError::CloneSyscall { errno: err })?
//     }
//     let exit = waitpid(Some(pid), Some(WaitPidFlag::__WCLONE))
//         .map_err(|err| DaemonError::WaitSyscall { errno: err })?;
//     println!("Child exited with {:?}", exit);
//     Ok(())
// }
//
// fn child() -> isize {
//     0
// }

use nix::{
    sched::{clone, CloneFlags},
    sys::wait::{waitpid, WaitPidFlag},
    unistd::Pid,
};

const STACK_SIZE: usize = 2 * 1024 * 1024; // 2 MB

fn main() {
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
            .expect("Failed to create new process");
    }
    let exit =
        waitpid(Some(pid), Some(WaitPidFlag::__WCLONE)).expect("Failed to wait for child process");
}

fn child() -> isize {
    0
}

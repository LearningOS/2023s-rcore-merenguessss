//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};
use crate::syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TASK_INFO, SYSCALL_YIELD};
use crate::task::TASK_MANAGER;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    TASK_MANAGER.incr_syscall_time(SYSCALL_EXIT);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    TASK_MANAGER.incr_syscall_time(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    TASK_MANAGER.incr_syscall_time(SYSCALL_GET_TIME);
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(t: *mut TaskInfo) -> isize {
    TASK_MANAGER.incr_syscall_time(SYSCALL_TASK_INFO);
    unsafe {
        *t = TaskInfo{
            status:TASK_MANAGER.get_task_status(),
            syscall_times:TASK_MANAGER.get_syscall_times(),
            time:TASK_MANAGER.get_time(),
        };
    }
    0
}
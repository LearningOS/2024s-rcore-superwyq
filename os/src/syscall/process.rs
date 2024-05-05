//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM, mm::{translated_byte_buffer,VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_times, mmap, suspend_current_and_run_next, task_start_time, TaskStatus
    },
    timer::{get_time_ms, get_time_us}
};

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
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
/// 先将TimeVal的指针转换为物理地址，然后获取时间，最后将时间写入TimeVal中
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let _us = get_time_us();
    let _ts_phy_ptr = translated_byte_buffer(current_user_token(), _ts as *const u8, core::mem::size_of::<TimeVal>());
    let _ts = unsafe { &mut *(_ts_phy_ptr[0].as_ptr() as *mut TimeVal) };
    _ts.sec = _us / 1_000_000;
    _ts.usec = _us % 1_000_000;
    return 0;
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info has implement");
    let _ti_phy_ptr = translated_byte_buffer(current_user_token(), _ti as *const u8, core::mem::size_of::<TaskInfo>());
    let _ti = unsafe { &mut *(_ti_phy_ptr[0].as_ptr() as *mut TaskInfo) };
    _ti.status = TaskStatus::Running;
    _ti.syscall_times = get_syscall_times();
    _ti.time = get_time_ms() - task_start_time();
    return 0;
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port & !0x7 !=0 || _port & 0b0000_0111 == 0 {
        debug!("kernel: sys_mmap port error");
        return -1;
    }
    if !VirtAddr(_start).aligned() || !VirtAddr(_len).aligned() {
        debug!("kernel: sys_mmap start or len not page aligned");
        return -1;
    }
    if _len == 0 {
        debug!("kernel: sys_mmap len is zero");
        return -1;
    }
    mmap(VirtAddr(_start), _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

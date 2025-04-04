//! Types related to task management

use super::TaskContext;
use crate::syscall::*;
use hashbrown::HashMap;
/// The task control block (TCB) of a task.
#[derive(Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The number of task call the sys
    pub task_syscall_count: HashMap<usize, usize>,
}

impl TaskControlBlock {
    /// 记录某种系统调用的次数
    pub fn set_task_syscall_count(&mut self, syscall_id: usize) {
        match syscall_id {
            SYSCALL_WRITE => {
                self.task_syscall_count.insert(SYSCALL_WRITE, self.task_syscall_count.get(&SYSCALL_WRITE).unwrap_or(&0) + 1);
                return;
            }
            SYSCALL_EXIT =>{
                self.task_syscall_count.insert(SYSCALL_EXIT, self.task_syscall_count.get(&SYSCALL_EXIT).unwrap_or(&0) + 1);
                return;
            }
            SYSCALL_YIELD => {
                self.task_syscall_count.insert(SYSCALL_YIELD, self.task_syscall_count.get(&SYSCALL_YIELD).unwrap_or(&0) + 1);
                return;
            }
            SYSCALL_GET_TIME => {
                self.task_syscall_count.insert(SYSCALL_GET_TIME, self.task_syscall_count.get(&SYSCALL_GET_TIME).unwrap_or(&0) + 1);
                return;
            }
            SYSCALL_TRACE => {
                self.task_syscall_count.insert(SYSCALL_TRACE, self.task_syscall_count.get(&SYSCALL_TRACE).unwrap_or(&0) + 1);
                return;
            },
            _ => {

            },
        }
    }
    ///获取某种系统调用的次数
    pub fn get_task_syscall_count(&self,syscall_id: usize) -> usize {
        match syscall_id {
            SYSCALL_WRITE => {
                return *self.task_syscall_count.get(&SYSCALL_WRITE).unwrap_or(&0);
            }
            SYSCALL_EXIT =>{
                return *self.task_syscall_count.get(&SYSCALL_EXIT).unwrap_or(&0);

            }
            SYSCALL_YIELD => {
                return *self.task_syscall_count.get(&SYSCALL_YIELD).unwrap_or(&0);

            }
            SYSCALL_GET_TIME => {
                return *self.task_syscall_count.get(&SYSCALL_GET_TIME).unwrap_or(&0);

            }
            SYSCALL_TRACE => {
                return *self.task_syscall_count.get(&SYSCALL_TRACE).unwrap();

            },
            _ => {

            },
        }
        return 0;
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

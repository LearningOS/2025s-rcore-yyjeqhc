//! Types related to task management

use super::TaskContext;
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
        self.task_syscall_count.insert(syscall_id, self.task_syscall_count.get(&syscall_id).unwrap_or(&0) + 1);

    }
    ///获取某种系统调用的次数
    pub fn get_task_syscall_count(&self,syscall_id: usize) -> usize {
        return *self.task_syscall_count.get(&syscall_id).unwrap_or(&0);
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

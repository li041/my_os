mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;


use crate::println;
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::config;
use lazy_static::lazy_static;
use crate::loader;
use task::{TaskControlBlock, TaskStatus};
use context::TaskContext;

use self::switch::__switch;

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; config::MAX_APP_NUM],
    current_task: usize,
}

/* 变量与常量分离，变量放在具有interior mutability的UPSafeCell中 */
pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

lazy_static! {
     pub static ref TASK_MANAGER: TaskManager = {
        let num_app = loader::get_num_app();
        let mut tasks = [
            TaskControlBlock {
                task_cx: TaskContext::zero_init(),
                task_status: TaskStatus::UnInit
            };
            config::MAX_APP_NUM
        ];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(loader::init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe { UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })},
        }
    };
}

impl  TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        let mut _unused = TaskContext::zero_init();
        drop(inner);
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(
                &mut _unused as *mut TaskContext, 
                next_task_cx_ptr,
            );
        }
        panic!("Unreachable in run_first_task");
    }
    //return Some(id) of task if find
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current..=current+self.num_app)
            .map(|id| id % self.num_app)
            .find(
                |id| {
                    inner.tasks[*id].task_status == TaskStatus::Ready
                })
    }
    /*
     * 1. get current task context
     * 2. change taskmanager.inner.current_task to next
     * 3. switch task context 
     */
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next; 
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(
                    current_task_cx_ptr,
                    next_task_cx_ptr
                );
            }
            // go back to user mode(ret at the end of __switch)
        } else {
            println!("All application completed!");
            shutdown(false);
        }
    }
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    } 
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/* 修改当前task status，然后run下一个 */
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
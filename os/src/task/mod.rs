mod context;
mod switch;
mod task;

use crate::config::*;
use crate::loader::*;
use crate::safe_cell::SafeCell;
use crate::trap::*;
use context::*;
use lazy_static::*;
use task::*;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        println!("[kernel] push_context Done!");
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}


pub struct TaskManager {
    num_app: usize,
    inner: SafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}


// TODO: lazy_static用法
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                SafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

pub fn init() {
    println!("[kernel] print_app_info Done!");
    let app_manager = APP_MANAGER.exclusive_access();
    app_manager.print_app_info();
    drop(app_manager);
}

pub fn run_app() -> ! {
    println!("run_app");
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    app_manager.move_to_next_app();
    // println!("{:#?}", app_manager);
    drop(app_manager);

    // if app_manager.get_current_app() >= app_manager.get_num_app() {
    //     println!("[kernel] All tasks are completed!");
    //     sys_exit(0);
    // }
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS + current_app * APP_SIZE_LIMIT,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in task::run_app!");
}
